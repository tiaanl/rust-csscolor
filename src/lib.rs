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
mod convert;

pub use alpha::WithAlpha;
pub use colors::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::IntoColor;

    #[test]
    fn to_srgb() {
        let _: Rgb<Srgb> = Rgb::srgb_linear(0.1, 0.2, 0.3).into_color();
        let _: Rgb<Srgb> = Hsl::new(0.1, 0.2, 0.3).into_color();
        let _: Rgb<Srgb> = Hwb::new(0.1, 0.2, 0.3).into_color();
        let _: Rgb<Srgb> = Lab::new(20.0, 0.0, 10.0).into_color();
        let _: Rgb<Srgb> = Lch::new(20.0, 0.0, 10.0).into_color();
        let _: Rgb<Srgb> = Oklab::new(20.0, 0.0, 10.0).into_color();
        let _: Rgb<Srgb> = Oklch::new(20.0, 0.0, 10.0).into_color();
    }
}
