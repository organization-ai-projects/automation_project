const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

const DIAGNOSTIC_SOURCE = 'repo-contract-enforcer';
const STATUS_PREFIX = 'Repo Contract Enforcer';
const DEFAULT_RULES_DOC_PATH = 'projects/products/unstable/repo_contract_enforcer/README.md';
const DEFAULT_STRUCTURE_DOC_PATH = 'documentation/technical_documentation/metadata.md';
const DEBOUNCE_MIN_MS = 50;
const DEBOUNCE_MAX_MS = 5000;
const DEFAULT_DEBOUNCE_MS = 300;
const RUN_TIMEOUT_MIN_MS = 1000;
const RUN_TIMEOUT_MAX_MS = 120000;
const DEFAULT_RUN_TIMEOUT_MS = 15000;
const RUN_HISTORY_MIN_SIZE = 5;
const RUN_HISTORY_MAX_SIZE = 200;
const DEFAULT_RUN_HISTORY_SIZE = 30;
const DEFAULT_ARGS = ['run', '-q', '-p', 'repo_contract_enforcer_backend', '--', 'serve'];
const DEFAULT_EXCLUDE_GLOBS = ['**/.git/**', '**/target/**', '**/node_modules/**'];

function toSeverity(value) {
  return value === 'ERROR' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning;
}

function toUri(workspaceRoot, rawPath) {
  if (!rawPath || typeof rawPath !== 'string') {
    return undefined;
  }
  if (path.isAbsolute(rawPath)) {
    return vscode.Uri.file(rawPath);
  }
  return vscode.Uri.file(path.join(workspaceRoot, rawPath));
}

function normalizePath(text) {
  return String(text || '').replace(/\\/g, '/');
}

function escapeRegExp(text) {
  return text.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function globToRegExp(glob) {
  const normalized = normalizePath(glob);
  let regexTxt = escapeRegExp(normalized)
    .replace(/\\\*\\\*/g, '.*')
    .replace(/\\\*/g, '[^/]*')
    .replace(/\\\?/g, '.');
  if (!regexTxt.startsWith('^')) {
    regexTxt = `^${regexTxt}`;
  }
  if (!regexTxt.endsWith('$')) {
    regexTxt = `${regexTxt}$`;
  }
  return new RegExp(regexTxt);
}

function hasWildcard(glob) {
  return glob.includes('*') || glob.includes('?');
}

function normalizeExcludeGlob(glob) {
  const trimmed = normalizePath(String(glob || '').trim()).replace(/^\/+|\/+$/g, '');
  if (!trimmed) {
    return '';
  }
  if (hasWildcard(trimmed)) {
    return trimmed;
  }
  // For plain folder/file tokens (e.g. "target"), match both:
  // - the token itself
  // - content under that token
  return `**/${trimmed}/**`;
}

function getRelativePath(workspaceRoot, rawPath) {
  if (!rawPath || typeof rawPath !== 'string') {
    return '';
  }
  const abs = path.isAbsolute(rawPath) ? rawPath : path.join(workspaceRoot, rawPath);
  const rel = path.relative(workspaceRoot, abs);
  return normalizePath(rel);
}

function isExcludedPath(workspaceRoot, rawPath, excludeRegexes) {
  const rel = getRelativePath(workspaceRoot, rawPath);
  if (!rel || rel.startsWith('..')) {
    return false;
  }
  return excludeRegexes.some((regex) => regex.test(rel));
}

function buildDiagnostic(violation) {
  const line = Math.max((violation.line || 1) - 1, 0);
  const hasColumn = Number.isInteger(violation.col) && violation.col > 0;
  const hasEndColumn = Number.isInteger(violation.end_col) && violation.end_col > 0;
  const startCol = hasColumn ? Math.max(violation.col - 1, 0) : 0;
  let endCol = Number.MAX_SAFE_INTEGER;
  if (hasEndColumn) {
    endCol = Math.max(violation.end_col - 1, startCol + 1);
  } else if (hasColumn) {
    endCol = startCol + 1;
  }

  const range = new vscode.Range(line, startCol, line, endCol);
  const msg = `${violation.message} [${violation.rule_id}/${violation.violation_code}]`;
  const diag = new vscode.Diagnostic(range, msg, toSeverity(violation.severity));
  diag.source = DIAGNOSTIC_SOURCE;
  diag.code = violation.violation_code || undefined;
  diag._repoContractMeta = {
    ruleId: violation.rule_id || '',
    violationCode: violation.violation_code || '',
    path: violation.path || '',
  };
  return diag;
}

function parseRuleMeta(diag) {
  if (!diag || diag.source !== DIAGNOSTIC_SOURCE) {
    return undefined;
  }
  if (diag._repoContractMeta && diag._repoContractMeta.ruleId) {
    return diag._repoContractMeta;
  }
  const txt = String(diag.message || '');
  const match = txt.match(/\[([^/\]]+)\/([^\]]+)\]/);
  if (!match) {
    return undefined;
  }
  return {
    ruleId: match[1],
    violationCode: match[2],
    path: '',
  };
}

function isRelevantDocument(doc) {
  if (!doc || typeof doc.fileName !== 'string') {
    return false;
  }
  return doc.languageId === 'rust' || doc.fileName.endsWith('.toml');
}

