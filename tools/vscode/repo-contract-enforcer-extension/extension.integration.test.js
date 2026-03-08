const test = require('node:test');
const assert = require('node:assert/strict');
const Module = require('node:module');
const path = require('node:path');

function createVscodeMock() {
  const commandHandlers = new Map();
  const executedCommands = [];
  const configValues = new Map([
    ['runOnStartup', false],
    ['syncProblemsOnTreeOpen', true],
  ]);

  class EventEmitter {
    constructor() {
      this._listeners = [];
      this.event = (listener) => {
        this._listeners.push(listener);
        return { dispose() {} };
      };
    }
    fire(value) {
      for (const listener of this._listeners) {
        listener(value);
      }
    }
    dispose() {
      this._listeners = [];
    }
  }

  class FakeCollection {
    constructor() {
      this._store = new Map();
    }
    clear() {
      this._store.clear();
    }
    set(uri, diagnostics) {
      this._store.set(uri, diagnostics);
    }
    get(uri) {
      return this._store.get(uri);
    }
    [Symbol.iterator]() {
      return this._store[Symbol.iterator]();
    }
  }

  class Position {
    constructor(line, character) {
      this.line = line;
      this.character = character;
    }
  }

  class Range {
    constructor(startLine, startChar, endLine, endChar) {
      this.start = new Position(startLine, startChar);
      this.end = new Position(endLine, endChar);
    }
    contains(position) {
      if (!position) {
        return false;
      }
      if (position.line < this.start.line || position.line > this.end.line) {
        return false;
      }
      if (position.line === this.start.line && position.character < this.start.character) {
        return false;
      }
      if (position.line === this.end.line && position.character > this.end.character) {
        return false;
      }
      return true;
    }
  }

  class Selection {
    constructor(start, end) {
      this.start = start;
      this.end = end;
    }
  }

  class TreeItem {
    constructor(label, collapsibleState) {
      this.label = label;
      this.collapsibleState = collapsibleState;
    }
  }

  class ThemeIcon {
    constructor(id) {
      this.id = id;
    }
  }

  class CodeAction {
    constructor(title, kind) {
      this.title = title;
      this.kind = kind;
    }
  }

  class MarkdownString {
    constructor() {
      this.value = '';
      this.isTrusted = false;
    }
    appendMarkdown(text) {
      this.value += text;
    }
  }

  class Hover {
    constructor(contents) {
      this.contents = contents;
    }
  }

  const vscode = {
    DiagnosticSeverity: { Error: 0, Warning: 1, Information: 2, Hint: 3 },
    StatusBarAlignment: { Left: 1 },
    CodeActionKind: { QuickFix: 'quickfix' },
    TreeItemCollapsibleState: { None: 0, Collapsed: 1 },
    TextEditorRevealType: { InCenter: 0 },
    EventEmitter,
    Position,
    Range,
    Selection,
    TreeItem,
    ThemeIcon,
    CodeAction,
    MarkdownString,
    Hover,
    Uri: {
      file(fsPath) {
        return { fsPath };
      },
      joinPath(base, next) {
        return { fsPath: path.join(base.fsPath, next) };
      },
    },
    commands: {
      registerCommand(name, handler) {
        commandHandlers.set(name, handler);
        return {
          dispose() {
            commandHandlers.delete(name);
          },
        };
      },
      executeCommand(name, ...args) {
        executedCommands.push({ name, args });
        if (commandHandlers.has(name)) {
          return commandHandlers.get(name)(...args);
        }
        return undefined;
      },
    },
    languages: {
      createDiagnosticCollection() {
        return new FakeCollection();
      },
      registerCodeActionsProvider() {
        return { dispose() {} };
      },
      registerHoverProvider() {
        return { dispose() {} };
      },
    },
    window: {
      activeTextEditor: undefined,
      createStatusBarItem() {
        return { text: '', show() {}, dispose() {} };
      },
      createOutputChannel() {
        return { appendLine() {}, show() {}, dispose() {} };
      },
      setStatusBarMessage() {},
      createTreeView() {
        return { dispose() {} };
      },
      showSaveDialog: async () => undefined,
      openTextDocument: async () => ({ getText: () => '' }),
      showTextDocument: async () => ({ selection: undefined, revealRange() {} }),
    },
    workspace: {
      workspaceFolders: [{ name: 'ws', uri: { fsPath: '/tmp/ws' } }],
      getWorkspaceFolder() {
        return { name: 'ws', uri: { fsPath: '/tmp/ws' } };
      },
      asRelativePath(input) {
        if (typeof input === 'string') {
          return input;
        }
        if (input && typeof input.fsPath === 'string') {
          return input.fsPath;
        }
        return String(input || '');
      },
      getConfiguration() {
        return {
          get(key, defaultValue) {
            return configValues.has(key) ? configValues.get(key) : defaultValue;
          },
        };
      },
      onDidSaveTextDocument() {
        return { dispose() {} };
      },
      onDidChangeTextDocument() {
        return { dispose() {} };
      },
      onDidRenameFiles() {
        return { dispose() {} };
      },
      onDidChangeConfiguration() {
        return { dispose() {} };
      },
      createFileSystemWatcher() {
        return {
          onDidChange() {},
          onDidCreate() {},
          onDidDelete() {},
          dispose() {},
        };
      },
      fs: {
        writeFile: async () => {},
      },
    },
    env: {
      clipboard: {
        writeText: async () => {},
      },
    },
  };

  return { vscode, commandHandlers, executedCommands, configValues };
}

