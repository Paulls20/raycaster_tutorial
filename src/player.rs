pub struct Player {
    pub x: f32,
    pub y: f32,
    pub a: f32
}

impl Player {
    pub fn new(x: f32, y: f32, a: f32) -> Self {
        Player { x, y, a }
    }
}
