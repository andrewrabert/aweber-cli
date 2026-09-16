//! arboard, then an OSC 52 write to the terminal, then a file under the temp
//! directory.

use std::io::Write as _;

use crate::ports::{Clipboard, CopyError, CopyRoute};

pub struct SystemClipboard;

impl Clipboard for SystemClipboard {
    fn copy(&self, text: &str) -> Result<CopyRoute, CopyError> {
        let mut reasons = Vec::new();
        match native(text) {
            Ok(()) => return Ok(CopyRoute::Native),
            Err(reason) => reasons.push(reason),
        }
        match osc52(text) {
            Ok(()) => return Ok(CopyRoute::Osc52),
            Err(reason) => reasons.push(reason),
        }
        match temp_file(text) {
            Ok(path) => Ok(CopyRoute::TempFile(path)),
            Err(reason) => {
                reasons.push(reason);
                Err(CopyError {
                    reason: reasons.join("; "),
                })
            }
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn native(text: &str) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|error| error.to_string())?;
    clipboard
        .set_text(text.to_string())
        .map_err(|error| error.to_string())
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn native(_text: &str) -> Result<(), String> {
    Err("no native clipboard on this platform".to_string())
}

fn osc52(text: &str) -> Result<(), String> {
    let encoded = base64(text.as_bytes());
    let mut out = std::io::stdout();
    write!(out, "\x1b]52;c;{encoded}\x07").map_err(|error| error.to_string())?;
    out.flush().map_err(|error| error.to_string())
}

fn temp_file(text: &str) -> Result<std::path::PathBuf, String> {
    let path = std::env::temp_dir().join(format!("aweber-tui-{}.txt", uuid::Uuid::new_v4()));
    std::fs::write(&path, text.as_bytes()).map_err(|error| error.to_string())?;
    Ok(path)
}

/// OSC 52 carries its payload base64-encoded.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let mut word = [0u8; 3];
        word[..chunk.len()].copy_from_slice(chunk);
        let packed = (u32::from(word[0]) << 16) | (u32::from(word[1]) << 8) | u32::from(word[2]);
        for index in 0..4 {
            if index <= chunk.len() {
                let shift = 18 - 6 * index;
                out.push(char::from(ALPHABET[((packed >> shift) & 0x3f) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}
