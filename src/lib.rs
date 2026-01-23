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

struct FishExtension {
    did_find_server: bool,
}

impl FishExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map(|m| m.is_file()).unwrap_or(false)
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

        let needs_install = !self.server_exists()
            || installed_version.as_ref() != Some(&latest_version);

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
        let mut options = zed::serde_json::json!({
            "fish_lsp_show_client_popups": false
        });

        if let Some(user_options) = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?
            .initialization_options
        {
            merge_json(&mut options, user_options);
        }

        Ok(Some(options))
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

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
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
}

zed::register_extension!(FishExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use zed_extension_api::Extension;

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
}
