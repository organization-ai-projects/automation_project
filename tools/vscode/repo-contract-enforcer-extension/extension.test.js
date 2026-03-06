const test = require('node:test');
const assert = require('node:assert/strict');
const Module = require('node:module');

const originalLoad = Module._load;
Module._load = function patchedLoad(request, parent, isMain) {
  if (request === 'vscode') {
    return {
      DiagnosticSeverity: { Error: 0, Warning: 1, Information: 2, Hint: 3 },
      workspace: {
        asRelativePath(value) {
          if (typeof value === 'string') {
            return value;
          }
          if (value && typeof value.fsPath === 'string') {
            return value.fsPath;
          }
          if (value && typeof value.path === 'string') {
            return value.path;
          }
          return String(value || '');
        },
      },
    };
  }
  return originalLoad(request, parent, isMain);
};

const ext = require('./extension.js');
const {
  parseReport,
  parsePersistentResponseLine,
  clampDebounceMs,
  clampTreeRefreshDebounceMs,
  isRelevantDocument,
  compareDiagnosticsEntries,
  collectDiagnosticsSnapshot,
  buildDiagnosticsSummaryText,
} = ext.__test;

test('parseReport returns report_json from report envelope', () => {
  const raw = [
    '{"type":"log","message":"hello"}',
    '{"type":"report","report_json":{"violations":[{"path":"a.rs"}]}}',
  ].join('\n');

  const parsed = parseReport(raw);
  assert.ok(parsed);
  assert.equal(Array.isArray(parsed.violations), true);
  assert.equal(parsed.violations.length, 1);
});

test('parseReport falls back to direct violations payload', () => {
  const raw = '{"violations":[{"path":"b.rs"},{"path":"c.rs"}]}';
  const parsed = parseReport(raw);
  assert.ok(parsed);
  assert.equal(parsed.violations.length, 2);
});

test('parseReport tolerates malformed lines and returns undefined without report', () => {
  const raw = ['{not-json}', 'plain-text-line', '{"type":"log","message":"x"}'].join('\n');
  const parsed = parseReport(raw);
  assert.equal(parsed, undefined);
});

test('parsePersistentResponseLine parses report and error envelopes', () => {
  const reportLine = JSON.stringify({
    id: 'req-1',
    type: 'report',
    report_json: { violations: [{ path: 'a.rs' }] },
  });
  const errorLine = JSON.stringify({
    id: 'req-2',
    type: 'error',
    code: 'BAD',
    message: 'bad request',
    details: 'x',
  });

  const reportParsed = parsePersistentResponseLine(reportLine);
  const errorParsed = parsePersistentResponseLine(errorLine);

  assert.equal(reportParsed.id, 'req-1');
  assert.equal(reportParsed.kind, 'report');
  assert.equal(reportParsed.report.violations.length, 1);

  assert.equal(errorParsed.id, 'req-2');
  assert.equal(errorParsed.kind, 'error');
  assert.equal(errorParsed.code, 'BAD');
  assert.equal(errorParsed.message, 'bad request');
  assert.equal(errorParsed.details, 'x');
});

test('clampDebounceMs enforces limits and default', () => {
  assert.equal(clampDebounceMs(undefined), 300);
  assert.equal(clampDebounceMs('not-a-number'), 300);
  assert.equal(clampDebounceMs(10), 50);
  assert.equal(clampDebounceMs(300), 300);
  assert.equal(clampDebounceMs(99999), 5000);
});

test('isRelevantDocument accepts rust and toml only', () => {
  assert.equal(isRelevantDocument({ languageId: 'rust', fileName: '/tmp/lib.rs' }), true);
  assert.equal(isRelevantDocument({ languageId: 'plaintext', fileName: '/tmp/Cargo.toml' }), true);
  assert.equal(isRelevantDocument({ languageId: 'markdown', fileName: '/tmp/readme.md' }), false);
  assert.equal(isRelevantDocument(undefined), false);
});

