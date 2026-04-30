use std::fs;
use zed_extension_api::{
    self as zed,
    lsp::{Completion, Symbol},
    settings::LspSettings,
    CodeLabel, CodeLabelSpan, LanguageServerId, Result,
};

const BINARY_NAME: &str = "fish-lsp";
const NPM_PACKAGE: &str = "fish-lsp";
const SERVER_PATH: &str = "node_modules/fish-lsp/dist/fish-lsp";

fn merge_json(base: &mut zed::serde_json::Value, overrides: zed::serde_json::Value) {
    if let (Some(base_obj), Some(override_obj)) = (base.as_object_mut(), overrides.as_object()) {
        for (key, value) in override_obj {
            base_obj.insert(key.clone(), value.clone());
        }
    }
}

fn initialization_options_with_defaults(
    user_options: Option<zed::serde_json::Value>,
) -> zed::serde_json::Value {
    let mut options = zed::serde_json::json!({
        "fish_lsp_show_client_popups": false
    });

    if let Some(user_options) = user_options {
        merge_json(&mut options, user_options);
    }

    options
}

fn label_for_completion_item(completion: Completion) -> Option<CodeLabel> {
    let label = &completion.label;
    let detail = completion.detail.as_deref();

    match completion.kind? {
        zed::lsp::CompletionKind::Function => {
            let code = match detail {
                Some(sig) => format!("function {label}{sig}"),
                None => format!("function {label}"),
            };
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(9..9 + label.len())],
                filter_range: (0..label.len()).into(),
                code,
            })
        }
        zed::lsp::CompletionKind::Variable => {
            let code = format!("set {label}");
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(4..4 + label.len())],
                filter_range: (0..label.len()).into(),
                code,
            })
        }
        zed::lsp::CompletionKind::Keyword => Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..label.len())],
            filter_range: (0..label.len()).into(),
            code: label.clone(),
        }),
        // External commands
        zed::lsp::CompletionKind::Class => Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..label.len())],
            filter_range: (0..label.len()).into(),
            code: label.clone(),
        }),
        // Abbreviations - show expansion if available
        zed::lsp::CompletionKind::Snippet => {
            let code = match detail {
                Some(expansion) => format!("{label} -> {expansion}"),
                None => label.clone(),
            };
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..label.len())],
                filter_range: (0..label.len()).into(),
                code,
            })
        }
        // File paths
        zed::lsp::CompletionKind::File => Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..label.len())],
            filter_range: (0..label.len()).into(),
            code: label.clone(),
        }),
        // Arguments/flags - show description if available
        zed::lsp::CompletionKind::Property => {
            let code = match detail {
                Some(desc) => format!("{label}  # {desc}"),
                None => label.clone(),
            };
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..label.len())],
                filter_range: (0..label.len()).into(),
                code,
            })
        }
        // Fish events
        zed::lsp::CompletionKind::Event => {
            let code = format!("--on-event {label}");
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(11..11 + label.len())],
                filter_range: (0..label.len()).into(),
                code,
            })
        }
        // Operators (pipes, wildcards)
        zed::lsp::CompletionKind::Operator => Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..label.len())],
            filter_range: (0..label.len()).into(),
            code: label.clone(),
        }),
        // Aliases
        zed::lsp::CompletionKind::Constructor => {
            let code = format!("alias {label}");
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(6..6 + label.len())],
                filter_range: (0..label.len()).into(),
                code,
            })
        }
        // Status codes
        zed::lsp::CompletionKind::EnumMember => Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(0..label.len())],
            filter_range: (0..label.len()).into(),
            code: label.clone(),
        }),
        _ => None,
    }
}

