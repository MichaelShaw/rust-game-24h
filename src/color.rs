

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub type ColorRaw = [u8; 4];
pub type ColorFloatRaw = [f32; 4];

pub fn rgba(r:u8, g:u8, b:u8, a: u8) -> Color {
    Color { r:r, g:g, b:b, a:a }
}

pub fn rgb(r:u8, g:u8, b:u8) -> Color {
    Color { r:r, g:g, b:b, a: 255}
}

impl Color {
    pub fn rf(&self) -> f32 {
        (self.r as f32) / 255.0
    }

    pub fn gf(&self) -> f32 {
        (self.g as f32) / 255.0
    }

    pub fn bf(&self) -> f32 {
        (self.b as f32) / 255.0
    }

    pub fn af(&self) -> f32 {
        (self.a as f32) / 255.0
    }

    pub fn raw(&self) -> ColorRaw {
        [self.r, self.g, self.b, self.a]
    }

    pub fn float_raw(&self) -> ColorFloatRaw {
        [self.rf(), self.gf(), self.bf(), self.af()]
    }

    pub fn tup(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn float_tup(&self) -> (f32, f32, f32, f32) {
        (self.rf(), self.gf(), self.bf(), self.af())
    }
}