const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

const DIAGNOSTIC_SOURCE = 'repo-contract-enforcer';
const STATUS_PREFIX = 'Repo Contract Enforcer';
const DEBOUNCE_MIN_MS = 50;
const DEBOUNCE_MAX_MS = 5000;
const DEFAULT_DEBOUNCE_MS = 300;
const RUN_TIMEOUT_MIN_MS = 1000;
const RUN_TIMEOUT_MAX_MS = 120000;
const DEFAULT_RUN_TIMEOUT_MS = 15000;

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
  return diag;
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
    if (obj && Array.isArray(obj.violations)) {
      fallbackReport = obj;
    }
  }

  return fallbackReport;
}

function getConfigValue(key, defaultValue) {
  const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
  return cfg.get(key, defaultValue);
}

function getCommandArgs() {
  const configured = getConfigValue('args', []);
  if (!Array.isArray(configured)) {
    return ['run', '-q', '-p', 'repo_contract_enforcer_backend', '--', 'serve'];
  }
  const args = configured
    .map((value) => String(value || '').trim())
    .filter((value) => value.length > 0);
  if (args.length === 0) {
    return ['run', '-q', '-p', 'repo_contract_enforcer_backend', '--', 'serve'];
  }
  return args;
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

function disposeRuntime(runtime) {
  if (runtime.debounceTimer) {
    clearTimeout(runtime.debounceTimer);
    runtime.debounceTimer = undefined;
  }
  if (runtime.currentChild && !runtime.currentChild.killed) {
    runtime.currentChild.kill();
  }
}

function updateStatus(runtime, text) {
  runtime.statusBar.text = `$(shield) ${STATUS_PREFIX}: ${text}`;
  runtime.statusBar.show();
}

function clearDiagnostics(runtime) {
  runtime.collection.clear();
  vscode.commands.executeCommand('setContext', 'repoContractEnforcer.lastCount', 0);
}

function runEnforcer(runtime, reason) {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    return;
  }

  const mode = getConfigValue('mode', 'auto');
  const cmd = getConfigValue('command', 'cargo');
  const args = getCommandArgs();
  const timeoutMs = clampRunTimeoutMs(getConfigValue('runTimeoutMs', DEFAULT_RUN_TIMEOUT_MS));
  const runId = ++runtime.runSeq;

  if (runtime.currentChild && !runtime.currentChild.killed) {
    runtime.currentChild.kill();
  }

  const child = cp.spawn(cmd, args, { cwd: folder.uri.fsPath, stdio: ['pipe', 'pipe', 'pipe'] });
  runtime.currentChild = child;
  let stdout = '';
  let stderr = '';
  let timedOut = false;
  let timeoutHandle = undefined;

  logOutput(runtime, `[run ${runId}] start (${reason})`);
  logOutput(runtime, `[run ${runId}] cmd: ${cmd} ${args.join(' ')}`);

  updateStatus(runtime, `running (${reason})...`);

  timeoutHandle = setTimeout(() => {
    if (runId !== runtime.runSeq) {
      return;
    }
    if (runtime.currentChild && !runtime.currentChild.killed) {
      timedOut = true;
      runtime.currentChild.kill();
    }
  }, timeoutMs);

  child.stdout.on('data', (chunk) => {
    const txt = chunk.toString();
    stdout += txt;
    if (getConfigValue('outputChannelEnabled', false) && txt.trim().length > 0) {
      runtime.outputChannel.appendLine(`[run ${runId}] stdout: ${txt.trimEnd()}`);
    }
  });
  child.stderr.on('data', (chunk) => {
    const txt = chunk.toString();
    stderr += txt;
    if (getConfigValue('outputChannelEnabled', false) && txt.trim().length > 0) {
      runtime.outputChannel.appendLine(`[run ${runId}] stderr: ${txt.trimEnd()}`);
    }
  });
  child.on('error', (err) => {
    if (timeoutHandle) {
      clearTimeout(timeoutHandle);
      timeoutHandle = undefined;
    }
    if (runId !== runtime.runSeq) {
      return;
    }
    logOutput(runtime, `[run ${runId}] spawn error: ${String(err?.message || 'unknown error')}`);
    clearDiagnostics(runtime);
    updateStatus(runtime, 'backend spawn failed');
    vscode.window.setStatusBarMessage(
      `${STATUS_PREFIX}: backend spawn failed (${String(err?.message || 'unknown error')})`,
      5000,
    );
  });

  child.on('close', (code, signal) => {
    if (timeoutHandle) {
      clearTimeout(timeoutHandle);
      timeoutHandle = undefined;
    }
    if (runId !== runtime.runSeq) {
      return;
    }
    runtime.currentChild = undefined;

    if (signal !== null) {
      if (timedOut) {
        clearDiagnostics(runtime);
        updateStatus(runtime, `timeout after ${timeoutMs}ms`);
        vscode.window.setStatusBarMessage(`${STATUS_PREFIX}: check timed out after ${timeoutMs}ms`, 6000);
        logOutput(runtime, `[run ${runId}] timeout after ${timeoutMs}ms`);
        return;
      }
      logOutput(runtime, `[run ${runId}] interrupted by signal ${signal}`);
      updateStatus(runtime, 'check interrupted');
      return;
    }

    const byFile = new Map();

    const parsed = parseReport(stdout);
    if (!parsed) {
      clearDiagnostics(runtime);
      const errSuffix = stderr && stderr.trim().length > 0 ? `: ${stderr.trim().split(/\r?\n/).slice(-1)[0]}` : '';
      logOutput(runtime, `[run ${runId}] invalid backend output${errSuffix}`);
      updateStatus(runtime, 'invalid backend output');
      vscode.window.setStatusBarMessage(`${STATUS_PREFIX}: invalid backend output${errSuffix}`, 6000);
      return;
    }

    const violations = Array.isArray(parsed.violations) ? parsed.violations : [];
    for (const v of violations) {
      const uri = toUri(folder.uri.fsPath, v.path);
      if (!uri) {
        continue;
      }
      const key = uri.toString();
      const list = byFile.get(key) || { uri, diagnostics: [] };
      list.diagnostics.push(buildDiagnostic(v));
      byFile.set(key, list);
    }

    runtime.collection.clear();
    for (const { uri, diagnostics } of byFile.values()) {
      runtime.collection.set(uri, diagnostics);
    }

    vscode.commands.executeCommand('setContext', 'repoContractEnforcer.lastCount', violations.length);
    const outcome = code === 0 ? `${violations.length} issue(s)` : `${violations.length} issue(s), exit ${code}`;
    logOutput(runtime, `[run ${runId}] completed: ${outcome}`);
    updateStatus(runtime, outcome);
  });

  const request = {
    id: `vscode-${runId}`,
    type: 'checkRepo',
    root_path: folder.uri.fsPath,
    mode,
  };
  const shutdown = { id: 'vscode-shutdown', type: 'shutdown' };
  child.stdin.write(JSON.stringify(request) + '\n');
  child.stdin.write(JSON.stringify(shutdown) + '\n');
  child.stdin.end();
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
    currentChild: undefined,
    runSeq: 0,
    debounceTimer: undefined,
  };

  const triggerNow = (reason) => runEnforcer(runtime, reason);
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

  context.subscriptions.push(collection, statusBar, outputChannel, { dispose: () => disposeRuntime(runtime) });

  const cmd = vscode.commands.registerCommand('repoContractEnforcer.runCheck', () => triggerNow('manual'));
  const showOutputCmd = vscode.commands.registerCommand('repoContractEnforcer.showOutput', () => {
    runtime.outputChannel.show(true);
  });
  context.subscriptions.push(cmd, showOutputCmd);

  const saveSub = vscode.workspace.onDidSaveTextDocument((doc) => {
    if (!getConfigValue('runOnSave', true) || !isRelevantDocument(doc)) {
      return;
    }
    triggerNow('save');
  });

  const changeSub = vscode.workspace.onDidChangeTextDocument((evt) => {
    if (!getConfigValue('runOnChange', true) || !isRelevantDocument(evt.document)) {
      return;
    }
    if (!evt.contentChanges || evt.contentChanges.length === 0) {
      return;
    }
    triggerDebounced('edit');
  });

  const watchFs = vscode.workspace.createFileSystemWatcher('**/*.{rs,toml}');
  const fsChanged = () => {
    if (!getConfigValue('runOnFileEvents', true)) {
      return;
    }
    triggerDebounced('file event');
  };
  watchFs.onDidChange(fsChanged);
  watchFs.onDidCreate(fsChanged);
  watchFs.onDidDelete(fsChanged);

  const renameSub = vscode.workspace.onDidRenameFiles(() => {
    if (!getConfigValue('runOnFileEvents', true)) {
      return;
    }
    triggerDebounced('rename');
  });

  const cfgSub = vscode.workspace.onDidChangeConfiguration((evt) => {
    if (!evt.affectsConfiguration('repoContractEnforcer')) {
      return;
    }
    triggerNow('config changed');
  });

  context.subscriptions.push(saveSub, changeSub, watchFs, renameSub, cfgSub);

  // initial run once workspace is open
  triggerNow('startup');
}

function deactivate() {
  // Runtime cleanup is handled by the context subscription disposable.
}

module.exports = {
  activate,
  deactivate,
  __test: {
    isRelevantDocument,
    clampDebounceMs,
    clampRunTimeoutMs,
    parseReport,
  },
};
