// Winit
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

// Crate
use crate::{ logger::Logger, time::Time };

// Pixels
use pixels::Pixels;
use pixels::SurfaceTexture;

pub struct TearContext {
    pub(crate) logger: Logger,
}

impl TearContext {
    pub fn new() -> Self {
        let time = Time::new();
        let logger = Logger::new(time);

        Self {
            logger,
        }
    }

    pub(crate) fn init_pixels(&mut self, window: &Window, width: u32, height: u32) -> Pixels {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        match Pixels::new(width, height, surface_texture) {
            Ok(p) => p,
            Err(err) => {
                self.logger.error("PIXELS BUFFER CREATOR", err.to_string().as_str());
                panic!()
            }
        }
    }

    pub(crate) fn init_window(&mut self, title: &str, width: u32, height: u32, event_loop: &EventLoop<()>) -> Window {
        let size = LogicalSize::new(width as f64, height as f64);
        let win_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size);

        // Return window
        win_builder
            .build(event_loop)
            .unwrap()
    }
}