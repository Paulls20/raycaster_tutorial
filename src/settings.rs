#[derive(Clone)]
pub struct Windows {
    pub width: usize,
    pub height: usize,
}

impl Default for Windows {
    fn default() -> Self {
        Windows { width: 512, height: 512 }
    }
}

impl Windows {
    pub fn new(width: usize, height: usize) -> Self {
        Windows {width, height}
    }
    pub fn size(&self) -> usize {
        self.width * self.height
    }
}