# Tear - Rust Fantasy Console Library
Tear is a Rust library that enables you to create your own fantasy console using winit and pixels.

## Features

- **Customizable Fantasy Console:** Build your game development environment inspired by vintage consoles.
- **Pixel Graphics:** Use pixels to easily manipulate pixels on the screen.
- **Easy Integration:** Seamlessly integrate the library into your Rust applications.

## Quick Example

```rust
use tear::{Tear, Color};

fn main() {
    // Initialize Tear
    let mut tear = Tear::new("My Fantasy Console", (640, 480));

    // Main loop
    while let Some(frame) = tear.next_frame() {
        // Draw something on the frame
        frame.render_color(Color::new(255, 0, 0, 255)); // Example: draw a red background
    }
}
