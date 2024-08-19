use std::num::NonZeroUsize;

use accessibility_ng::{AXAttribute, AXUIElement};
use accessibility_sys_ng::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use active_win_pos_rs::get_active_window;
use core_foundation::string::CFString;
use debug_print::debug_println;
use lru::LruCache;
use parking_lot::Mutex;

use crate::SelectedText;

static GET_SELECTED_TEXT_METHOD: Mutex<Option<LruCache<String, u8>>> = Mutex::new(None);

// TDO: optimize / refactor / test later
fn split_file_paths(input: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut current_path = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '\'' => {
                current_path.push(ch);
                in_quotes = !in_quotes;
                if !in_quotes {
                    paths.push(current_path.clone());
                    current_path.clear();
                }
            }
            ' ' if !in_quotes => {
                if !current_path.is_empty() {
                    paths.push(current_path.clone());
                    current_path.clear();
                }
            }
            _ => current_path.push(ch),
        }
    }

    if !current_path.is_empty() {
        paths.push(current_path);
    }

    paths
}

pub fn get_selected_text() -> Result<SelectedText, Box<dyn std::error::Error>> {
    if GET_SELECTED_TEXT_METHOD.lock().is_none() {
        let cache = LruCache::new(NonZeroUsize::new(100).unwrap());
        *GET_SELECTED_TEXT_METHOD.lock() = Some(cache);
    }
    let mut cache = GET_SELECTED_TEXT_METHOD.lock();
    let cache = cache.as_mut().unwrap();
    let app_name = match get_active_window() {
        Ok(window) => window.app_name,
        Err(_) => {
            // user might be in the desktop / home view
            String::new()
        }
    };

    if app_name == "Finder" || app_name.is_empty() {
        if let Ok(text) = get_selected_file_paths_by_clipboard_using_applescript() {
            return Ok(SelectedText {
                is_file_paths: true,
                app_name: app_name,
                text: split_file_paths(&text),
            });
        }
    }

    let mut selected_text = SelectedText {
        is_file_paths: false,
        app_name: app_name.clone(),
        text: vec![],
    };

    if let Some(text) = cache.get(&app_name) {
        if *text == 0 {
            let ax_text = get_selected_text_by_ax()?;
            if !ax_text.is_empty() {
                cache.put(app_name.clone(), 0);
                selected_text.text = vec![ax_text];
                return Ok(selected_text);
            }
        }
        let txt = get_selected_text_by_clipboard_using_applescript()?;
        selected_text.text = vec![txt];
        return Ok(selected_text);
    }
    match get_selected_text_by_ax() {
        Ok(txt) => {
            if !txt.is_empty() {
                cache.put(app_name.clone(), 0);
            }
            selected_text.text = vec![txt];
            Ok(selected_text)
        }
        Err(_) => match get_selected_text_by_clipboard_using_applescript() {
            Ok(txt) => {
                if !txt.is_empty() {
                    cache.put(app_name, 1);
                }
                selected_text.text = vec![txt];
                Ok(selected_text)
            }
            Err(e) => Err(e),
        },
    }
}

fn get_selected_text_by_ax() -> Result<String, Box<dyn std::error::Error>> {
    // debug_println!("get_selected_text_by_ax");
    let system_element = AXUIElement::system_wide();
    let Some(selected_element) = system_element
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXFocusedUIElementAttribute,
        )))
        .map(|element| element.downcast_into::<AXUIElement>())
        .ok()
        .flatten()
    else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No selected element",
        )));
    };
    let Some(selected_text) = selected_element
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXSelectedTextAttribute,
        )))
        .map(|text| text.downcast_into::<CFString>())
        .ok()
        .flatten()
    else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No selected text",
        )));
    };
    Ok(selected_text.to_string())
}

const REGULAR_TEXT_COPY_APPLE_SCRIPT: &str = r#"
use AppleScript version "2.4"
use scripting additions
use framework "Foundation"
use framework "AppKit"

set savedAlertVolume to alert volume of (get volume settings)

-- Back up clipboard contents:
set savedClipboard to the clipboard

set thePasteboard to current application's NSPasteboard's generalPasteboard()
set theCount to thePasteboard's changeCount()

tell application "System Events"
    set volume alert volume 0
end tell

-- Copy selected text to clipboard:
tell application "System Events" to keystroke "c" using {command down}
delay 0.1 -- Without this, the clipboard may have stale data.

tell application "System Events"
    set volume alert volume savedAlertVolume
end tell

if thePasteboard's changeCount() is theCount then
    return ""
end if

set theSelectedText to the clipboard

set the clipboard to savedClipboard

theSelectedText
"#;

const FILE_PATH_COPY_APPLE_SCRIPT: &str = r#"
use AppleScript version "2.4"
use scripting additions
use framework "Foundation"
use framework "AppKit"

set savedAlertVolume to alert volume of (get volume settings)

-- Back up clipboard contents:
set savedClipboard to the clipboard

set thePasteboard to current application's NSPasteboard's generalPasteboard()
set theCount to thePasteboard's changeCount()

tell application "System Events"
    set volume alert volume 0
end tell

-- Copy selected text to clipboard:
tell application "System Events" to keystroke "c" using {command down, option down}
delay 0.1 -- Without this, the clipboard may have stale data.

tell application "System Events"
    set volume alert volume savedAlertVolume
end tell

if thePasteboard's changeCount() is theCount then
    return ""
end if

set theSelectedText to the clipboard

set the clipboard to savedClipboard

theSelectedText
"#;

fn get_selected_text_by_clipboard_using_applescript() -> Result<String, Box<dyn std::error::Error>>
{
    // debug_println!("get_selected_text_by_clipboard_using_applescript");
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(REGULAR_TEXT_COPY_APPLE_SCRIPT)
        .output()?;
    if output.status.success() {
        let content = String::from_utf8(output.stdout)?;
        let content = content.trim();
        Ok(content.to_string())
    } else {
        let err = output
            .stderr
            .into_iter()
            .map(|c| c as char)
            .collect::<String>()
            .into();
        Err(err)
    }
}

fn get_selected_file_paths_by_clipboard_using_applescript(
) -> Result<String, Box<dyn std::error::Error>> {
    // debug_println!("get_selected_text_by_clipboard_using_applescript");
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(FILE_PATH_COPY_APPLE_SCRIPT)
        .output()?;
    if output.status.success() {
        let content = String::from_utf8(output.stdout)?;
        let content = content.trim();
        Ok(content.to_string())
    } else {
        let err = output
            .stderr
            .into_iter()
            .map(|c| c as char)
            .collect::<String>()
            .into();
        Err(err)
    }
}
