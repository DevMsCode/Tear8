use pixels::Pixels;
use winit::{event::Event, event_loop::{EventLoop, ControlFlow}, window::Window};
use crate::{ gpu::Gpu, input::Input, logger::Logger, context::TearContext, geo::*, state::State };

// Crea il device di vilp
pub struct Tear {
    gpu: Gpu,
    input: Input,
    logger: Logger,
    window: Window,
    event_loop: EventLoop<()>,
    pixels: Pixels,
}

impl Tear {
    // Fai in modo che l'utente possa ridimensionare la finestra
    pub fn new(title: &str, size: Point<u32>, buffer_size: Point<u32>) -> Self {
        let mut ctx = TearContext::new();
        // Init window
        let event_loop = EventLoop::new();
        let window = ctx.init_window(title, size.x, size.y, &event_loop);

        // Init gpu
        let pixels = ctx.init_pixels(&window, buffer_size.x, buffer_size.y);
        let gpu = Gpu::new(buffer_size, &ctx.logger);

        // Init window
        let input = Input::new();

        Self { 
            gpu,
            input,
            logger: ctx.logger,
            window,
            event_loop,
            pixels
        }
    }

    pub fn build<W: State + 'static>(mut self, mut world: W) {
        self.event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                world.draw(&mut self.gpu, self.pixels.frame_mut());
                if let Err(err) = self.pixels.render() {
                    self.logger.error("PIXELS.RENDER", err.to_string().as_str());
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Handle input events
            if self.input.winit_input.update(&event) {
                // Close events
                if self.input.winit_input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Resize the window
                if let Some(size) = self.input.winit_input.window_resized() {
                    if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                        self.logger.error("RESIZE PIXELS SURFACE", err.to_string().as_str());
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                // Update input
                world.input(&mut self.input);
                // Update internal state and request a redraw
                world.update();
                self.window.request_redraw();
            }
        });
    }
    
}