fn label_for_symbol_item(symbol: Symbol) -> Option<CodeLabel> {
    let name = &symbol.name;

    match symbol.kind {
        zed::lsp::SymbolKind::Function => {
            let code = format!("function {name}");
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(9..9 + name.len())],
                filter_range: (0..name.len()).into(),
                code,
            })
        }
        zed::lsp::SymbolKind::Variable => {
            let code = format!("set {name}");
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(4..4 + name.len())],
                filter_range: (0..name.len()).into(),
                code,
            })
        }
        zed::lsp::SymbolKind::Constant => {
            let code = format!("set -g {name}");
            Some(CodeLabel {
                spans: vec![CodeLabelSpan::code_range(7..7 + name.len())],
                filter_range: (0..name.len()).into(),
                code,
            })
        }
        _ => None,
    }
}

struct FishExtension {
    did_find_server: bool,
}

impl FishExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH)
            .map(|m| m.is_file())
            .unwrap_or(false)
    }

    fn get_server_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if self.did_find_server && self.server_exists() {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = zed::npm_package_latest_version(NPM_PACKAGE)?;
        let installed_version = zed::npm_package_installed_version(NPM_PACKAGE)?;

        let needs_install =
            !self.server_exists() || installed_version.as_ref() != Some(&latest_version);

        if needs_install {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(NPM_PACKAGE, &latest_version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        return Err(format!(
                            "installed {NPM_PACKAGE} but server not found at {SERVER_PATH}"
                        ));
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        return Err(format!("failed to install {NPM_PACKAGE}: {error}"));
                    }
                    // Existing installation works, continue despite update failure
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for FishExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(zed::Command {
                command: path,
                args: vec!["start".to_string()],
                env: Default::default(),
            });
        }

        let server_path = self.get_server_path(language_server_id)?;

        let full_path = std::env::current_dir()
            .map_err(|e| format!("failed to get current directory: {e}"))?
            .join(&server_path)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![full_path, "start".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let user_options = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?
            .initialization_options;

        Ok(Some(initialization_options_with_defaults(user_options)))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?
            .settings
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        label_for_completion_item(completion)
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        label_for_symbol_item(symbol)
    }
}

