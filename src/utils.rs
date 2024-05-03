use enigo::*;
use parking_lot::Mutex;

static INPUT_LOCK_LOCKER: Mutex<()> = Mutex::new(());

pub(crate) fn right_arrow_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK_LOCKER.lock();

    for _ in 0..n {
        enigo.key_click(Key::RightArrow);
    }
}

pub(crate) fn up_control_keys(enigo: &mut Enigo) {
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Alt);
    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Space);
    enigo.key_up(Key::Tab);
}
