use arboard::Clipboard;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum ClipboardError {
    AccessFailed(String),
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardError::AccessFailed(msg) => write!(f, "Failed to access clipboard: {}", msg),
        }
    }
}

impl std::error::Error for ClipboardError {}

impl From<arboard::Error> for ClipboardError {
    fn from(err: arboard::Error) -> Self {
        ClipboardError::AccessFailed(err.to_string())
    }
}

pub fn copy_and_clear(text: String, timeout_secs: u64) -> Result<(), ClipboardError> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(timeout_secs));
        if let Ok(mut clipboard) = Clipboard::new() {
            let _ = clipboard.set_text("".to_string());
        }
    });

    Ok(())
}
