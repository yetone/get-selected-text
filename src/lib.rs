#[cfg(not(target_os = "macos"))]
mod utils;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
use crate::linux::get_selected_text as _get_selected_text;
#[cfg(target_os = "macos")]
use crate::macos::get_selected_text as _get_selected_text;
#[cfg(target_os = "windows")]
use crate::windows::get_selected_text as _get_selected_text;

/// # Example
///
/// ```
/// use get_selected_text::get_selected_text;
///
/// let text = get_selected_text().unwrap();
/// println!("{}", text);
/// ```
#[cfg(not(target_os = "macos"))]
pub fn get_selected_text() -> Result<String, Box<dyn std::error::Error>> {
    _get_selected_text()
}

#[cfg(target_os = "macos")]
pub fn get_selected_text() -> Result<SelectedText, Box<dyn std::error::Error>> {
    _get_selected_text()
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SelectedText {
    is_file_paths: bool,
    app_name: String,
    text: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_selected_text() {
        println!("--- get_selected_text ---");
        let text = get_selected_text().unwrap();
        println!("selected text: {:#?}", text);
        println!("--- get_selected_text ---");
        let text = get_selected_text().unwrap();
        println!("selected text: {:#?}", text);
        println!("--- get_selected_text ---");
        let text = get_selected_text().unwrap();
        println!("selected text: {:#?}", text);
    }
}
