//! Infrastructure to convert between color formats/spaces.
//!
//! This is basically a reimplementation of the From/Into traits from `std::convert`, but
//! with more strict rules.  This also allows us to convert between `WithAlpha<C>` variants.

/// Implement this trait to convert from another color format/space.
pub trait FromColor<T>: Sized {
    #[must_use]
    fn from_color(from: T) -> Self;
}

/// Do not implement this trait for any color format/space.  It is there for convenience, e.g.
///
/// ```rust
/// use rust_csscolor::{IntoColor, Rgb, Srgb};
/// let srgb: Rgb<Srgb> = Rgb::rec2020(0.1, 0.2, 0.3).into_color();
/// ```
pub trait IntoColor<T>: Sized {
    #[must_use]
    fn into_color(self) -> T;
}

impl<T, U: FromColor<T>> IntoColor<U> for T {
    fn into_color(self) -> U {
        U::from_color(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ab {
        pub a: f32,
        pub b: f32,
    }

    struct Cd {
        pub c: f32,
        pub d: f32,
    }

    impl FromColor<Ab> for Cd {
        fn from_color(from: Ab) -> Self {
            Self {
                c: from.a,
                d: from.b,
            }
        }
    }

    struct Ef {
        e: f32,
        f: f32,
    }

    impl FromColor<Ab> for Ef {
        fn from_color(from: Ab) -> Self {
            Self {
                e: from.a,
                f: from.b,
            }
        }
    }

    #[test]
    fn from() {
        let cd = Cd::from_color(Ab { a: 0.1, b: 0.2 });
        assert_eq!(cd.c, 0.1);
        assert_eq!(cd.d, 0.2);
        let ef = Ef::from_color(Ab { a: 0.1, b: 0.2 });
        assert_eq!(ef.e, 0.1);
        assert_eq!(ef.f, 0.2);
    }

    #[test]
    fn into() {
        let cd: Cd = Ab { a: 0.1, b: 0.2 }.into_color();
        assert_eq!(cd.c, 0.1);
        assert_eq!(cd.d, 0.2);
        let ef: Ef = Ab { a: 0.1, b: 0.2 }.into_color();
        assert_eq!(ef.e, 0.1);
        assert_eq!(ef.f, 0.2);
    }
}
