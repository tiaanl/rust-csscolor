pub struct Hsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl Hsl {
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue,
            saturation,
            lightness,
        }
    }
}
