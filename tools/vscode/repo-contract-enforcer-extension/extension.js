const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

const DIAGNOSTIC_SOURCE = 'repo-contract-enforcer';
const STATUS_PREFIX = 'Repo Contract Enforcer';
const DEBOUNCE_MIN_MS = 50;
const DEBOUNCE_MAX_MS = 5000;
const DEFAULT_DEBOUNCE_MS = 300;

let extensionRuntime = undefined;

function toSeverity(value) {
  return value === 'ERROR'
    ? vscode.DiagnosticSeverity.Error
    : vscode.DiagnosticSeverity.Warning;
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
  const range = new vscode.Range(line, 0, line, 200);
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

  for (let i = parsed.length - 1; i >= 0; i -= 1) {
    const obj = parsed[i];
    if (obj && obj.type === 'report' && obj.report_json && typeof obj.report_json === 'object') {
      return obj.report_json;
    }
  }

  for (let i = parsed.length - 1; i >= 0; i -= 1) {
    const obj = parsed[i];
    if (obj && Array.isArray(obj.violations)) {
      return obj;
    }
  }

  return undefined;
}

function updateStatus(runtime, text) {
  runtime.statusBar.text = `$(shield) ${STATUS_PREFIX}: ${text}`;
  runtime.statusBar.show();
}

function runEnforcer(runtime, reason) {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    return;
  }

  const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
  const mode = cfg.get('mode', 'auto');
  const cmd = cfg.get('command', 'cargo');
  const runId = ++runtime.runSeq;

  if (runtime.currentChild && !runtime.currentChild.killed) {
    runtime.currentChild.kill();
  }

  const args = ['run', '-q', '-p', 'repo_contract_enforcer_backend', '--', 'serve'];
  const child = cp.spawn(cmd, args, { cwd: folder.uri.fsPath, stdio: ['pipe', 'pipe', 'pipe'] });
  runtime.currentChild = child;
  let stdout = '';
  let stderr = '';

  updateStatus(runtime, `running (${reason})...`);

  child.stdout.on('data', (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on('data', (chunk) => {
    stderr += chunk.toString();
  });
  child.on('error', (err) => {
    if (runId !== runtime.runSeq) {
      return;
    }
    updateStatus(runtime, 'backend spawn failed');
    vscode.window.setStatusBarMessage(`${STATUS_PREFIX}: backend spawn failed (${String(err?.message || 'unknown error')})`, 5000);
  });

  child.on('close', (code, signal) => {
    if (runId !== runtime.runSeq) {
      return;
    }
    runtime.currentChild = undefined;

    if (signal === 'SIGTERM') {
      updateStatus(runtime, 'superseded by newer check');
      return;
    }

    const byFile = new Map();

    const parsed = parseReport(stdout);
    if (!parsed) {
      const errSuffix = stderr && stderr.trim().length > 0 ? `: ${stderr.trim().split(/\r?\n/).slice(-1)[0]}` : '';
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
      const list = byFile.get(uri.toString()) || { uri, diagnostics: [] };
      list.diagnostics.push(buildDiagnostic(v));
      byFile.set(uri.toString(), list);
    }

    collection.clear();
    for (const { uri, diagnostics } of byFile.values()) {
      collection.set(uri, diagnostics);
    }

    vscode.commands.executeCommand('setContext', 'repoContractEnforcer.lastCount', violations.length);
    const outcome = code === 0 ? `${violations.length} issue(s)` : `${violations.length} issue(s), exit ${code}`;
    updateStatus(runtime, outcome);
  });

  const request = {
    id: `vscode-${runId}`,
    type: 'checkRepo',
    root_path: folder.uri.fsPath,
    mode
  };
  const shutdown = { id: 'vscode-shutdown', type: 'shutdown' };
  child.stdin.write(JSON.stringify(request) + '\n');
  child.stdin.write(JSON.stringify(shutdown) + '\n');
  child.stdin.end();
}

function activate(context) {
  const collection = vscode.languages.createDiagnosticCollection(DIAGNOSTIC_SOURCE);
  const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 50);
  updateStatus({ statusBar }, 'idle');

  const runtime = {
    collection,
    statusBar,
    currentChild: undefined,
    runSeq: 0,
    debounceTimer: undefined
  };
  extensionRuntime = runtime;

  const triggerNow = (reason) => runEnforcer(runtime, reason);
  const triggerDebounced = (reason) => {
    const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
    const debounceMs = clampDebounceMs(cfg.get('debounceMs', DEFAULT_DEBOUNCE_MS));
    if (runtime.debounceTimer) {
      clearTimeout(runtime.debounceTimer);
    }
    runtime.debounceTimer = setTimeout(() => {
      runtime.debounceTimer = undefined;
      triggerNow(reason);
    }, debounceMs);
  };

  context.subscriptions.push(collection, statusBar);

  const cmd = vscode.commands.registerCommand('repoContractEnforcer.runCheck', () => triggerNow('manual'));
  context.subscriptions.push(cmd);

  const saveSub = vscode.workspace.onDidSaveTextDocument((doc) => {
    const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
    if (!cfg.get('runOnSave', true) || !isRelevantDocument(doc)) {
      return;
    }
    triggerNow('save');
  });

  const changeSub = vscode.workspace.onDidChangeTextDocument((evt) => {
    const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
    if (!cfg.get('runOnChange', true) || !isRelevantDocument(evt.document)) {
      return;
    }
    if (!evt.contentChanges || evt.contentChanges.length === 0) {
      return;
    }
    triggerDebounced('edit');
  });

  const watchFs = vscode.workspace.createFileSystemWatcher('**/*.{rs,toml}');
  const fsChanged = () => {
    const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
    if (!cfg.get('runOnFileEvents', true)) {
      return;
    }
    triggerDebounced('file event');
  };
  watchFs.onDidChange(fsChanged);
  watchFs.onDidCreate(fsChanged);
  watchFs.onDidDelete(fsChanged);

  const renameSub = vscode.workspace.onDidRenameFiles(() => {
    const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
    if (!cfg.get('runOnFileEvents', true)) {
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
  if (!extensionRuntime) {
    return;
  }
  if (extensionRuntime.debounceTimer) {
    clearTimeout(extensionRuntime.debounceTimer);
    extensionRuntime.debounceTimer = undefined;
  }
  if (extensionRuntime.currentChild && !extensionRuntime.currentChild.killed) {
    extensionRuntime.currentChild.kill();
  }
}

module.exports = {
  activate,
  deactivate
};
