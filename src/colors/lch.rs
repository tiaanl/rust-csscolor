pub struct Lch {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
}

impl Lch {
    pub fn new(lightness: f32, chroma: f32, hue: f32) -> Self {
        Self {
            lightness,
            chroma,
            hue,
        }
    }
}