test('clampTreeRefreshDebounceMs enforces limits and default', () => {
  assert.equal(clampTreeRefreshDebounceMs(undefined), 80);
  assert.equal(clampTreeRefreshDebounceMs('not-a-number'), 80);
  assert.equal(clampTreeRefreshDebounceMs(-100), 0);
  assert.equal(clampTreeRefreshDebounceMs(35.8), 35);
  assert.equal(clampTreeRefreshDebounceMs(2500), 2000);
});

test('compareDiagnosticsEntries sorts by file, line, column, message', () => {
  const mk = (path, line, col, message) => ({
    uri: { fsPath: path },
    diag: {
      range: { start: { line, character: col } },
      message,
    },
  });
  const arr = [
    mk('b.rs', 1, 0, 'zzz'),
    mk('a.rs', 3, 0, 'third'),
    mk('a.rs', 2, 5, 'second-col-5'),
    mk('a.rs', 2, 1, 'second-col-1'),
    mk('a.rs', 2, 1, 'second-col-1-b'),
  ];

  arr.sort(compareDiagnosticsEntries);
  const tuples = arr.map((e) => [e.uri.fsPath, e.diag.range.start.line, e.diag.range.start.character, e.diag.message]);
  assert.deepEqual(tuples, [
    ['a.rs', 2, 1, 'second-col-1'],
    ['a.rs', 2, 1, 'second-col-1-b'],
    ['a.rs', 2, 5, 'second-col-5'],
    ['a.rs', 3, 0, 'third'],
    ['b.rs', 1, 0, 'zzz'],
  ]);
});

test('collectDiagnosticsSnapshot aggregates totals and files', () => {
  const collection = new Map();
  collection.set({ fsPath: 'src/a.rs' }, [
    {
      message: 'err-1',
      severity: 0,
      code: 'E001',
      range: { start: { line: 2, character: 4 } },
      source: 'repo-contract-enforcer',
      _repoContractMeta: { ruleId: 'structure', violationCode: 'missing_manifest' },
    },
    {
      message: 'warn-1',
      severity: 1,
      code: 'W001',
      range: { start: { line: 4, character: 0 } },
      source: 'repo-contract-enforcer',
      _repoContractMeta: { ruleId: 'style', violationCode: 'naming' },
    },
  ]);
  collection.set({ fsPath: 'src/b.rs' }, [
    {
      message: 'info-1',
      severity: 2,
      code: '',
      range: { start: { line: 0, character: 0 } },
      source: 'repo-contract-enforcer',
      _repoContractMeta: { ruleId: 'info', violationCode: 'hint' },
    },
  ]);

  const snapshot = collectDiagnosticsSnapshot(collection);
  assert.equal(snapshot.totals.files, 2);
  assert.equal(snapshot.totals.issues, 3);
  assert.equal(snapshot.totals.errors, 1);
  assert.equal(snapshot.totals.warnings, 1);
  assert.equal(snapshot.totals.infos, 1);
  assert.equal(snapshot.files.length, 2);
  assert.equal(snapshot.files[0].path, 'src/a.rs');
  assert.equal(snapshot.files[0].diagnostics[0].line, 3);
  assert.equal(snapshot.files[0].diagnostics[0].col, 5);
});

test('buildDiagnosticsSummaryText renders compact multiline summary', () => {
  const collection = new Map();
  collection.set({ fsPath: 'x.rs' }, [
    {
      message: 'broken',
      severity: 0,
      code: 'E123',
      range: { start: { line: 9, character: 1 } },
      source: 'repo-contract-enforcer',
      _repoContractMeta: { ruleId: 'structure', violationCode: 'bad' },
    },
  ]);

  const summary = buildDiagnosticsSummaryText(collection);
  assert.match(summary, /Repo Contract Enforcer Diagnostics Summary/);
  assert.match(summary, /Totals: 1 issue\(s\), 1 error\(s\), 0 warning\(s\), 1 file\(s\)/);
  assert.match(summary, /x\.rs \(1\)/);
  assert.match(summary, /L10:2 error \[E123\] \{structure\} broken/);
});
