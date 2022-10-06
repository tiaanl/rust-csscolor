use std::ops::{Deref, DerefMut};

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

// impl<C, F> From<WithAlpha<F>> for WithAlpha<C> {
//     fn from(from: WithAlpha<F>) -> Self {
//         WithAlpha::new(Self::from(from.components), from.alpha)
//     }
// }

pub trait WithAlphaExt<C> {
    fn with_alpha(self, alpha: f32) -> WithAlpha<C>;
}

impl<T> WithAlphaExt<T> for T {
    fn with_alpha(self, alpha: f32) -> WithAlpha<T> {
        WithAlpha::new(self, alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rgb;

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
        // let srgb: WithAlpha<Rgb<Srgb>> = Rgb::rec2020(0.1, 0.2, 0.3).with_alpha(1.0).into();
    }
}
