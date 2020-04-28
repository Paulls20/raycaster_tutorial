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

fn run() {
    let w: Windows = Default::default();
    let m: Map = Default::default();

    let mut fb = FrameBuffer::new(&w, &m);
    fb.draw_map();
    let p = Player::new(3.456, 2.345, 1.523);
    fb.draw_player(&p);
    PPMImage::draw_image(format!("frame.ppm").as_str(), &fb, &w);
}

fn main() {
    run();
}
