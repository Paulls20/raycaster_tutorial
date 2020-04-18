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

    let mut fb = FrameBuffer::new(w.clone());
    fb.fill_gradient();

    let m: Map = Default::default();
    fb.draw_map(&m);

    let p = Player::new(3.456, 2.345);
    fb.draw_player(&m, &p);

    PPMImage::draw_image("test.ppm", &fb, &w);
}