function clampDebounceMs(value) {
  const num = Number(value);
  if (!Number.isFinite(num)) {
    return DEFAULT_DEBOUNCE_MS;
  }
  return Math.min(Math.max(num, DEBOUNCE_MIN_MS), DEBOUNCE_MAX_MS);
}

function clampRunTimeoutMs(value) {
  const num = Number(value);
  if (!Number.isFinite(num)) {
    return DEFAULT_RUN_TIMEOUT_MS;
  }
  return Math.min(Math.max(num, RUN_TIMEOUT_MIN_MS), RUN_TIMEOUT_MAX_MS);
}

function clampRunHistorySize(value) {
  const num = Number(value);
  if (!Number.isFinite(num)) {
    return DEFAULT_RUN_HISTORY_SIZE;
  }
  return Math.min(Math.max(Math.floor(num), RUN_HISTORY_MIN_SIZE), RUN_HISTORY_MAX_SIZE);
}

function parseReport(stdout) {
  const lines = stdout
    .split(/\r?\n/)
    .map((s) => s.trim())
    .filter((s) => s.startsWith('{') && s.endsWith('}'));

  const parsed = [];
  for (const line of lines) {
    try {
      parsed.push(JSON.parse(line));
    } catch (_err) {
      // Ignore malformed lines and continue scanning for a valid report line.
    }
  }

  let fallbackReport = undefined;
  for (let i = parsed.length - 1; i >= 0; i -= 1) {
    const obj = parsed[i];
    if (obj && obj.type === 'report' && obj.report_json && typeof obj.report_json === 'object') {
      return obj.report_json;
    }
    if (obj && Array.isArray(obj.violations) && !fallbackReport) {
      fallbackReport = obj;
    }
  }

  return fallbackReport;
}

function getConfigValue(key, defaultValue) {
  const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
  return cfg.get(key, defaultValue);
}

function isEnabled(key, defaultValue = true) {
  return Boolean(getConfigValue(key, defaultValue));
}

function getCommandArgs() {
  const configured = getConfigValue('args', []);
  if (!Array.isArray(configured)) {
    return DEFAULT_ARGS;
  }
  const args = configured.map((value) => String(value || '').trim()).filter((value) => value.length > 0);
  if (args.length === 0) {
    return DEFAULT_ARGS;
  }
  return args;
}

function getProcessMode() {
  const raw = String(getConfigValue('processMode', 'spawn')).toLowerCase();
  return raw === 'persistent' ? 'persistent' : 'spawn';
}

function getExcludeRegexes() {
  const configured = getConfigValue('excludeGlobs', DEFAULT_EXCLUDE_GLOBS);
  const globs = Array.isArray(configured) ? configured : DEFAULT_EXCLUDE_GLOBS;
  return globs
    .map((glob) => normalizeExcludeGlob(glob))
    .filter((glob) => glob.length > 0)
    .map((glob) => globToRegExp(glob));
}

function logOutput(runtime, message) {
  if (!runtime.outputChannel) {
    return;
  }
  if (!getConfigValue('outputChannelEnabled', false)) {
    return;
  }
  runtime.outputChannel.appendLine(message);
}

function addRunHistory(runtime, entry) {
  const maxSize = clampRunHistorySize(getConfigValue('runHistorySize', DEFAULT_RUN_HISTORY_SIZE));
  runtime.runHistory.push(entry);
  if (runtime.runHistory.length > maxSize) {
    runtime.runHistory.splice(0, runtime.runHistory.length - maxSize);
  }
}

function shouldShowTransient(level) {
  const policy = String(getConfigValue('statusBarMessagePolicy', 'errors'));
  if (policy === 'none') {
    return false;
  }
  if (policy === 'errors') {
    return level === 'error';
  }
  return true;
}

function showTransientMessage(message, timeoutMs, level) {
  if (!shouldShowTransient(level)) {
    return;
  }
  vscode.window.setStatusBarMessage(message, timeoutMs);
}

function killChildProcess(child) {
  if (!child || child.killed) {
    return;
  }
  try {
    child.kill();
  } catch (_err) {
    // Ignore process-kill race conditions.
  }
}

function killActiveChildren(runtime) {
  for (const child of runtime.activeChildren) {
    killChildProcess(child);
  }
}

function disposeRuntime(runtime) {
  if (runtime.debounceTimer) {
    clearTimeout(runtime.debounceTimer);
    runtime.debounceTimer = undefined;
  }
  if (runtime.treeDataEmitter) {
    runtime.treeDataEmitter.dispose();
    runtime.treeDataEmitter = undefined;
  }
  killActiveChildren(runtime);
  runtime.persistentSessions.clear();
}

function updateStatus(runtime, text) {
  runtime.statusBar.text = `$(shield) ${STATUS_PREFIX}: ${text}`;
  runtime.statusBar.show();
}

function clearDiagnostics(runtime) {
  runtime.collection.clear();
  setLastCount(0);
  refreshDiagnosticsTree(runtime);
}

function setLastCount(value) {
  vscode.commands.executeCommand('setContext', 'repoContractEnforcer.lastCount', value);
}

