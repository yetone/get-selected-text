get-selected-text
=================

A tiny Rust library that allows you to easily obtain selected text across all platforms (macOS, Windows, Linux).

## Usage

### Add to Cargo.toml:

```toml
[dependencies]
get-selected-text = { git = "https://github.com/yetone/get-selected-text", branch = "main" }
```

### Use:

```rust
use get_selected_text::get_selected_text;

fn main() {
    match get_selected_text() {
        Ok(selected_text) => {
            println!("selected text: {}", selected_text);
        },
        Err(()) => {
            println!("error occurred while getting the selected text");
        }
    }
}
```

## How does it work?

### macOS

Prioritize using the A11y API to obtain selected text. If the application does not comply with the A11y API, simulate pressing cmd+c to borrow from the clipboard to get the selected text.

To avoid annoying alert sounds when simulating pressing cmd+c, it will automatically mute the alert sound. The volume of the alert sound will be restored after releasing the key.

Therefore, on macOS, you need to grant accessbility permissions in advance. The sample code is as follows:

```rust
fn query_accessibility_permissions() -> bool {
    let trusted = macos_accessibility_client::accessibility::application_is_trusted_with_prompt();
    if trusted {
        print!("Application is totally trusted!");
    } else {
        print!("Application isn't trusted :(");
    }
    trusted
}
```

### Windows + Linux

Simulate pressing ctrl+c to use the clipboard to obtain the selected text.
