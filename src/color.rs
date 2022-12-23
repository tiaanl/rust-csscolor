use crate::convert;
use crate::ColorSpace;

/// The three color components that represent a color.
#[derive(Clone)]
pub struct ColorComponents(pub f32, pub f32, pub f32);

impl ColorComponents {
    /// Apply the given function to each component.
    pub fn copy_and_apply(&self, f: fn(f32) -> f32) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }
}

/// An absolutely specified color.
pub struct Color {
    components: ColorComponents,
    alpha: f32,
    color_space: ColorSpace,
}

impl Color {
    pub fn new(color_space: ColorSpace, components: ColorComponents) -> Self {
        Self {
            components,
            alpha: 1.0,
            color_space,
        }
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Convert this color to the specified color space.
    pub fn into_color_space(self, color_space: ColorSpace) -> Self {
        let result = convert::convert(self.color_space, &self.components, color_space);
        Self::new(color_space, result).with_alpha(self.alpha)
    }
}