function refreshDiagnosticsTree(runtime) {
  if (runtime.treeDataEmitter) {
    runtime.treeDataEmitter.fire(undefined);
  }
}

function shouldTriggerForDocument(configKey, doc) {
  return isEnabled(configKey, true) && isRelevantDocument(doc);
}

function resolveWorkspaceFolders(strategy) {
  const folders = vscode.workspace.workspaceFolders || [];
  if (folders.length === 0) {
    return [];
  }
  if (strategy === 'all') {
    return folders;
  }
  if (strategy === 'active') {
    const activeUri = vscode.window.activeTextEditor?.document?.uri;
    const activeFolder = activeUri ? vscode.workspace.getWorkspaceFolder(activeUri) : undefined;
    if (activeFolder) {
      return [activeFolder];
    }
  }
  return [folders[0]];
}

function extractReportPayload(obj) {
  if (!obj || typeof obj !== 'object') {
    return undefined;
  }
  if (obj.type === 'report' && obj.report_json && typeof obj.report_json === 'object') {
    return obj.report_json;
  }
  if (Array.isArray(obj.violations)) {
    return obj;
  }
  return undefined;
}

function makeCheckResult({
  folder,
  runId,
  code = null,
  signal = null,
  timedOut = false,
  stdout = '',
  stderr = '',
  error = null,
  report = undefined,
}) {
  return {
    folder,
    runId,
    code,
    signal,
    timedOut,
    stdout,
    stderr,
    error,
    report,
  };
}

function removePersistentSession(runtime, folderPath) {
  runtime.persistentSessions.delete(folderPath);
}

function setupPersistentSession(runtime, folder, cmd, args) {
  const folderPath = folder.uri.fsPath;
  const child = cp.spawn(cmd, args, { cwd: folderPath, stdio: ['pipe', 'pipe', 'pipe'] });
  const session = {
    folder,
    child,
    buffer: '',
    lastStderr: '',
    pending: new Map(),
  };

  runtime.persistentSessions.set(folderPath, session);
  runtime.activeChildren.add(child);
  logOutput(runtime, `[persistent] start @ ${folder.name}`);
  logOutput(runtime, `[persistent] cmd: ${cmd} ${args.join(' ')}`);

  const flushPendingWithError = (message) => {
    for (const [requestId, entry] of session.pending.entries()) {
      clearTimeout(entry.timeoutHandle);
      entry.resolve(
        makeCheckResult({
          folder: entry.folder,
          runId: entry.runId,
          error: message,
          stderr: session.lastStderr,
        }),
      );
      session.pending.delete(requestId);
    }
  };

  const resolvePendingWithPayload = (requestId, payload, rawLine) => {
    const entry = session.pending.get(requestId);
    if (!entry) {
      return false;
    }
    clearTimeout(entry.timeoutHandle);
    session.pending.delete(requestId);
    entry.resolve(
      makeCheckResult({
        folder: entry.folder,
        runId: entry.runId,
        code: 0,
        signal: null,
        timedOut: false,
        stdout: rawLine,
        stderr: session.lastStderr,
        report: payload,
      }),
    );
    return true;
  };

  child.stdout.on('data', (chunk) => {
    const txt = chunk.toString();
    session.buffer += txt;
    logOutput(runtime, `[persistent] stdout(${folder.name}): ${txt.trimEnd()}`);

    while (true) {
      const nl = session.buffer.indexOf('\n');
      if (nl < 0) {
        break;
      }
      const line = session.buffer.slice(0, nl).trim();
      session.buffer = session.buffer.slice(nl + 1);
      if (!line) {
        continue;
      }

      let obj;
      try {
        obj = JSON.parse(line);
      } catch (_err) {
        continue;
      }

      const payload = extractReportPayload(obj);
      if (!payload || session.pending.size === 0) {
        continue;
      }

      const responseId = typeof obj.id === 'string' ? obj.id : '';
      if (responseId && resolvePendingWithPayload(responseId, payload, line)) {
        continue;
      }

      // Fallback when backend payload has no id: resolve oldest pending request.
      const firstPendingId = session.pending.keys().next().value;
      if (firstPendingId) {
        resolvePendingWithPayload(firstPendingId, payload, line);
      }
    }
  });

  child.stderr.on('data', (chunk) => {
    const txt = chunk.toString();
    session.lastStderr += txt;
    if (session.lastStderr.length > 8000) {
      session.lastStderr = session.lastStderr.slice(-8000);
    }
    logOutput(runtime, `[persistent] stderr(${folder.name}): ${txt.trimEnd()}`);
  });

  child.on('error', (err) => {
    runtime.activeChildren.delete(child);
    removePersistentSession(runtime, folderPath);
    flushPendingWithError(`persistent process error (${String(err?.message || 'unknown error')})`);
  });

  child.on('close', (code, signal) => {
    runtime.activeChildren.delete(child);
    removePersistentSession(runtime, folderPath);
    flushPendingWithError(`persistent process closed (code=${String(code)}, signal=${String(signal)})`);
  });

  return session;
}

function getPersistentSession(runtime, folder, cmd, args) {
  const folderPath = folder.uri.fsPath;
  const existing = runtime.persistentSessions.get(folderPath);
  if (existing && existing.child && !existing.child.killed) {
    return existing;
  }
  removePersistentSession(runtime, folderPath);
  return setupPersistentSession(runtime, folder, cmd, args);
}

