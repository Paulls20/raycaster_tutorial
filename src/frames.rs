use crate::settings::Windows;
use crate::utility::Color;
use std::ops::Index;

pub struct FrameBuffer {
    pub buffer: Vec<u32>,
    windows: Windows,
}

impl FrameBuffer {
    pub fn new(w: Windows) -> Self {
        FrameBuffer { buffer: vec![255; (w.size()) as usize], windows: w }
    }

    pub fn print(&self) {
        for (index, value) in self.buffer.iter().enumerate() {
            println!("buffer[{}] = {}", index, value)
        }
    }

    pub fn fill_gradient(&mut self) {
        for j in 0..self.windows.height {
            for i in 0..self.windows.width {
                let r = ((255 * j) / self.windows.height) as u8;
                let g = ((255 * i) / self.windows.width) as u8;
                let b = 0 as u8;
                self.buffer[(i + j * self.windows.width) as usize] = Color::new(r, g, b).pack(255);
            }
        }
    }
}

impl Index<usize> for FrameBuffer {
    type Output = u32;
    fn index(&self, i: usize) -> &u32 {
        &self.buffer[i]
    }
}