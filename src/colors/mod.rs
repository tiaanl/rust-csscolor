pub mod hsl;
pub mod hwb;
pub mod lab;
pub mod lch;
pub mod oklab;
pub mod oklch;
pub mod rgb;
pub mod xyz;

pub use hsl::Hsl;
pub use hwb::Hwb;
pub use lab::Lab;
pub use lch::Lch;
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use rgb::{DisplayP3, Prophoto, Rec2020, Rgb, Srgb, SrgbLinear, A98};
pub use xyz::{Xyz, D50, D65};

/// Convert from the polar form.
fn lch_to_lab(lightness: f32, chroma: f32, hue: f32) -> (f32, f32, f32) {
    (
        lightness,
        (chroma * hue * std::f32::consts::PI / 180.0).cos(),
        (chroma * hue * std::f32::consts::PI / 180.0).sin(),
    )
}
