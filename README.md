<p align="left">
  <img src="brand/logo.png" width="321" height="275,5">
</p>

# Tear
Tear is a Rust library that enables you to create your own fantasy console using winit and pixels.

## Quick Example ⚡

```rust
use tear8::*;

struct World;

fn main() {
    let world = World {};

    Tear::new(
        "Tear render",
        Point::new(300, 300), 
        Point::new(128, 128)
    ).build(world);
}

impl State for World {
    fn draw(&mut self, gpu: &mut Gpu, buffer: &mut [u8]) {
        
    }

    fn update(&mut self) {
        
    }

    fn input(&mut self, input: &mut Input) {
        
    }
}
```

## Attention ⚠
- **Unstable code:** The code cannot be fully optimized.
- **Missing functions:** You can draw pixels, rectangles and lines on the screen, but for now you cannot draw images, sounds and music.

## API
- ## Windowing:
  Use the "new()" function of the "Tear" structure to create a window, the first parameter is the window name, the second is the window size, and the third is buffer size. To   compile everything use the "build(world)" function and insert a structure that implements the trait state to compile everything:
  
  ```rust
  use tear8::*;

  // Create struct world (main tear8 app loop)
  struct World;
  
  fn main() {
      // Init world
      let world = World {};
  
      // Create tear and build window
      // with Pixels and Winit
      Tear::new(
          "Tear render",
          // Point structure is a structure that contains
          // the parameters x and y which in this case are
          // the size of the window
          Point::new(300, 300), 
          Point::new(128, 128)
      ).build(world); // Build window
  }
  
  impl State for World {
      // Draw fn
      // for update draw in the game
      fn draw(&mut self, gpu: &mut Gpu, buffer: &mut [u8]) {
          
      }

      // Update fn
      // for update the game
      fn update(&mut self) {
          
      }
  
      // Input fn
      // for input
      fn input(&mut self, input: &mut Input) {
          
      }
  }
  ```
- ## Rendering
  - ## How to set background color
    Go to the "draw" function of your structure that implements the "state" trait like we did before and write this:
    Still in the draw function write this:
    ```rust
    // Set background color
    gpu.render_color(buffer, DARK_BLUE);
    ```
    The first parameter indicates where to set the background color, in the second you must enter the color.
    
  - ## How to draw a pixel on the screen:
    Still in the draw function write this:
    ```rust
    // Render pixel
    gpu.render_pixel(buffer, &Point::new(0, 0), WHITE);
    ```
    The first parameter indicates where to draw the pixel, in the second you have to enter the position and in the last there is the color.
  
  - ## How to draw a line on the screen:
    Write:
    ```rust
    // Render line
    gpu.render_line(
        buffer, 
        // Line struct
        &Line::new(
            // Start line point
            Point::new(0, 0), 
            // End line point
            Point::new(10, 10)
        ), 
        WHITE
    );
    ```
    As always, the first parameter indicates the buffer where the line must be drawn, the second must include the structure of the line and the last must include its color.

  - ## How to draw a rectangle on the screen:
    Write:
    ```rust
      // Render rect
      gpu.render_rect(
          buffer, 
          &mut Rect::new(0, 0, 100, 100), 
          RED, 
          // Don't let it go off the screen
          true
      );
    ```
    The first parameter indicates where to draw the rectangle, in the second you must insert the rect structure which has x, y and width, height as parameters, while in the third parameter there is the color and in the last you choose whether or not to make it exit the screen
  
  - ## How to draw a other buffer on the screen:
    Write:
    ```rust
      // Draw buffer in the main buffer
      gpu.render_buffer(main_buffer, other_buffer);
    ```
    In the first parameter there is the main buffer and in the second the buffer that must be drawn
- ## Coming soon...
