pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }
    pub fn pack(&self, a: u8) -> u32 {
        let mut c = (a as u32) << 24;
        c += (self.b as u32) << 16;
        c += (self.g as u32) << 8;
        c += self.r as u32;
        c
    }

    pub fn unpack(color: u32) -> Color {
        Color {
            r: ((color >> 0) & 255) as u8,
            g: ((color >> 8) & 255) as u8,
            b: ((color >> 16) & 255) as u8,
            a: ((color >> 24) & 255) as u8,
        }
    }
}