function executeCheckForFolderPersistent(runtime, params) {
  const { folder, runId, mode, cmd, args, timeoutMs } = params;
  return new Promise((resolve) => {
    const session = getPersistentSession(runtime, folder, cmd, args);
    const requestId = `vscode-${runId}-${folder.name}-${Date.now()}-${Math.random().toString(16).slice(2)}`;
    const request = {
      id: requestId,
      type: 'checkRepo',
      root_path: folder.uri.fsPath,
      mode,
    };

    const timeoutHandle = setTimeout(() => {
      session.pending.delete(requestId);
      killChildProcess(session.child);
      resolve(
        makeCheckResult({
          folder,
          runId,
          timedOut: true,
          error: `timeout after ${timeoutMs}ms`,
          stderr: session.lastStderr,
        }),
      );
    }, timeoutMs);

    session.pending.set(requestId, {
      folder,
      runId,
      timeoutHandle,
      resolve,
    });

    try {
      session.child.stdin.write(JSON.stringify(request) + '\n');
    } catch (err) {
      clearTimeout(timeoutHandle);
      session.pending.delete(requestId);
      resolve(
        makeCheckResult({
          folder,
          runId,
          error: `request write failed (${String(err?.message || 'unknown error')})`,
          stderr: session.lastStderr,
        }),
      );
    }
  });
}

function executeCheckForFolderSpawn(runtime, params) {
  const { folder, runId, reason, mode, cmd, args, timeoutMs } = params;
  return new Promise((resolve) => {
    const child = cp.spawn(cmd, args, { cwd: folder.uri.fsPath, stdio: ['pipe', 'pipe', 'pipe'] });
    runtime.activeChildren.add(child);
    let stdout = '';
    let stderr = '';
    let timedOut = false;
    let settled = false;

    const settle = (result) => {
      if (settled) {
        return;
      }
      settled = true;
      runtime.activeChildren.delete(child);
      resolve(result);
    };

    const timeoutHandle = setTimeout(() => {
      if (runId !== runtime.runSeq) {
        return;
      }
      if (child && !child.killed) {
        timedOut = true;
        killChildProcess(child);
      }
    }, timeoutMs);

    const clearTimeoutHandle = () => clearTimeout(timeoutHandle);

    logOutput(runtime, `[run ${runId}] start (${reason}) @ ${folder.name}`);
    logOutput(runtime, `[run ${runId}] cmd: ${cmd} ${args.join(' ')}`);

    child.stdout.on('data', (chunk) => {
      const txt = chunk.toString();
      stdout += txt;
      logOutput(runtime, `[run ${runId}] stdout(${folder.name}): ${txt.trimEnd()}`);
    });
    child.stderr.on('data', (chunk) => {
      const txt = chunk.toString();
      stderr += txt;
      logOutput(runtime, `[run ${runId}] stderr(${folder.name}): ${txt.trimEnd()}`);
    });
    child.on('error', (err) => {
      clearTimeoutHandle();
      settle(
        makeCheckResult({
          folder,
          runId,
          timedOut,
          stdout,
          stderr,
          error: String(err?.message || 'unknown error'),
        }),
      );
    });
    child.on('close', (code, signal) => {
      clearTimeoutHandle();
      settle(
        makeCheckResult({
          folder,
          runId,
          code,
          signal,
          timedOut,
          stdout,
          stderr,
          error: null,
        }),
      );
    });

    const request = {
      id: `vscode-${runId}-${folder.name}`,
      type: 'checkRepo',
      root_path: folder.uri.fsPath,
      mode,
    };
    const shutdown = { id: `vscode-shutdown-${runId}-${folder.name}`, type: 'shutdown' };
    child.stdin.write(JSON.stringify(request) + '\n');
    child.stdin.write(JSON.stringify(shutdown) + '\n');
    child.stdin.end();
  });
}

function executeCheckForFolder(runtime, params) {
  const processMode = getProcessMode();
  if (processMode === 'persistent') {
    return executeCheckForFolderPersistent(runtime, params);
  }
  return executeCheckForFolderSpawn(runtime, params);
}

function mergeDiagnosticsByFile(byFile, folder, violations, excludeRegexes) {
  for (const violation of violations) {
    if (isExcludedPath(folder.uri.fsPath, violation.path, excludeRegexes)) {
      continue;
    }
    const uri = toUri(folder.uri.fsPath, violation.path);
    if (!uri) {
      continue;
    }
    const key = uri.toString();
    const list = byFile.get(key) || { uri, diagnostics: [] };
    list.diagnostics.push(buildDiagnostic(violation));
    byFile.set(key, list);
  }
}

function pushCodeAction(actions, keySet, key, action) {
  if (keySet.has(key)) {
    return;
  }
  keySet.add(key);
  actions.push(action);
}

function buildCommandCodeAction(title, kind, command, commandTitle, args = []) {
  const action = new vscode.CodeAction(title, kind);
  action.command = {
    command,
    title: commandTitle,
    arguments: args,
  };
  return action;
}

