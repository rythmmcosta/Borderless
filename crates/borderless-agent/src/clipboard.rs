/// Read the current clipboard text.
/// Tries X11/Wayland via arboard; falls back to /run/borderless-agent/clipboard on headless.
pub fn read() -> Option<String> {
    // X11 / Wayland path
    #[cfg(target_os = "linux")]
    if std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some() {
        if let Ok(mut ctx) = arboard::Clipboard::new() {
            if let Ok(t) = ctx.get_text() {
                if !t.is_empty() { return Some(t); }
            }
        }
    }

    // Headless fallback: read from tmpfs file (written by `borderless-agent copy`)
    std::fs::read_to_string(headless_path()).ok().filter(|s| !s.is_empty())
}

/// Write text to the local clipboard.
/// Tries X11/Wayland via arboard; falls back to /run/borderless-agent/clipboard on headless.
pub fn write(text: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    if std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some() {
        if let Ok(mut ctx) = arboard::Clipboard::new() {
            if ctx.set_text(text).is_ok() { return Ok(()); }
        }
    }

    // Headless fallback
    let path = headless_path();
    if let Some(p) = path.parent() { std::fs::create_dir_all(p)?; }
    std::fs::write(path, text)?;
    Ok(())
}

fn headless_path() -> std::path::PathBuf {
    std::path::PathBuf::from("/run/borderless-agent/clipboard")
}
