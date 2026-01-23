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

Configure the extension in your Zed `settings.json`:

```json
{
  "lsp": {
    "fish-lsp": {
      "initialization_options": {
        "fish_lsp_show_client_popups": false,
        "fish_lsp_diagnostic_disable_error_codes": [1001, 1002],
        "fish_lsp_max_diagnostics": 100
      },
      "settings": {}
    }
  }
}
```

### Available Options

| Option                                    | Type    | Default                                      | Description                              |
| ----------------------------------------- | ------- | -------------------------------------------- | ---------------------------------------- |
| `fish_lsp_show_client_popups`             | boolean | `false`                                      | Show popup notifications                 |
| `fish_lsp_diagnostic_disable_error_codes` | array   | `[]`                                         | Disable specific diagnostic codes        |
| `fish_lsp_max_diagnostics`                | number  | `0`                                          | Max diagnostics per file (0 = unlimited) |
| `fish_lsp_enabled_handlers`               | array   | `[]`                                         | Enable specific LSP handlers             |
| `fish_lsp_disabled_handlers`              | array   | `[]`                                         | Disable specific LSP handlers            |
| `fish_lsp_all_indexed_paths`              | array   | `["$__fish_config_dir", "$__fish_data_dir"]` | Paths to index                           |

See [fish-lsp documentation](https://github.com/ndonfris/fish-lsp) for all options.

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