function buildDiagnosticHover(meta) {
  if (!meta || !meta.ruleId) {
    return undefined;
  }

  const md = new vscode.MarkdownString();
  md.appendMarkdown(`**Repo Contract Enforcer**\n\n`);
  md.appendMarkdown(`- Rule: \`${meta.ruleId}\`\n`);
  if (meta.violationCode) {
    md.appendMarkdown(`- Code: \`${meta.violationCode}\`\n`);
  }
  if (meta.path) {
    md.appendMarkdown(`- Path: \`${meta.path}\`\n`);
  }
  md.appendMarkdown('\nUse the quick fix to open rule docs.');
  md.isTrusted = false;
  return new vscode.Hover(md);
}

function severityLabelFromDiagnostic(diag) {
  if (!diag) {
    return 'unknown';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Error) {
    return 'error';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Warning) {
    return 'warning';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Information) {
    return 'info';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Hint) {
    return 'hint';
  }
  return 'unknown';
}

function severityIconFromDiagnostic(diag) {
  if (!diag) {
    return 'info';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Error) {
    return 'error';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Warning) {
    return 'warning';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Information) {
    return 'info';
  }
  if (diag.severity === vscode.DiagnosticSeverity.Hint) {
    return 'lightbulb';
  }
  return 'info';
}

function getTreeGroupingMode() {
  const raw = String(getConfigValue('treeViewGrouping', 'file')).toLowerCase();
  return raw === 'rule' ? 'rule' : 'file';
}

function getTreeSeverityFilter() {
  const raw = String(getConfigValue('treeViewSeverityFilter', 'all')).toLowerCase();
  return raw === 'errors' ? 'errors' : 'all';
}

function includeDiagnosticInTree(diag) {
  const filter = getTreeSeverityFilter();
  if (filter === 'errors') {
    return diag && diag.severity === vscode.DiagnosticSeverity.Error;
  }
  return true;
}

function diagnosticsTreeFileItem(uri, diagnostics) {
  const errors = diagnostics.filter((d) => d.severity === vscode.DiagnosticSeverity.Error).length;
  const warnings = diagnostics.filter((d) => d.severity === vscode.DiagnosticSeverity.Warning).length;
  const label = vscode.workspace.asRelativePath(uri, false);
  const fileItem = new vscode.TreeItem(label, vscode.TreeItemCollapsibleState.Collapsed);
  fileItem.contextValue = 'repoContractEnforcer.file';
  fileItem.iconPath = new vscode.ThemeIcon('file');
  fileItem.description = `${diagnostics.length} issues`;
  fileItem.tooltip = `${label}\n${errors} error(s), ${warnings} warning(s)`;
  fileItem.resourceUri = uri;
  return fileItem;
}

function diagnosticsTreeRuleItem(ruleId, entries) {
  const label = ruleId || 'unknown-rule';
  const diagnostics = entries.map((entry) => entry.diag);
  const errors = diagnostics.filter((d) => d.severity === vscode.DiagnosticSeverity.Error).length;
  const warnings = diagnostics.filter((d) => d.severity === vscode.DiagnosticSeverity.Warning).length;
  const item = new vscode.TreeItem(label, vscode.TreeItemCollapsibleState.Collapsed);
  item.contextValue = 'repoContractEnforcer.rule';
  item.iconPath = new vscode.ThemeIcon('symbol-property');
  item.description = `${entries.length} issues`;
  item.tooltip = `${label}\n${errors} error(s), ${warnings} warning(s)`;
  item._repoRuleId = label;
  return item;
}

function diagnosticsTreeViolationItem(uri, diag, options = {}) {
  const { showPathPrefix = false } = options;
  const line = diag.range.start.line + 1;
  const code = diag.code ? String(diag.code) : '';
  const severity = severityLabelFromDiagnostic(diag);
  const icon = severityIconFromDiagnostic(diag);
  const relPath = vscode.workspace.asRelativePath(uri, false);
  const prefix = showPathPrefix ? `${relPath}:${line} ` : '';
  const item = new vscode.TreeItem(`${prefix}L${line}: ${diag.message}`, vscode.TreeItemCollapsibleState.None);
  item.contextValue = 'repoContractEnforcer.violation';
  item.iconPath = new vscode.ThemeIcon(icon);
  item.description = `${severity}${code ? ` • ${code}` : ''}`;
  item.tooltip = `${diag.message}\n${relPath}:${line}`;
  item.command = {
    command: 'vscode.open',
    title: 'Open diagnostic location',
    arguments: [uri, { selection: diag.range, preview: true }],
  };
  return item;
}

