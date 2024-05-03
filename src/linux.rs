use enigo::*;
use parking_lot::Mutex;
use std::{thread, time::Duration};

static COPY_PASTE_LOCKER: Mutex<()> = Mutex::new(());

fn copy(enigo: &mut Enigo) {
    let _guard = COPY_PASTE_LOCKER.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    thread::sleep(Duration::from_millis(50));
    enigo.key_click(Key::Layout('c'));
    thread::sleep(Duration::from_millis(50));
    enigo.key_up(Key::Control);
}

fn paste(enigo: &mut Enigo) {
    let _guard = COPY_PASTE_LOCKER.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
}

pub fn get_selected_text() -> Result<String, Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new();
    get_selected_text_by_clipboard(&mut enigo, false)
}

fn get_selected_text_by_clipboard(
    enigo: &mut Enigo,
    cancel_select: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    use arboard::Clipboard;

    let old_clipboard = (Clipboard::new()?.get_text(), Clipboard::new()?.get_image());

    let mut write_clipboard = Clipboard::new()?;

    let not_selected_placeholder = "";

    write_clipboard.set_text(not_selected_placeholder)?;

    thread::sleep(Duration::from_millis(50));

    copy(enigo);

    if cancel_select {
        crate::utils::right_arrow_click(enigo, 1);
    }

    thread::sleep(Duration::from_millis(100));

    let new_text = Clipboard::new()?.get_text();

    match old_clipboard {
        (Ok(old_text), _) => {
            // Old Content is Text
            write_clipboard.set_text(old_text.clone())?;
            if let Ok(new) = new_text {
                if new.trim() == not_selected_placeholder.trim() {
                    Ok(String::new())
                } else {
                    Ok(new)
                }
            } else {
                Ok(String::new())
            }
        }
        (_, Ok(image)) => {
            // Old Content is Image
            write_clipboard.set_image(image)?;
            if let Ok(new) = new_text {
                if new.trim() == not_selected_placeholder.trim() {
                    Ok(String::new())
                } else {
                    Ok(new)
                }
            } else {
                Ok(String::new())
            }
        }
        _ => {
            // Old Content is Empty
            write_clipboard.clear()?;
            if let Ok(new) = new_text {
                if new.trim() == not_selected_placeholder.trim() {
                    Ok(String::new())
                } else {
                    Ok(new)
                }
            } else {
                Ok(String::new())
            }
        }
    }
}
