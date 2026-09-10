//! `$VISUAL`, then `$EDITOR`, then `vi`.

use crate::ports::{BoxFuture, Editor, EditorError};

pub struct SystemEditor;

impl Editor for SystemEditor {
    fn edit(&self, seed: &str, extension: &str) -> BoxFuture<'static, Result<String, EditorError>> {
        let program = std::env::var("VISUAL")
            .or_else(|_| std::env::var("EDITOR"))
            .unwrap_or_else(|_| "vi".to_string());
        let seed = seed.to_string();
        let extension = extension.to_string();
        Box::pin(async move {
            if program.trim().is_empty() {
                return Err(EditorError::NotConfigured);
            }
            let path = std::env::temp_dir()
                .join(format!("aweber-tui-{}.{extension}", uuid::Uuid::new_v4()));
            std::fs::write(&path, seed.as_bytes()).map_err(|error| EditorError::Failed {
                reason: error.to_string(),
            })?;
            let status = tokio::process::Command::new(&program)
                .arg(&path)
                .status()
                .await
                .map_err(|error| EditorError::Failed {
                    reason: error.to_string(),
                })?;
            if !status.success() {
                let _ = std::fs::remove_file(&path);
                return Err(EditorError::Failed {
                    reason: format!("{program} exited with {status}"),
                });
            }
            let text = std::fs::read_to_string(&path).map_err(|error| EditorError::Failed {
                reason: error.to_string(),
            })?;
            let _ = std::fs::remove_file(&path);
            if text.trim().is_empty() {
                Err(EditorError::Empty)
            } else {
                Ok(text)
            }
        })
    }
}
