mod settings;
mod frames;
mod image;
mod utility;
mod player;

mod texture;

use crate::settings::{Windows, Map};
use crate::frames::FrameBuffer;
use crate::image::{PPMImage, Image};
use crate::player::Player;
use crate::utility::Color;

fn run() {
    let w: Windows = Default::default();
    let m: Map = Default::default();
    let colors = Color::generate_random(10);

    let mut fb = FrameBuffer::new(&w, &m, &colors);
    fb.draw_map();
    fb.draw_texture();

    let p = Player::new(3.456, 2.345, 1.523);
    fb.draw_player(&p);
    PPMImage::draw_image(format!("frame.ppm").as_str(), &fb, &w);
}

fn main() {
    run();
}
