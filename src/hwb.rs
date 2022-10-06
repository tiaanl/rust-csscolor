pub struct Hwb {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
}

impl Hwb {
    pub fn new(hue: f32, whiteness: f32, blackness: f32) -> Self {
        Self {
            hue,
            whiteness,
            blackness,
        }
    }
}
