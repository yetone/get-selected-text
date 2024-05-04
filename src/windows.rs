use enigo::*;

pub fn get_selected_text() -> Result<String, Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    crate::utils::get_selected_text_by_clipboard(&mut enigo, false)
}