const originalLoad = Module._load;
const mock = createVscodeMock();
Module._load = function patchedLoad(request, parent, isMain) {
  if (request === 'vscode') {
    return mock.vscode;
  }
  return originalLoad(request, parent, isMain);
};

const extensionPath = require.resolve('./extension.js');
delete require.cache[extensionPath];
const ext = require('./extension.js');

test('activate registers expected extension commands', () => {
  const context = { subscriptions: [] };
  ext.activate(context);

  assert.equal(mock.commandHandlers.has('repoContractEnforcer.runCheck'), true);
  assert.equal(mock.commandHandlers.has('repoContractEnforcer.focusDiagnosticsView'), true);
  assert.equal(mock.commandHandlers.has('repoContractEnforcer.openDiagnosticLocation'), true);
  assert.equal(mock.commandHandlers.has('repoContractEnforcer.refreshDiagnosticsView'), true);
});

test('focus command routes to explorer and diagnostics view focus', async () => {
  mock.executedCommands.length = 0;
  const handler = mock.commandHandlers.get('repoContractEnforcer.focusDiagnosticsView');
  await handler();

  const names = mock.executedCommands.map((entry) => entry.name);
  assert.equal(names.includes('workbench.view.explorer'), true);
  assert.equal(names.includes('repoContractEnforcerDiagnostics.focus'), true);
});

test('openDiagnosticLocation reveals Problems panel when sync is enabled', () => {
  mock.executedCommands.length = 0;
  mock.configValues.set('syncProblemsOnTreeOpen', true);
  const handler = mock.commandHandlers.get('repoContractEnforcer.openDiagnosticLocation');
  handler({ fsPath: '/tmp/ws/src/lib.rs' }, 1, 2, 1, 4);

  const names = mock.executedCommands.map((entry) => entry.name);
  assert.equal(names.includes('vscode.open'), true);
  assert.equal(names.includes('workbench.actions.view.problems'), true);
});

test('openDiagnosticLocation skips Problems panel when sync is disabled', () => {
  mock.executedCommands.length = 0;
  mock.configValues.set('syncProblemsOnTreeOpen', false);
  const handler = mock.commandHandlers.get('repoContractEnforcer.openDiagnosticLocation');
  handler({ fsPath: '/tmp/ws/src/lib.rs' }, 1, 2, 1, 4);

  const names = mock.executedCommands.map((entry) => entry.name);
  assert.equal(names.includes('vscode.open'), true);
  assert.equal(names.includes('workbench.actions.view.problems'), false);
});
