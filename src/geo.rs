use std::ops::Sub;
use std::convert::Into;

// Punto 
#[derive(Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> 
where
    T: Sub<Output = T> + Into<f64> + Copy
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // Calcola la distanza tra questo punto e un altro punto
    pub fn distance(&self, other: &Point<T>) -> f64
    {
        let dx = (self.x.into() - other.x.into()) as f64;
        let dy = (self.y.into() - other.y.into()) as f64;
        ((dx * dx + dy * dy) as f64).sqrt()
    }
}

// Linea
pub struct Line {
    pub start: Point<i32>,
    pub end: Point<i32>,
}

impl Line {
    pub fn new(start: Point<i32>, end: Point<i32>) -> Self {
        Self { start, end }
    }

    // Calcola la lunghezza della linea
    pub fn length(&self) -> f64 {
        self.start.distance(&self.end)
    }
}

// Rettangolo
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x, y,
            width, 
            height
        }
    }

    pub fn new_square_rect(x: i32, y: i32, size: i32) -> Self {
        Self {
            x, y, 
            width: size, 
            height: size
        }
    }

    // Restituisce i limiti del rettangolo
    pub fn get_bounds(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.x + self.width, self.y + self.height)
    }

    // Controlla se questo rettangolo interseca un altro rettangolo
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }

    pub fn scale(&mut self, scale: f32) {
        self.width = (self.width as f32 * scale) as i32;
        self.height = (self.height as f32 * scale) as i32;
    }

    pub fn clip_into_the_buffer(&mut self, buffer_size: Point<u32>) {
        if self.x < 0 {
            self.x = 0;
        }
        if self.y < 0 {
            self.y = 0;
        }
        if self.x + self.width > buffer_size.x as i32{
            self.x = buffer_size.x as i32 - self.width;
        }
        if self.y + self.height > buffer_size.y as i32 {
            self.y = buffer_size.y as i32 - self.height;
        }
    }

    pub fn is_into_the_buffer(&self, buffer_size: Point<u32>) -> bool {
        self.x >= 0 && self.y >= 0 &&
        self.x + self.width <= buffer_size.x as i32 &&
        self.y + self.height <= buffer_size.y as i32
    }
}