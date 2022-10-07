//! Color operations related to the [CSS Color specification ](https://drafts.csswg.org/css-color)
//!
//! The following color formats are supported:
//! - [RGB](Rgb) (red, green, blue) (with color spaces: srgb, srgb_linear, display_p3, a98, prophoto, rec2020)
//! - [XYZ](Xyz) (with white references: D50, D65)
//! - [HSL](Hsl) (hue, saturation, lightness)
//! - [HWB](Hwb) (hue, whiteness, blackness)
//! - [LAB](Lab) (lightness, a, b)
//! - [LCH](Lch) (lightness, chroma, hue)
//! - [OKLAB](Oklab) (lightness, a, b)
//! - [OKLCH](Oklch) (lightness, chroma, hue)

mod alpha;
mod colors;

pub use alpha::WithAlpha;
pub use colors::*;

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
