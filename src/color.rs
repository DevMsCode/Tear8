#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

pub const DARK_BLUE: Color = Color {
    r: 41,
    g: 50,
    b: 64,
    a: 255,
};

pub const BROWN: Color = Color {
    r: 127,
    g: 83,
    b: 75,
    a: 255,
};

pub const WHITE: Color = Color {
    r: 243,
    g: 250,
    b: 225,
    a: 255,
};

pub const RED: Color = Color {
    r: 223,
    g: 67,
    b: 77,
    a: 255,
};

pub const ORANGE: Color = Color {
    r: 255,
    g: 106,
    b: 77,
    a: 255,
};

pub const YELLOW: Color = Color {
    r: 243,
    g: 182,
    b: 31,
    a: 255,
};

pub const GREEN: Color = Color {
    r: 157,
    g: 181,
    b: 63,
    a: 255,
};

pub const BLUE: Color = Color {
    r: 110,
    g: 155,
    b: 211,
    a: 255,
};

pub const PINK: Color = Color {
    r: 194,
    g: 87,
    b: 130,
    a: 255,
};

pub const VIOLET: Color = Color {
    r: 145,
    g: 65,
    b: 130,
    a: 255,
};