use crate::{FromColor, Lch};

/// Lightness, a, b color format.
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

impl FromColor<Lch> for Lab {
    fn from_color(from: Lch) -> Self {
        let (l, a, b) = super::lch_to_lab(from.lightness, from.chroma, from.hue);
        Self::new(l, a, b)
    }
}
