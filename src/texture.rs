use stb_image::image::LoadResult;
use crate::utility::Color;
use std::ops::Index;

pub struct Texture {
    pub wall: Vec<u32>,
    pub size: usize,
    pub cnt: usize,
}

pub fn load_texture(filename: &str) -> Result<Texture, &'static str> {
    match stb_image::image::load(filename) {
        LoadResult::Error(_s) => {
            Err("Error loading texture file")
        }
        LoadResult::ImageF32(_) => {
            Err("The texture must be a 32 bit image")
        }
        LoadResult::ImageU8(image) => {
            let mut texture = Texture {
                wall: vec![0u32; image.width * image.height],
                size: image.width / (image.width / image.height),
                cnt: image.width / image.height,
            };

            for j in 0..image.height {
                for i in 0..image.width {
                    let r = image.data[(i + j * image.width) * 4 + 0] as u8;
                    let g = image.data[(i + j * image.width) * 4 + 1] as u8;
                    let b = image.data[(i + j * image.width) * 4 + 2] as u8;
                    let a = image.data[(i + j * image.width) * 4 + 3] as u8;

                    texture.wall[i + j * image.width] = Color::new(r, g, b, a).pack();
                }
            }

            Ok(texture)
        }
    }
}

impl Index<usize> for Texture {
    type Output = u32;
    fn index(&self, i: usize) -> &u32 {
        &self.wall[i]
    }
}

#[cfg(test)]
mod test {
    use crate::texture::load_texture;

    #[test]
    fn test_texture() {
        let texture = load_texture("/home/paul/Workspace/tiny_raycaster/resource/walltext.png").unwrap();
        assert_eq!(texture.size, 64)
    }
}

