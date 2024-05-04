use enigo::*;
use parking_lot::Mutex;
use std::{thread, time::Duration};

static COPY_PASTE_LOCKER: Mutex<()> = Mutex::new(());
static INPUT_LOCK_LOCKER: Mutex<()> = Mutex::new(());

pub(crate) fn right_arrow_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK_LOCKER.lock();

    for _ in 0..n {
        enigo.key(Key::RightArrow, Direction::Click).unwrap();
    }
}

pub(crate) fn up_control_keys(enigo: &mut Enigo) {
    enigo.key(Key::Control, Direction::Release).unwrap();
    enigo.key(Key::Alt, Direction::Release).unwrap();
    enigo.key(Key::Shift, Direction::Release).unwrap();
    enigo.key(Key::Space, Direction::Release).unwrap();
    enigo.key(Key::Tab, Direction::Release).unwrap();
}

pub(crate) fn copy(enigo: &mut Enigo) {
    let _guard = COPY_PASTE_LOCKER.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key(Key::Control, Direction::Press).unwrap();
    #[cfg(target_os = "windows")]
    enigo.key(Key::C, Direction::Click).unwrap();
    #[cfg(target_os = "linux")]
    enigo.key(Key::Unicode('c'), Direction::Click).unwrap();
    enigo.key(Key::Control, Direction::Release).unwrap();
}

pub(crate) fn get_selected_text_by_clipboard(
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