zed::register_extension!(FishExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed::lsp::{CompletionKind, SymbolKind};
    use zed_extension_api::Extension;

    fn completion(kind: CompletionKind, label: &str, detail: Option<&str>) -> Completion {
        Completion {
            label: label.to_string(),
            label_details: None,
            detail: detail.map(str::to_string),
            kind: Some(kind),
            insert_text_format: None,
        }
    }

    fn symbol(kind: SymbolKind, name: &str) -> Symbol {
        Symbol {
            kind,
            name: name.to_string(),
        }
    }

    fn assert_code_label(
        label: CodeLabel,
        code: &str,
        code_range: std::ops::Range<u32>,
        filter_range: std::ops::Range<u32>,
    ) {
        assert_eq!(label.code, code);
        assert_eq!(label.filter_range.start, filter_range.start);
        assert_eq!(label.filter_range.end, filter_range.end);
        assert_eq!(label.spans.len(), 1);

        match &label.spans[0] {
            CodeLabelSpan::CodeRange(range) => {
                assert_eq!(range.start, code_range.start);
                assert_eq!(range.end, code_range.end);
            }
            CodeLabelSpan::Literal(_) => panic!("expected code range span"),
        }
    }

    #[test]
    fn extension_initializes_without_server() {
        let ext = FishExtension::new();
        assert!(!ext.did_find_server);
    }

    #[test]
    fn server_path_constant_is_valid() {
        assert!(SERVER_PATH.contains("fish-lsp"));
        assert!(SERVER_PATH.starts_with("node_modules/"));
    }

    #[test]
    fn binary_name_matches_package() {
        assert_eq!(BINARY_NAME, NPM_PACKAGE);
    }

    #[test]
    fn merge_json_overrides_top_level_keys() {
        let mut base = zed::serde_json::json!({
            "fish_lsp_show_client_popups": false,
            "fish_lsp_max_diagnostics": 10,
        });
        let overrides = zed::serde_json::json!({
            "fish_lsp_max_diagnostics": 100,
            "fish_lsp_log_level": "debug",
        });

        merge_json(&mut base, overrides);

        assert_eq!(
            base,
            zed::serde_json::json!({
                "fish_lsp_show_client_popups": false,
                "fish_lsp_max_diagnostics": 100,
                "fish_lsp_log_level": "debug",
            })
        );
    }

    #[test]
    fn initialization_options_include_defaults_and_user_overrides() {
        assert_eq!(
            initialization_options_with_defaults(None),
            zed::serde_json::json!({
                "fish_lsp_show_client_popups": false,
            })
        );

        assert_eq!(
            initialization_options_with_defaults(Some(zed::serde_json::json!({
                "fish_lsp_show_client_popups": true,
                "fish_lsp_max_diagnostics": 100,
            }))),
            zed::serde_json::json!({
                "fish_lsp_show_client_popups": true,
                "fish_lsp_max_diagnostics": 100,
            })
        );
    }

    #[test]
    fn completion_labels_cover_fish_lsp_completion_kinds() {
        assert_code_label(
            label_for_completion_item(completion(CompletionKind::Function, "greet", Some(" name")))
                .unwrap(),
            "function greet name",
            9..14,
            0..5,
        );
        assert_code_label(
            label_for_completion_item(completion(CompletionKind::Variable, "status", None))
                .unwrap(),
            "set status",
            4..10,
            0..6,
        );
        assert_code_label(
            label_for_completion_item(completion(
                CompletionKind::Property,
                "--help",
                Some("show help"),
            ))
            .unwrap(),
            "--help  # show help",
            0..6,
            0..6,
        );
        assert_code_label(
            label_for_completion_item(completion(CompletionKind::Event, "fish_prompt", None))
                .unwrap(),
            "--on-event fish_prompt",
            11..22,
            0..11,
        );
        assert_code_label(
            label_for_completion_item(completion(CompletionKind::Constructor, "ll", None)).unwrap(),
            "alias ll",
            6..8,
            0..2,
        );
        assert_code_label(
            label_for_completion_item(completion(
                CompletionKind::Snippet,
                "gs",
                Some("git status"),
            ))
            .unwrap(),
            "gs -> git status",
            0..2,
            0..2,
        );
    }

    #[test]
    fn completion_labels_return_plain_labels_for_token_like_kinds() {
        for kind in [
            CompletionKind::Keyword,
            CompletionKind::Class,
            CompletionKind::File,
            CompletionKind::Operator,
            CompletionKind::EnumMember,
        ] {
            assert_code_label(
                label_for_completion_item(completion(kind, "token", None)).unwrap(),
                "token",
                0..5,
                0..5,
            );
        }
    }

    #[test]
    fn completion_labels_ignore_unsupported_or_missing_kinds() {
        assert!(label_for_completion_item(Completion {
            label: "plain".to_string(),
            label_details: None,
            detail: None,
            kind: None,
            insert_text_format: None,
        })
        .is_none());

        assert!(
            label_for_completion_item(completion(CompletionKind::Method, "call", None)).is_none()
        );
    }

    #[test]
    fn symbol_labels_cover_fish_lsp_symbol_kinds() {
        assert_code_label(
            label_for_symbol_item(symbol(SymbolKind::Function, "greet")).unwrap(),
            "function greet",
            9..14,
            0..5,
        );
        assert_code_label(
            label_for_symbol_item(symbol(SymbolKind::Variable, "path")).unwrap(),
            "set path",
            4..8,
            0..4,
        );
        assert_code_label(
            label_for_symbol_item(symbol(SymbolKind::Constant, "PATH")).unwrap(),
            "set -g PATH",
            7..11,
            0..4,
        );
    }

    #[test]
    fn symbol_labels_ignore_unsupported_kinds() {
        assert!(label_for_symbol_item(symbol(SymbolKind::Class, "Thing")).is_none());
    }
}
