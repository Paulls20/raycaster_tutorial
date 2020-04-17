use crate::settings::{Windows, Map};
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

    #[allow(dead_code)]
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

    fn draw_rectangle(&mut self, w: usize, h: usize, x: usize, y: usize, color: u32) {
        for j in 0..w {
            for i in 0..h {
                let cx = x + i;
                let cy = y + j;
                self.buffer[cx + cy * self.windows.width] = color
            }
        }
    }

    pub fn draw_map(&mut self, map: Map) {
        let rect_w = self.windows.width / map.width;
        let rect_h = self.windows.height / map.height;
        for j in 0..map.height {
            for i in 0..map.width {
                if char::from(map[i + j * map.width]) == ' ' {
                    continue;
                }
                let rect_x = i * rect_w;
                let rect_y = j * rect_h;
                self.draw_rectangle(rect_w, rect_h, rect_x, rect_y, Color::new(0, 255, 255).pack(255))
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