function registerDiagnosticsTreeProvider(runtime) {
  const emitter = new vscode.EventEmitter();
  runtime.treeDataEmitter = emitter;

  const collectEntries = () => {
    const entries = [];
    for (const [uri, diagnostics] of runtime.collection) {
      if (!Array.isArray(diagnostics) || diagnostics.length === 0) {
        continue;
      }
      for (const diag of diagnostics) {
        if (!includeDiagnosticInTree(diag)) {
          continue;
        }
        entries.push({ uri, diag, meta: parseRuleMeta(diag) });
      }
    }
    return entries;
  };

  const provider = {
    getTreeItem(element) {
      return element;
    },
    getChildren(element) {
      const grouping = getTreeGroupingMode();
      const entries = collectEntries();

      if (!element) {
        if (grouping === 'rule') {
          const byRule = new Map();
          for (const entry of entries) {
            const ruleId = (entry.meta && entry.meta.ruleId) || 'unknown-rule';
            const list = byRule.get(ruleId) || [];
            list.push(entry);
            byRule.set(ruleId, list);
          }
          const roots = [];
          for (const [ruleId, ruleEntries] of byRule.entries()) {
            roots.push(diagnosticsTreeRuleItem(ruleId, ruleEntries));
          }
          roots.sort((a, b) => String(a.label).localeCompare(String(b.label)));
          return roots;
        }

        const byFile = new Map();
        for (const entry of entries) {
          const key = entry.uri.toString();
          const list = byFile.get(key) || { uri: entry.uri, diagnostics: [] };
          list.diagnostics.push(entry.diag);
          byFile.set(key, list);
        }
        const roots = [];
        for (const { uri, diagnostics } of byFile.values()) {
          roots.push(diagnosticsTreeFileItem(uri, diagnostics));
        }
        roots.sort((a, b) => String(a.label).localeCompare(String(b.label)));
        return roots;
      }

      if (element.contextValue === 'repoContractEnforcer.file' && element.resourceUri) {
        const diagnostics = (runtime.collection.get(element.resourceUri) || []).filter((diag) => includeDiagnosticInTree(diag));
        return diagnostics.map((diag) => diagnosticsTreeViolationItem(element.resourceUri, diag));
      }

      if (element.contextValue === 'repoContractEnforcer.rule' && element._repoRuleId) {
        const ruleId = String(element._repoRuleId);
        const ruleEntries = entries.filter((entry) => {
          const currentRuleId = (entry.meta && entry.meta.ruleId) || 'unknown-rule';
          return currentRuleId === ruleId;
        });
        ruleEntries.sort((a, b) => {
          const fileCmp = vscode.workspace.asRelativePath(a.uri, false).localeCompare(vscode.workspace.asRelativePath(b.uri, false));
          if (fileCmp !== 0) {
            return fileCmp;
          }
          return a.diag.range.start.line - b.diag.range.start.line;
        });
        return ruleEntries.map((entry) => diagnosticsTreeViolationItem(entry.uri, entry.diag, { showPathPrefix: true }));
      }

      return [];
    },
    onDidChangeTreeData: emitter.event,
  };

  return vscode.window.createTreeView('repoContractEnforcerDiagnostics', { treeDataProvider: provider });
}

async function runEnforcer(runtime, reason) {
  const strategy = String(getConfigValue('workspaceFolderStrategy', 'first'));
  const folders = resolveWorkspaceFolders(strategy);
  if (folders.length === 0) {
    return;
  }

  const processMode = getProcessMode();
  const mode = getConfigValue('mode', 'auto');
  const cmd = getConfigValue('command', 'cargo');
  const args = getCommandArgs();
  const timeoutMs = clampRunTimeoutMs(getConfigValue('runTimeoutMs', DEFAULT_RUN_TIMEOUT_MS));
  const excludeRegexes = getExcludeRegexes();
  const runId = ++runtime.runSeq;
  const startedAt = Date.now();

  if (processMode === 'spawn') {
    killActiveChildren(runtime);
  }

  updateStatus(runtime, `running (${reason})...`);

  const byFile = new Map();
  let totalViolations = 0;
  let hadError = false;
  const issues = [];

  for (const folder of folders) {
    if (runId !== runtime.runSeq) {
      return;
    }
    const result = await executeCheckForFolder(runtime, {
      folder,
      runId,
      reason,
      mode,
      cmd,
      args,
      timeoutMs,
    });

    if (runId !== runtime.runSeq) {
      return;
    }

    if (result.error) {
      hadError = true;
      issues.push(`${folder.name}: spawn failed (${result.error})`);
      continue;
    }

    if (result.signal !== null) {
      if (result.timedOut) {
        hadError = true;
        issues.push(`${folder.name}: timeout after ${timeoutMs}ms`);
      } else {
        issues.push(`${folder.name}: interrupted (${result.signal})`);
      }
      continue;
    }

    const parsed = result.report || parseReport(result.stdout);
    if (!parsed) {
      hadError = true;
      const errSuffix =
        result.stderr && result.stderr.trim().length > 0 ? `: ${result.stderr.trim().split(/\r?\n/).slice(-1)[0]}` : '';
      issues.push(`${folder.name}: invalid backend output${errSuffix}`);
      continue;
    }

    const violations = Array.isArray(parsed.violations) ? parsed.violations : [];
    totalViolations += violations.length;
    mergeDiagnosticsByFile(byFile, folder, violations, excludeRegexes);
  }

  runtime.collection.clear();
  for (const { uri, diagnostics } of byFile.values()) {
    runtime.collection.set(uri, diagnostics);
  }
  refreshDiagnosticsTree(runtime);

  setLastCount(totalViolations);

  const durationMs = Date.now() - startedAt;
  const outcomeParts = [`${totalViolations} issue(s)`];
  if (folders.length > 1) {
    outcomeParts.push(`${folders.length} folders`);
  }
  if (hadError) {
    outcomeParts.push('with errors');
  }
  const outcome = outcomeParts.join(', ');
  updateStatus(runtime, outcome);
  logOutput(runtime, `[run ${runId}] completed in ${durationMs}ms: ${outcome}`);
  for (const issue of issues) {
    logOutput(runtime, `[run ${runId}] issue: ${issue}`);
  }

  addRunHistory(runtime, {
    runId,
    at: new Date().toISOString(),
    reason,
    strategy,
    folders: folders.map((f) => f.name),
    durationMs,
    totalViolations,
    hadError,
    issues: [...issues],
  });

  if (hadError) {
    showTransientMessage(`${STATUS_PREFIX}: ${issues[issues.length - 1] || 'check failed'}`, 6000, 'error');
  } else {
    showTransientMessage(`${STATUS_PREFIX}: ${outcome}`, 3000, 'info');
  }
}

