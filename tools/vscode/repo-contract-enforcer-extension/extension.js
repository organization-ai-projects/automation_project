const vscode = require('vscode');
const cp = require('child_process');
const path = require('path');

const DIAGNOSTIC_SOURCE = 'repo-contract-enforcer';

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

function runEnforcer(collection) {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (!folder) {
    return;
  }

  const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
  const mode = cfg.get('mode', 'auto');
  const cmd = cfg.get('command', 'cargo');

  const args = ['run', '-q', '-p', 'repo_contract_enforcer_backend', '--', 'serve'];
  const child = cp.spawn(cmd, args, { cwd: folder.uri.fsPath, stdio: ['pipe', 'pipe', 'pipe'] });
  let stdout = '';
  let stderr = '';

  child.stdout.on('data', (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on('data', (chunk) => {
    stderr += chunk.toString();
  });
  child.on('error', () => {
    vscode.window.setStatusBarMessage('repo_contract_enforcer: backend spawn failed', 4000);
  });
  child.on('close', (_code) => {
    const byFile = new Map();

    let parsedReportEnvelope;
    try {
      const lines = stdout
        .split(/\r?\n/)
        .map((s) => s.trim())
        .filter((s) => s.startsWith('{') && s.endsWith('}'));
      const reportLine = lines.find((line) => line.includes('"type":"report"'));
      if (!reportLine) {
        throw new Error('missing report line');
      }
      parsedReportEnvelope = JSON.parse(reportLine);
    } catch (_parseErr) {
      if (stderr && stderr.trim().length > 0) {
        vscode.window.setStatusBarMessage('repo_contract_enforcer: invalid JSON output', 4000);
      }
      return;
    }

    const parsed = parsedReportEnvelope.report_json || {};
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
  });

  const request = {
    id: 'vscode-1',
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
  context.subscriptions.push(collection);

  const trigger = () => runEnforcer(collection);

  const cmd = vscode.commands.registerCommand('repoContractEnforcer.runCheck', trigger);
  context.subscriptions.push(cmd);

  const cfg = vscode.workspace.getConfiguration('repoContractEnforcer');
  if (cfg.get('runOnSave', true)) {
    const saveSub = vscode.workspace.onDidSaveTextDocument((doc) => {
      if (doc.languageId === 'rust' || doc.fileName.endsWith('.toml')) {
        trigger();
      }
    });
    context.subscriptions.push(saveSub);
  }

  // initial run once workspace is open
  trigger();
}

function deactivate() {}

module.exports = {
  activate,
  deactivate
};
