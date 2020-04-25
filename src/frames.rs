use crate::settings::{Windows, Map};
use crate::utility::{Color, Position};
use std::ops::Index;
use crate::player::Player;

pub struct FrameBuffer<'a> {
    pub buffer: Vec<u32>,
    windows: &'a Windows,
    map: &'a Map,
    rect_w: usize,
    rect_h: usize,
    colors: &'a Vec<u32>,
}

struct Rectangle {
    pos: Position,
    w: usize,
    h: usize,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(w: &'a Windows, m: &'a Map, c: &'a Vec<u32>) -> Self {
        FrameBuffer {
            buffer: vec![Color::new(255, 255, 255, 255).pack(); (w.size()) as usize],
            windows: w,
            map: m,
            rect_w: w.width / (m.width * 2),
            rect_h: w.height / m.height,
            colors: c,
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for (index, value) in self.buffer.iter().enumerate() {
            println!("buffer[{}] = {}", index, value)
        }
    }

    #[allow(dead_code)]
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
                let cx = rectangle.pos.x + i;
                let cy = rectangle.pos.y + j;
                if cx >= self.windows.width || cy >= self.windows.height {
                    continue;
                }
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
                let r = Rectangle {
                    pos: Position { x: rect_x, y: rect_y },
                    w: self.rect_w,
                    h: self.rect_h,
                };
                let icolor = char::from(self.map[i + j * self.map.width]) as usize - '0' as usize;
                self.draw_rectangle(r, self.colors[icolor]);
            }
        }
    }

    pub fn draw_player(&mut self, player: &Player) {
        self.draw_field_of_view(player);
    }

    fn draw_field_of_view(&mut self, player: &Player) {
        const FOV: f32 = std::f32::consts::PI / 3f32;
        for i in 0..self.windows.width / 2 {
            let angle = player.a - FOV / 2f32 + FOV * i as f32 / (self.windows.width / 2) as f32;
            let mut t = 0f32;
            while t < 20f32 {
                let cx = player.x + t * angle.cos();
                let cy = player.y + t * angle.sin();

                let pix_x = cx * self.rect_w as f32;
                let pix_y = cy * self.rect_h as f32;
                let index = pix_x as usize + pix_y as usize * self.windows.width;
                self.buffer[index] = Color::new(160, 160, 160, 255).pack();

                let index = cx as usize + cy as usize * self.map.width;
                if char::from(self.map[index]) != ' ' {
                    let col_height = self.windows.height as f32 / t;
                    let r = Rectangle {
                        pos: Position { x: self.windows.width / 2 + i, y: self.windows.height / 2 - col_height as usize / 2 },
                        w: 1,
                        h: col_height as usize,
                    };
                    let icolor = char::from(self.map[cx as usize + cy as usize * self.map.width]) as usize - '0' as usize;
                    self.draw_rectangle(r, self.colors[icolor]);
                    break;
                }
                t += 0.01f32;
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