function getRuleDocUri(workspaceRoot, ruleId) {
  const defaultDocPath = getConfigValue('docsPathDefault', DEFAULT_RULES_DOC_PATH);
  const structureDocPath = getConfigValue('docsPathStructure', DEFAULT_STRUCTURE_DOC_PATH);
  if (ruleId === 'structure') {
    return vscode.Uri.file(path.join(workspaceRoot, structureDocPath));
  }
  return vscode.Uri.file(path.join(workspaceRoot, defaultDocPath));
}

function findFirstLineContaining(text, needle) {
  if (!needle) {
    return undefined;
  }
  const lines = text.split(/\r?\n/);
  for (let i = 0; i < lines.length; i += 1) {
    if (lines[i].includes(needle)) {
      return i;
    }
  }
  return undefined;
}

function registerCodeActionProvider(runtime) {
  const provider = {
    provideCodeActions(_document, _range, context) {
      const actions = [];
      const keys = new Set();
      for (const diag of context.diagnostics || []) {
        if (diag.source !== DIAGNOSTIC_SOURCE) {
          continue;
        }
        pushCodeAction(
          actions,
          keys,
          'rerun',
          buildCommandCodeAction(
            'Repo Contract Enforcer: rerun check',
            vscode.CodeActionKind.QuickFix,
            'repoContractEnforcer.runCheck',
            'Run check',
          ),
        );

        const meta = parseRuleMeta(diag);
        if (!meta || !meta.ruleId) {
          continue;
        }
        const key = `docs:${meta.ruleId}:${meta.violationCode}`;
        pushCodeAction(
          actions,
          keys,
          key,
          buildCommandCodeAction(
            `Repo Contract Enforcer: open docs (${meta.ruleId}/${meta.violationCode})`,
            vscode.CodeActionKind.QuickFix,
            'repoContractEnforcer.openRuleDocs',
            'Open rule documentation',
            [meta.ruleId, meta.violationCode],
          ),
        );
      }
      return actions;
    },
  };

  return vscode.languages.registerCodeActionsProvider([{ language: 'rust' }, { language: 'toml' }], provider, {
    providedCodeActionKinds: [vscode.CodeActionKind.QuickFix],
  });
}

function registerHoverProvider(runtime) {
  const provider = {
    provideHover(document, position) {
      const diagnostics = runtime.collection.get(document.uri) || [];
      for (const diag of diagnostics) {
        if (!diag.range.contains(position)) {
          continue;
        }
        const meta = parseRuleMeta(diag);
        const hover = buildDiagnosticHover(meta);
        if (hover) {
          return hover;
        }
      }
      return undefined;
    },
  };

  return vscode.languages.registerHoverProvider([{ language: 'rust' }, { language: 'toml' }], provider);
}

