use std::ops::Index;

#[derive(Clone)]
pub struct Windows {
    pub width: usize,
    pub height: usize,
}

impl Default for Windows {
    fn default() -> Self {
        Windows { width: 1024, height: 512 }
    }
}

impl Windows {
    pub fn size(&self) -> usize {
        self.width * self.height
    }
}

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    path: &'static str,
}

impl Default for Map {
    fn default() -> Self
    {
        Map {
            width: 16,
            height: 16,
            path: "0000222222220000\
                   1              0\
                   1      11111   0\
                   1     0        0\
                   0     0  1110000\
                   0     3        0\
                   0   10000      0\
                   0   0   11100  0\
                   0   0   0      0\
                   0   0   1  00000\
                   0       1      0\
                   2       1      0\
                   0       0      0\
                   0 0000000      0\
                   0              0\
                   0002222222200000"
        }
    }
}

impl Index<usize> for Map {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.path.as_bytes()[i]
    }
}

