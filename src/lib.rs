use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result, settings::LspSettings};

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
                    "--logLevel=Information".into(),
                    log_path,
                    "--stdio".into(),
                ]
            }),
            env: Default::default(),
        })
    }
}

zed::register_extension!(RoslynCsharpExtension);
