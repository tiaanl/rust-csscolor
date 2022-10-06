pub struct Oklab {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
}

impl Oklab {
    pub fn new(lightness: f32, a: f32, b: f32) -> Self {
        Self { lightness, a, b }
    }
}
