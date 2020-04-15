use crate::frames::FrameBuffer;
use crate::settings::Windows;
use crate::utility::Color;
use std::fs::File;
use std::io::Write;

pub struct PPMImage;

pub trait Image {
    fn draw_image(filename: &str, framebuffer: &FrameBuffer, windows: &Windows);
}

impl Image for PPMImage {
    fn draw_image(filename: &str, image: &FrameBuffer, windows: &Windows)
    {
        let mut pixel_array = vec![format!("P3\n{} {}\n255", windows.width, windows.height)];
        for i in 0..windows.size() {
            let Color {r, g, b, a} = Color::unpack(image[i]);
            println!("r: {}, g: {}, b: {}, a: {}", r, g, b, a);
            pixel_array.push(format!("{} {} {}", r, g, b));
        }
        let mut file = File::create(filename).unwrap();
        let output = pixel_array.join("\n");
        file.write_all(output.as_bytes());
    }
}

