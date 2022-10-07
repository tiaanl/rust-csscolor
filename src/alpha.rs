use crate::convert::FromColor;
use std::ops::{Deref, DerefMut};

/// Container used to add an alpha component to any color format.
///
/// ```rust
/// use rust_csscolor::{Rgb, WithAlphaExt};
///
/// let srgb_with_alpha = Rgb::srgb(0.1, 0.2, 0.3).with_alpha(0.4);
/// assert_eq!(srgb_with_alpha.red, 0.1);
/// assert_eq!(srgb_with_alpha.green, 0.2);
/// assert_eq!(srgb_with_alpha.blue, 0.3);
/// assert_eq!(srgb_with_alpha.alpha, 0.4);
/// ```
pub struct WithAlpha<C> {
    pub components: C,
    pub alpha: f32,
}

impl<C> WithAlpha<C> {
    pub fn new(components: C, alpha: f32) -> Self {
        Self { components, alpha }
    }
}

impl<C> Deref for WithAlpha<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

impl<C> DerefMut for WithAlpha<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.components
    }
}

pub trait WithAlphaExt<C> {
    fn with_alpha(self, alpha: f32) -> WithAlpha<C>;
}

impl<T> WithAlphaExt<T> for T {
    fn with_alpha(self, alpha: f32) -> WithAlpha<T> {
        WithAlpha::new(self, alpha)
    }
}

/// Allows converting from one color format/space with alpha to another color format/space with
/// alpha.
impl<F, T: FromColor<F>> FromColor<WithAlpha<F>> for WithAlpha<T> {
    fn from_color(from: WithAlpha<F>) -> Self {
        Self {
            components: T::from_color(from.components),
            alpha: from.alpha,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{convert::IntoColor, Rgb, Srgb};

    #[test]
    fn basic() {
        let a = WithAlpha::new(Rgb::rec2020(0.1, 0.2, 0.3), 1.0);
        assert_eq!(a.red, 0.1);
        assert_eq!(a.green, 0.2);
        assert_eq!(a.blue, 0.3);
        assert_eq!(a.alpha, 1.0);

        let a = Rgb::rec2020(0.1, 0.2, 0.3).with_alpha(1.0);
        assert_eq!(a.red, 0.1);
        assert_eq!(a.green, 0.2);
        assert_eq!(a.blue, 0.3);
        assert_eq!(a.alpha, 1.0);
    }

    #[test]
    fn conversion() {
        let with_alpha = Rgb::rec2020(0.1, 0.2, 0.3).with_alpha(1.0);
        let _srgb: WithAlpha<Rgb<Srgb>> = with_alpha.into_color();
    }
}
