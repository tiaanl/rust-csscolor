//! Color operations related to the [CSS Color specification ](https://drafts.csswg.org/css-color)

mod alpha;
mod hsl;
mod hwb;
mod lab;
mod lch;
mod oklab;
mod oklch;
mod rgb;
mod xyz;

pub use alpha::WithAlpha;
pub use hsl::Hsl;
pub use hwb::Hwb;
pub use lab::Lab;
pub use lch::Lch;
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use rgb::{DisplayP3, Prophoto, Rec2020, Rgb, Srgb, SrgbLinear, A98};
pub use xyz::{D50, D65, XYZ};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_srgb() {
        let _ = Rgb::<Srgb>::from(Rgb::srgb_linear(0.1, 0.2, 0.3));
        let _ = Rgb::<Srgb>::from(Hsl::new(0.1, 0.2, 0.3));
        let _ = Rgb::<Srgb>::from(Hwb::new(0.1, 0.2, 0.3));
        let _ = Rgb::<Srgb>::from(Lab::new(20.0, 0.0, 10.0));
        let _ = Rgb::<Srgb>::from(Lch::new(20.0, 0.0, 10.0));
        let _ = Rgb::<Srgb>::from(Oklab::new(20.0, 0.0, 10.0));
        let _ = Rgb::<Srgb>::from(Oklch::new(20.0, 0.0, 10.0));
    }
}
