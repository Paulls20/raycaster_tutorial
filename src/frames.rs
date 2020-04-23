use crate::settings::{Windows, Map};
use crate::utility::Color;
use std::ops::Index;
use crate::player::Player;

pub struct FrameBuffer<'a> {
    pub buffer: Vec<u32>,
    windows: &'a Windows,
    map: &'a Map,
    rect_w: usize,
    rect_h: usize,
}

struct Rectangle {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(w: &'a Windows, m: &'a Map) -> Self {
        FrameBuffer {
            buffer: vec![255; (w.size()) as usize],
            windows: w,
            map: m,
            rect_w: w.width / m.width,
            rect_h: w.height / m.height,
        }
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
                self.buffer[(i + j * self.windows.width) as usize] = Color::new(r, g, b, 255).pack();
            }
        }
    }

    fn draw_rectangle(&mut self, rectangle: Rectangle, color: u32) {
        for i in 0..rectangle.w {
            for j in 0..rectangle.h {
                let cx = rectangle.x + i;
                let cy = rectangle.y + j;
                self.buffer[cx + cy * self.windows.width] = color
            }
        }
    }

    pub fn draw_map(&mut self) {
        for j in 0..self.map.height {
            for i in 0..self.map.width {
                if char::from(self.map[i + j * self.map.width]) == ' ' {
                    continue;
                }
                let rect_x = i * self.rect_w;
                let rect_y = j * self.rect_h;
                let r = Rectangle { x: rect_x, y: rect_y, w: self.rect_w, h: self.rect_h };
                self.draw_rectangle(r, Color::new(0, 255, 255, 255).pack())
            }
        }
    }

    pub fn draw_player(&mut self, player: &Player) {
        let x_pos = player.x * self.rect_w as f32;
        let y_pos = player.y * self.rect_h as f32;
        let r = Rectangle { x: x_pos as usize, y: y_pos as usize, w: 5, h: 5 };
        self.draw_rectangle(r, Color::new(255, 255, 255, 255).pack());
        self.draw_field_of_view(player);
    }

    fn draw_field_of_view(&mut self, player: &Player) {
        const FOV: f32 = std::f32::consts::PI / 3f32;
        for i in 0..self.windows.width {
            let angle1 = player.a - FOV / 2f32;
            let angle = angle1 + FOV * i as f32 / self.windows.width as f32;
            let mut t = 0f32;
            while t < 20f32 {
                let cx = player.x + t * angle.cos();
                let cy = player.y + t * angle.sin();
                let index = cx as usize + cy as usize * self.map.width;
                if char::from(self.map[index]) != ' ' {
                    break;
                }

                let pix_x = cx * self.rect_w as f32;
                let pix_y = cy * self.rect_h as f32;
                let index = pix_x as usize + pix_y as usize * self.windows.width;
                println!("Index = {}", index);
                self.buffer[index] = Color::new(255, 255, 255, 255).pack();
                t += 0.05f32;
            }
        }
    }
}

impl<'a> Index<usize> for FrameBuffer<'a> {
    type Output = u32;
    fn index(&self, i: usize) -> &u32 {
        &self.buffer[i]
    }
}