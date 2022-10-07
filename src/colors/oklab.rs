use crate::{FromColor, Oklch};

/// Lightness, a, b color format. (Oklab)
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

impl FromColor<Oklch> for Oklab {
    fn from_color(from: Oklch) -> Self {
        let (l, a, b) = super::lch_to_lab(from.lightness, from.chroma, from.hue);
        Self::new(l, a, b)
    }
}
