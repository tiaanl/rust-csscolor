/// The color space that color components can be specified in.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ColorSpace {
    /// CIE L*a*b* color space.
    /// https://w3c.github.io/csswg-drafts/css-color-4/#lab-colors
    Lab,
    /// Polar form of [Lab].
    /// https://w3c.github.io/csswg-drafts/css-color-4/#lch-colors
    Lch,
    // Oklab color space.
    /// https://w3c.github.io/csswg-drafts/css-color-4/#lab-colors
    Oklab,
    /// Polar form of [Oklab].
    /// https://w3c.github.io/csswg-drafts/css-color-4/#lch-colors
    Oklch,
    /// Industry standard sRGB color space.
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-sRGB
    Srgb,
    /// Linear sRGB is the same as sRGB, except that the transfer function is
    /// linear-light (there is no gamma-encoding).
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-sRGB-linear
    SrgbLinear,
    /// The "display-p3" color space.
    /// https://www.color.org/chardata/rgb/DisplayP3.xalter
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-display-p3
    DisplayP3,
    /// The "a98-rgb" color space.
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-a98-rgb
    A98Rgb,
    /// The "prophoto-rgb" color space.
    /// http://www.realtimerendering.com/blog/2011-color-and-imaging-conference-part-vi-special-session/
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-prophoto-rgb
    ProphotoRgb,
    /// The "rec-2020" color space.
    /// http://www.itu.int/rec/R-REC-BT.2020/en
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-rec2020
    Rec2020,
    /// The CIE XYZ color space with a D50 white reference.
    /// http://www.cie.co.at/publications/colorimetry-4th-edition
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-xyz
    XyzD50,
    /// The CIE XYZ color space with a D65 white reference.
    /// http://www.cie.co.at/publications/colorimetry-4th-edition
    /// https://w3c.github.io/csswg-drafts/css-color-4/#predefined-xyz
    XyzD65,
}

impl ColorSpace {
    /// Returns true if the color space represents a rectangular orthogonal color.
    #[inline]
    pub fn is_rectangular(&self) -> bool {
        !self.is_polar()
    }

    /// Returns true if the color space represents a cylindrical polar color.
    #[inline]
    pub fn is_polar(&self) -> bool {
        matches!(self, ColorSpace::Lch | ColorSpace::Oklch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_polar_or_rectangular() {
        let expected = [
            (ColorSpace::Lab, false),
            (ColorSpace::Lch, true),
            (ColorSpace::Oklab, false),
            (ColorSpace::Oklch, true),
            (ColorSpace::Srgb, false),
            (ColorSpace::SrgbLinear, false),
            (ColorSpace::DisplayP3, false),
            (ColorSpace::A98Rgb, false),
            (ColorSpace::ProphotoRgb, false),
            (ColorSpace::Rec2020, false),
            (ColorSpace::XyzD50, false),
            (ColorSpace::XyzD65, false),
        ];

        for (color_space, is_polar) in expected {
            assert_eq!(color_space.is_polar(), is_polar);
            assert_eq!(color_space.is_rectangular(), !is_polar);
        }
    }
}
