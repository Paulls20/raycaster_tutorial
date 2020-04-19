mod settings;
mod frames;
mod image;
mod utility;
mod player;

use crate::settings::{Windows, Map};
use crate::frames::FrameBuffer;
use crate::image::{PPMImage, Image};
use crate::player::Player;

fn main() {
    let w: Windows = Default::default();
    let m: Map = Default::default();

    let mut fb = FrameBuffer::new(&w, &m);
    fb.fill_gradient();
    fb.draw_map();

    let p = Player::new(3.456, 2.345);
    fb.draw_player(&p);

    PPMImage::draw_image("test.ppm", &fb, &w);
}
