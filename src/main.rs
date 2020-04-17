mod settings;
mod frames;
mod image;
mod utility;

use crate::settings::{Windows, Map};
use crate::frames::FrameBuffer;
use crate::image::{PPMImage, Image};

fn main() {
    let w: Windows = Default::default();

    let mut fb = FrameBuffer::new(w.clone());
    fb.fill_gradient();

    let m: Map = Default::default();
    fb.draw_map(m);

    PPMImage::draw_image("test.ppm", &fb, &w);
}
