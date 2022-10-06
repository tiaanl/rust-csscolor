pub struct LCH {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
}

impl LCH {
    pub fn new(lightness: f32, chroma: f32, hue: f32) -> Self {
        Self {
            lightness,
            chroma,
            hue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let lab = LCH::new(20.0, 0.0, 10.0);
        assert_eq!(lab.lightness, 20.0);
        assert_eq!(lab.chroma, 0.0);
        assert_eq!(lab.hue, 10.0);
    }
}