function activate(context) {
  const collection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE);
  const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 50);
  const outputChannel = vscode.window.createOutputChannel('Repo Contract Enforcer');
  updateStatus({ statusBar }, 'idle');

  const runtime = {
    collection,
    statusBar,
    outputChannel,
    treeDataEmitter: undefined,
    activeChildren: new Set(),
    persistentSessions: new Map(),
    runSeq: 0,
    debounceTimer: undefined,
    runHistory: [],
  };

  const triggerNow = (reason) => {
    if (runtime.debounceTimer) {
      clearTimeout(runtime.debounceTimer);
      runtime.debounceTimer = undefined;
    }
    runEnforcer(runtime, reason).catch((err) => {
      updateStatus(runtime, 'unexpected extension error');
      showTransientMessage(
        `${STATUS_PREFIX}: unexpected extension error (${String(err?.message || 'unknown')})`,
        6000,
        'error',
      );
      logOutput(runtime, `unexpected extension error: ${String(err?.stack || err?.message || err)}`);
    });
  };

  const triggerDebounced = (reason) => {
    const debounceMs = clampDebounceMs(getConfigValue('debounceMs', DEFAULT_DEBOUNCE_MS));
    if (runtime.debounceTimer) {
      clearTimeout(runtime.debounceTimer);
    }
    runtime.debounceTimer = setTimeout(() => {
      runtime.debounceTimer = undefined;
      triggerNow(reason);
    }, debounceMs);
  };

  const runCmd = vscode.commands.registerCommand('repoContractEnforcer.runCheck', () => triggerNow('manual'));
  const refreshDiagnosticsViewCmd = vscode.commands.registerCommand('repoContractEnforcer.refreshDiagnosticsView', () => {
    refreshDiagnosticsTree(runtime);
  });
  const collapseDiagnosticsViewCmd = vscode.commands.registerCommand(
    'repoContractEnforcer.collapseDiagnosticsView',
    async () => {
      await vscode.commands.executeCommand(
        'workbench.actions.treeView.repoContractEnforcerDiagnostics.collapseAll',
      );
    },
  );
  const clearDiagnosticsCmd = vscode.commands.registerCommand('repoContractEnforcer.clearDiagnostics', () => {
    clearDiagnostics(runtime);
    updateStatus(runtime, 'diagnostics cleared');
    showTransientMessage(`${STATUS_PREFIX}: diagnostics cleared`, 3000, 'info');
  });
  const showOutputCmd = vscode.commands.registerCommand('repoContractEnforcer.showOutput', () => {
    runtime.outputChannel.show(true);
  });
  const showHistoryCmd = vscode.commands.registerCommand('repoContractEnforcer.showRunHistory', () => {
    runtime.outputChannel.show(true);
    runtime.outputChannel.appendLine('----- Run History -----');
    if (runtime.runHistory.length === 0) {
      runtime.outputChannel.appendLine('No runs yet.');
      return;
    }
    for (const entry of runtime.runHistory) {
      runtime.outputChannel.appendLine(
        `[run ${entry.runId}] ${entry.at} | reason=${entry.reason} | strategy=${entry.strategy} | folders=${entry.folders.join(',')} | duration=${entry.durationMs}ms | violations=${entry.totalViolations} | hadError=${entry.hadError}`,
      );
      for (const issue of entry.issues) {
        runtime.outputChannel.appendLine(`  - ${issue}`);
      }
    }
  });
  const openRuleDocsCmd = vscode.commands.registerCommand(
    'repoContractEnforcer.openRuleDocs',
    async (ruleId, violationCode) => {
      const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri?.fsPath;
      if (!workspaceRoot) {
        return;
      }
      const uri = getRuleDocUri(workspaceRoot, String(ruleId || ''));
      try {
        const doc = await vscode.workspace.openTextDocument(uri);
        const editor = await vscode.window.showTextDocument(doc, { preview: true });
        const targetLine =
          findFirstLineContaining(doc.getText(), String(violationCode || '')) ??
          findFirstLineContaining(doc.getText(), String(ruleId || ''));
        if (Number.isInteger(targetLine)) {
          const position = new vscode.Position(targetLine, 0);
          editor.selection = new vscode.Selection(position, position);
          editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
        }
      } catch (_err) {
        showTransientMessage(`${STATUS_PREFIX}: documentation file not found`, 4000, 'error');
      }
    },
  );

  const codeActionProvider = registerCodeActionProvider(runtime);
  const hoverProvider = registerHoverProvider(runtime);
  const diagnosticsTreeView = registerDiagnosticsTreeProvider(runtime);

  const saveSub = vscode.workspace.onDidSaveTextDocument((doc) => {
    if (!shouldTriggerForDocument('runOnSave', doc)) {
      return;
    }
    triggerNow('save');
  });

  const changeSub = vscode.workspace.onDidChangeTextDocument((evt) => {
    if (!shouldTriggerForDocument('runOnChange', evt.document)) {
      return;
    }
    if (!evt.contentChanges || evt.contentChanges.length === 0) {
      return;
    }
    triggerDebounced('edit');
  });

  const watchFs = vscode.workspace.createFileSystemWatcher('**/*.{rs,toml}');
  const fsChanged = () => {
    if (!isEnabled('runOnFileEvents', true)) {
      return;
    }
    triggerDebounced('file event');
  };
  watchFs.onDidChange(fsChanged);
  watchFs.onDidCreate(fsChanged);
  watchFs.onDidDelete(fsChanged);

  const renameSub = vscode.workspace.onDidRenameFiles(() => {
    if (!isEnabled('runOnFileEvents', true)) {
      return;
    }
    triggerDebounced('rename');
  });

  const cfgSub = vscode.workspace.onDidChangeConfiguration((evt) => {
    if (!evt.affectsConfiguration('repoContractEnforcer')) {
      return;
    }
    killActiveChildren(runtime);
    runtime.persistentSessions.clear();
    refreshDiagnosticsTree(runtime);
    triggerNow('config changed');
  });

  context.subscriptions.push(
    collection,
    statusBar,
    outputChannel,
    runCmd,
    refreshDiagnosticsViewCmd,
    collapseDiagnosticsViewCmd,
    clearDiagnosticsCmd,
    showOutputCmd,
    showHistoryCmd,
    openRuleDocsCmd,
    codeActionProvider,
    hoverProvider,
    diagnosticsTreeView,
    saveSub,
    changeSub,
    watchFs,
    renameSub,
    cfgSub,
    { dispose: () => disposeRuntime(runtime) },
  );

  if (isEnabled('runOnStartup', true)) {
    triggerNow('startup');
  }
}

module.exports = {
  activate,
  __test: {
    isRelevantDocument,
    clampDebounceMs,
    clampRunTimeoutMs,
    clampRunHistorySize,
    globToRegExp,
    parseReport,
  },
};
