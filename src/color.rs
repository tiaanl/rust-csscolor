use crate::ColorSpace;

/// The three color components that represent a color.
pub struct ColorComponents(pub f32, pub f32, pub f32);

/// An absolutely specified color.
pub struct Color {
    components: ColorComponents,
    alpha: f32,
    color_space: ColorSpace,
}

impl Color {
    pub fn new(color_space: ColorSpace, c1: f32, c2: f32, c3: f32) -> Self {
        Self {
            components: ColorComponents(c1, c2, c3),
            alpha: 1.0,
            color_space,
        }
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Return the components and alpha of the color as an array of floats.
    pub fn as_components(&self) -> &[f32; 4] {
        unsafe { std::mem::transmute(&self.components) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_components() {
        let color = Color::new(ColorSpace::Srgb, 0.1, 0.2, 0.3).with_alpha(0.4);
        assert_eq!(color.as_components(), &[0.1, 0.2, 0.3, 0.4]);
    }
}
