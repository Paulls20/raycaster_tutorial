mod settings;
mod frames;
mod image;
mod utility;

use crate::settings::Windows;
use crate::frames::FrameBuffer;
use crate::image::{PPMImage, Image};

fn main() {
    let w: Windows = Default::default();

    let mut fb = FrameBuffer::new(w.clone());
    fb.fill_gradient();

    PPMImage::draw_image("test.ppm", &fb, &w);
}
