# Fish LSP Extension for Zed

[![CI](https://github.com/alysson-souza/zed-fish-lsp/actions/workflows/ci.yml/badge.svg)](https://github.com/alysson-souza/zed-fish-lsp/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Fish shell language support with [fish-lsp](https://github.com/ndonfris/fish-lsp) integration for the [Zed editor](https://zed.dev).

Get intelligent code completion, hover documentation, go-to-definition, diagnostics, and more for your Fish shell scripts.

## Features

### Syntax Highlighting

- Full syntax highlighting via tree-sitter
- Keyword bracket matching (`function`/`end`, `if`/`end`, etc.)
- Regex syntax highlighting inside `grep`, `sed`, `rg`, `awk`, `string match -r`

### LSP Support (via fish-lsp)

- Code completion
- Hover documentation
- Go to definition
- Find references
- Diagnostics
- Code actions
- Formatting

### Editor Integration

- Code outline
- Auto-indentation
- Vim/Helix text objects (`vaf` for function, `vic` for control structure body)
- Screen sharing privacy (strings and variables redacted when sharing)
- Runnables (run scripts and functions with a click)

## Installation

1. Clone this repository
2. Open Zed
3. Open the command palette
4. Run `zed: install dev extension`
5. Select this directory

### LSP Binary

The extension will:

1. First check if `fish-lsp` is available in your PATH
2. If not found, automatically download it via Zed's built-in npm

To install fish-lsp manually:

```bash
npm install -g fish-lsp
```

## Configuration

The extension works out of the box. To customize fish-lsp, add options to your Zed `settings.json`:

```jsonc
{
  "lsp": {
    "fish-lsp": {
      "initialization_options": {
        // Disable specific diagnostic codes (default: [])
        "fish_lsp_diagnostic_disable_error_codes": [2002, 2003],
        // Max diagnostics per file (default: 0 = unlimited)
        "fish_lsp_max_diagnostics": 100,
        // Paths to index for completions and go-to-definition (default: ["$__fish_config_dir", "$__fish_data_dir"])
        "fish_lsp_all_indexed_paths": [
          "$__fish_config_dir",
          "$__fish_data_dir",
          "~/my-fish-scripts",
        ],
        // Paths where rename/refactoring is allowed (default: ["$__fish_config_dir"])
        "fish_lsp_modifiable_paths": ["$__fish_config_dir"],
        // Disable specific LSP handlers (default: [])
        // Available: hover, complete, rename, definition, references, formatting, codeAction, signatureHelp, executeCommand
        "fish_lsp_disabled_handlers": ["formatting"],
        // Formatter executable (default: "fish_indent")
        "fish_lsp_format_exec": "fish_indent",
        // Formatter arguments (default: [])
        "fish_lsp_format_args": ["--no-indent"],
        // Log file for debugging (default: "" = disabled)
        "fish_lsp_logfile": "/tmp/fish-lsp.log",
        // Log level (default: "warning") — error, warning, info, debug, trace
        "fish_lsp_log_level": "debug",
      },
    },
  },
}
```

See [fish-lsp](https://github.com/ndonfris/fish-lsp) for diagnostic codes and more options.

### Runnables (Run Buttons)

The extension provides run buttons for Fish scripts and functions. To enable them, add these tasks to your project's `.zed/tasks.json` (or global `~/.config/zed/tasks.json`):

```json
[
  {
    "label": "Run: $ZED_FILENAME",
    "command": "fish $ZED_FILE",
    "shell": { "program": "fish" },
    "tags": ["fish-script"]
  },
  {
    "label": "Run: $ZED_SYMBOL",
    "command": "source $ZED_FILE; and $ZED_SYMBOL",
    "shell": { "program": "fish" },
    "tags": ["fish-function"]
  }
]
```

This enables:

- **Scripts**: Click the run button on the shebang line (`#!/usr/bin/env fish`) to run the entire script
- **Functions**: Click the run button on any `function` definition to run that function

> **Note**: Nested functions (functions defined inside other functions) won't work as standalone runnables since they only exist within their parent function's scope.

## Development

### Prerequisites

- Rust (via rustup)
- Zed editor

### Building

```bash
cargo check --target wasm32-wasip1
```

### Testing

```bash
cargo test
```

## Credits

- [fish-lsp](https://github.com/ndonfris/fish-lsp) - LSP implementation for Fish
- [tree-sitter-fish](https://github.com/ram02z/tree-sitter-fish) - Tree-sitter grammar
- [hasit/zed-fish](https://github.com/hasit/zed-fish) - Reference for tree-sitter queries

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
