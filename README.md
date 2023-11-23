# Tear - Rust Fantasy Console Library
Tear is a Rust library that enables you to create your own fantasy console using winit and pixels.

## Features

- **Customizable Fantasy Console:** Build your game development environment inspired by vintage consoles.
- **Pixel Graphics:** Use pixels to easily manipulate pixels on the screen.
- **Easy Integration:** Seamlessly integrate the library into your Rust applications.

## Quick Example

```rust

use tear::*;

struct World;

fn main() {
    let world = World {};

    Tear::new(
        "Tear render",
        Point::new(300, 300), 
        Point::new(128, 128)
    ).build(world);
}
