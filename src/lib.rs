use std::{fs, path};
use zed_extension_api::{
    self as zed, LanguageServerId, Result,
    serde_json::{self, json},
    settings::LspSettings,
};

struct RoslynBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct RoslynCsharpExtension {
    cached_binary_path: Option<String>,
}

impl RoslynCsharpExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<RoslynBinary> {
        let binary_settings = LspSettings::for_worktree("roslyn", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(RoslynBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("Microsoft.CodeAnalysis.LanguageServer") {
            return Ok(RoslynBinary {
                path,
                args: binary_args,
            });
        }
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(RoslynBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "Crashdummyy/roslynLanguageServer",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "microsoft.codeanalysis.languageserver.{os}-{architecture}.zip",
            os = match platform {
                zed::Os::Mac => "osx",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "win",
            },
            architecture = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X8664 => "x64",
                zed::Architecture::X86 => "x64",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("No asset found matching {:?}", asset_name))?;

        let version_dir = format!("roslyn-server-{}", release.version);
        let binary_path = format!("{version_dir}/Microsoft.CodeAnalysis.LanguageServer");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|err| format!("Failed to download file {err}"))?;

            zed::make_file_executable(&binary_path);

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }
        self.cached_binary_path = Some(binary_path.clone());
        Ok(RoslynBinary {
            path: binary_path,
            args: binary_args,
        })
    }
}

impl zed::Extension for RoslynCsharpExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let roslyn_binary = self.language_server_binary(language_server_id, worktree)?;
        let log_path = format!("--extensionLogDirectory={}", worktree.root_path());
        Ok(zed::Command {
            command: roslyn_binary.path,
            args: roslyn_binary.args.unwrap_or_else(|| {
                vec![
                    "--stdio".into(),
                    "--logLevel=Information".into(),
                    log_path,
                    "--razorSourceGenerator=/etc/profiles/per-user/genga/lib/roslyn-ls/Microsoft.CodeAnalysis.Razor.Compiler.dll".into(),
                    "--razorDesignTimePath=/etc/profiles/per-user/genga/lib/rzls/Targets/Microsoft.NET.Sdk.Razor.DesignTime.targets".into(),
                ]
            }),
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let csharp_workspace_settings = json!({
          "csharp|inlay_hints.csharp_enable_inlay_hints_for_implicit_object_creation": true,
          "csharp|inlay_hints.csharp_enable_inlay_hints_for_implicit_variable_types": true,
          "csharp|inlay_hints.csharp_enable_inlay_hints_for_lambda_parameter_types": true,
          "csharp|inlay_hints.csharp_enable_inlay_hints_for_types": true,
          "csharp|inlay_hints.dotnet_enable_inlay_hints_for_indexer_parameters": true,
          "csharp|inlay_hints.dotnet_enable_inlay_hints_for_literal_parameters": true,
          "csharp|inlay_hints.dotnet_enable_inlay_hints_for_object_creation_parameters": true,
          "csharp|inlay_hints.dotnet_enable_inlay_hints_for_other_parameters": true,
          "csharp|inlay_hints.dotnet_enable_inlay_hints_for_parameters": true,
          "csharp|inlay_hints.dotnet_suppress_inlay_hints_for_parameters_that_differ_only_by_suffix": true,
          "csharp|inlay_hints.dotnet_suppress_inlay_hints_for_parameters_that_match_argument_name": true,
          "csharp|inlay_hints.dotnet_suppress_inlay_hints_for_parameters_that_match_method_intent": true,
          "csharp|code_lens.dotnet_enable_references_code_lens": true,
          "csharp|formatting.dotnet_organize_imports_on_format": true,
          "csharp|completion.dotnet_show_completion_items_from_unimported_namespaces": true,
        });
        Ok(Some(csharp_workspace_settings))
    }
}

zed::register_extension!(RoslynCsharpExtension);
