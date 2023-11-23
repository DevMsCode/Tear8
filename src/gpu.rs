use std::slice::ChunksMut;

use crate::{logger::Logger, color::Color, geo::*};

pub enum Backend {
    Primary,
    Secondary,
    Vulkan,
    Metal,
    OpenGL,
    DirectX11,
    DirectX12,
    WebGPU,
}


pub struct Gpu {
    pub buffer_width: u32,
    pub buffer_height: u32,
    logger: Logger,
    disable_logger: bool,
}

impl Gpu {
    pub fn new(buffer_size: Point<u32>, logger: &Logger) -> Self {
        Self { 
            buffer_width: buffer_size.x,
            buffer_height: buffer_size.y, 
            logger: logger.clone(),
            disable_logger: false,
        }
    }

    pub fn disable_logger(&mut self) {
        self.disable_logger = true;
    }
    
    // Cambia il colore allo schermo
    pub fn render_color(&mut self, buffer: &mut [u8], color: Color) {
        for pixel in self.get_px(buffer) {
            pixel[0] = color.r; // R
            pixel[1] = color.g; // G
            pixel[2] = color.b; // B
            pixel[3] = color.a; // A
        }
    }

    // Get pixel from buffer
    pub fn get_px<'a>(&'a mut self, buffer: &'a mut [u8]) -> ChunksMut<'_, u8> {
        buffer.chunks_mut(4)
    }

    // Disegna un pixel a schermo
    pub fn render_pixel(&mut self, buffer: &mut [u8], pos: &Point<i32>, color: Color) {
        let x = pos.x as u64;
        let y = pos.y as u64;
        let width = self.buffer_width as u64;
        let height = self.buffer_height as u64;

        if x >= width || y >= height {
            if !self.disable_logger {
                self.logger.warning("GPU; DRAW PIXEL", "X and Y are too large, so they exceed the size of buffer. Try to reduce them.");
            }
            return;
        }

        // Crea l'index per scegliere il pixel in cui colorare
        let index = (y * width + x) * 4;
        let buffer_len = buffer.len() as u64;
        let pixels_color = [color.r, color.g, color.b, color.a];
        
        // Controlla se l'index è maggiore alla lunghezza del buffer
        if index >= buffer_len {
            if !self.disable_logger{
                self.logger.warning("GPU; DRAW PIXEL", "X and Y are too large, so they exceed the size of buffer. Try to reduce them.");
            }
            return;
        }

        buffer[index as usize..index as usize + 4].copy_from_slice(&pixels_color);
    }

    // Disegna un rettangolo a schermo
    pub fn render_rect(&mut self, buffer: &mut [u8], rect: &mut Rect, color: Color, clip: bool) {
        let buffer_size = Point::new(self.buffer_width, self.buffer_height);
        // Regola le dimensioni e la posizione del rettangolo
        if clip {
            rect.clip_into_the_buffer(buffer_size.clone());
        }

        // Controlla se il rettangolo è all'interno dei limiti dello schermo
        if !rect.is_into_the_buffer(buffer_size.clone()) {
            if !self.disable_logger {
                self.logger.error("GPU; DRAW RECT", "The rectangle is out of bounds. Try to reduce its size or change its position.");
            }
            return;
        }
        
        for x in rect.x..rect.x + rect.width  {
            for y in rect.y..rect.y + rect.height {
                self.render_pixel(buffer, &Point::new(x, y), color);
            }
        }
    }

    // Disegna una linea a schermo
    pub fn render_line(&mut self, buffer: &mut [u8], line: &Line, color: Color) {
        // Init vars
        let x1 = line.start.x;
        let y1 = line.start.y;
        let x2 = line.end.x;
        let y2 = line.end.y;

        // Calculate line
        let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        let mut x = x1;
        let mut y = y1;

        loop {
            // Controlla se esso è dentro lo schermo
            if x >= 0 && x < self.buffer_width as i32 && y >= 0 && y < self.buffer_height as i32 {
                self.render_pixel(buffer, &Point::new(x, y), color);
            }
    
            if x == x2 && y == y2 { break; }

            err2 = err;

            if err2 > -dx { err -= dy; x += sx; }
            if err2 < dy { err += dx; y += sy; }
        }
    }

    // Disegna un buffer a schermo 
    pub fn render_buffer(&self, main_buffer: &mut [u8], buffer: &mut [u8]) {
        main_buffer.copy_from_slice(buffer);
    }

}
