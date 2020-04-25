mod settings;
mod frames;
mod image;
mod utility;
mod player;

use crate::settings::{Windows, Map};
use crate::frames::FrameBuffer;
use crate::image::{PPMImage, Image};
use crate::player::Player;
use crate::utility::Color;

fn run() {
    let w: Windows = Default::default();
    let m: Map = Default::default();
    let mut angle = 0f32;
    let colors = Color::generate_random(10);
    for i in 350..360 {
        let mut fb = FrameBuffer::new(&w, &m, &colors);
        fb.draw_map();
        angle += 2f32 * std::f32::consts::PI / 360f32;
        let p = Player::new(3.456, 2.345, angle);
        fb.draw_player(&p);
        PPMImage::draw_image(format!("{}_frame.ppm", i).as_str(), &fb, &w);
    }
}

fn main() {
    run();
}
