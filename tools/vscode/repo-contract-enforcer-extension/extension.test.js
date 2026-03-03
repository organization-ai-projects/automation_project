const test = require('node:test');
const assert = require('node:assert/strict');
const Module = require('node:module');

const originalLoad = Module._load;
Module._load = function patchedLoad(request, parent, isMain) {
  if (request === 'vscode') {
    return {
      DiagnosticSeverity: { Error: 0, Warning: 1 }
    };
  }
  return originalLoad(request, parent, isMain);
};

const ext = require('./extension.js');
const { parseReport, clampDebounceMs, isRelevantDocument } = ext.__test;

test('parseReport returns report_json from report envelope', () => {
  const raw = [
    '{"type":"log","message":"hello"}',
    '{"type":"report","report_json":{"violations":[{"path":"a.rs"}]}}'
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
