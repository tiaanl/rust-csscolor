pub struct Lab {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
}

impl Lab {
    pub fn new(lightness: f32, a: f32, b: f32) -> Self {
        Self { lightness, a, b }
    }
}
