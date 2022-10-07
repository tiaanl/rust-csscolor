use crate::convert::IntoColor;
use crate::{convert::FromColor, Hsl, Hwb, Lab, Lch, Oklab, Oklch, Xyz, D50, D65};
use euclid::default::{Transform3D, Vector3D};
use std::marker::PhantomData;

pub trait ColorSpace {}

macro_rules! declare_color_space {
    ($name:ident, $new_name:ident) => {
        /// Color space.
        pub struct $name;

        impl Rgb<$name> {
            pub fn $new_name(red: f32, green: f32, blue: f32) -> Rgb<$name> {
                Rgb::<$name>::new(red, green, blue)
            }
        }

        impl ColorSpace for $name {}
    };
}

declare_color_space!(Srgb, srgb);

impl FromColor<Xyz<D65>> for Rgb<SrgbLinear> {
    /// Convert XYZ with D65 white point to linear sRGB.
    fn from_color(xyz: Xyz<D65>) -> Self {
        #[rustfmt::skip]
        const MAT: Transform3D<f32> = Transform3D::new(
            12831.0 / 3959.0,    -329.0 / 214.0,        -1974.0 / 3959.0,    0.0,
            -851781.0 / 878810.0,  1648619.0 / 878810.0,  36519.0 / 878810.0, 0.0,
            705.0 / 12673.0,     -2585.0 / 12673.0,      705.0 / 667.0,      0.0,
            0.0,                  0.0,                   0.0,                1.0,
        );

        let (x, y, z) = MAT
            .transform_vector3d(Vector3D::new(xyz.x, xyz.y, xyz.z))
            .into();

        Self::new(x, y, z)
    }
}

impl FromColor<Rgb<SrgbLinear>> for Rgb<Srgb> {
    fn from_color(from: Rgb<SrgbLinear>) -> Self {
        #[inline(always)]
        fn map(value: f32) -> f32 {
            let sign = if value < 0.0 { -1.0 } else { 1.0 };
            let abs = value.abs();

            if abs > 0.0031308 {
                sign * (1.055 * abs.powf(1.0 / 2.4) - 0.055)
            } else {
                value * 12.92
            }
        }

        Self::new(map(from.red), map(from.green), map(from.blue))
    }
}

impl FromColor<Rgb<Rec2020>> for Rgb<Srgb> {
    /// ITU-R BT.2020-2 p.4
    fn from_color(from: Rgb<Rec2020>) -> Self {
        const ALPHA: f32 = 1.09929682680944;
        const BETA: f32 = 0.018053968510807;

        fn map(value: f32) -> f32 {
            let sign = if value < 0.0 { -1.0 } else { 1.0 };
            let abs = value.abs();

            if abs > BETA {
                sign * (ALPHA * abs.powf(0.45) - (ALPHA - 1.0))
            } else {
                4.5 * value
            }
        }

        Self::new(map(from.red), map(from.green), map(from.blue))
    }
}

impl FromColor<Hsl> for Rgb<Srgb> {
    fn from_color(from: Hsl) -> Self {
        #[inline(always)]
        fn hue_to_rgb(m1: f32, m2: f32, mut h3: f32) -> f32 {
            if h3 < 0. {
                h3 += 3.0
            }
            if h3 > 3. {
                h3 -= 3.0
            }
            if h3 * 2. < 1. {
                m1 + (m2 - m1) * h3 * 2.0
            } else if h3 * 2.0 < 3.0 {
                m2
            } else if h3 < 2.0 {
                m1 + (m2 - m1) * (2.0 - h3) * 2.0
            } else {
                m1
            }
        }
        let m2 = if from.lightness <= 0.5 {
            from.lightness * (from.saturation + 1.)
        } else {
            from.lightness + from.saturation - from.lightness * from.saturation
        };
        let m1 = from.lightness * 2.0 - m2;
        let hue_times_3 = from.hue * 3.0;
        let red = hue_to_rgb(m1, m2, hue_times_3 + 1.0);
        let green = hue_to_rgb(m1, m2, hue_times_3);
        let blue = hue_to_rgb(m1, m2, hue_times_3 - 1.0);

        Self::new(red, green, blue)
    }
}

impl FromColor<Hwb> for Rgb<Srgb> {
    fn from_color(from: Hwb) -> Self {
        if from.whiteness + from.blackness >= 1.0 {
            let gray = from.whiteness / (from.whiteness + from.blackness);
            Self::new(gray, gray, gray)
        } else {
            let rgb: Rgb<Srgb> = Hsl::new(from.hue, 1.0, 0.5).into_color();
            let x = 1.0 - from.whiteness - from.blackness;
            let red = rgb.red * x + from.whiteness;
            let green = rgb.green * x + from.whiteness;
            let blue = rgb.blue * x + from.whiteness;
            Self::new(red, green, blue)
        }
    }
}

impl FromColor<Lab> for Rgb<Srgb> {
    fn from_color(from: Lab) -> Self {
        let xyz_d50 = Xyz::<D50>::from_color(from);
        let xyz_d65 = Xyz::<D65>::from_color(xyz_d50);
        let linear = Rgb::<SrgbLinear>::from_color(xyz_d65);
        Self::from_color(linear)
    }
}

impl FromColor<Lch> for Rgb<Srgb> {
    fn from_color(from: Lch) -> Self {
        let lab = Lab::from_color(from);
        lab.into_color()
    }
}

impl FromColor<Oklab> for Rgb<Srgb> {
    fn from_color(from: Oklab) -> Self {
        let xyz: Xyz<D65> = from.into_color();
        let linear: Rgb<SrgbLinear> = xyz.into_color();
        linear.into_color()
    }
}

impl FromColor<Oklch> for Rgb<Srgb> {
    fn from_color(from: Oklch) -> Self {
        let oklab: Oklab = from.into_color();
        oklab.into_color()
    }
}

declare_color_space!(SrgbLinear, srgb_linear);

impl FromColor<Rgb<Srgb>> for Rgb<SrgbLinear> {
    fn from_color(from: Rgb<Srgb>) -> Self {
        #[inline(always)]
        fn map(value: f32) -> f32 {
            let sign = if value < 0.0 { -1.0 } else { 1.0 };
            let abs = value.abs();

            if abs < 0.04045 {
                value / 12.92
            } else {
                sign * ((abs + 0.055) / 1.055).powf(2.4)
            }
        }

        Self::new(map(from.red), map(from.green), map(from.blue))
    }
}

declare_color_space!(DisplayP3, display_p3);
declare_color_space!(A98, a98);
declare_color_space!(Prophoto, prophoto);
declare_color_space!(Rec2020, rec2020);

/// Red, green, blue color format. Specialized on a color space.
///
/// Examples:
/// ```rust
/// use rust_csscolor::{DisplayP3, Rgb, Srgb};
///
/// let srgb = Rgb::<Srgb>::new(0.1, 0.2, 0.3);
/// let display_p3 = Rgb::<DisplayP3>::new(0.1, 0.2, 0.3);
/// ```
pub struct Rgb<C: ColorSpace> {
    pub red: f32,
    pub green: f32,
    pub blue: f32,

    phantom: PhantomData<C>,
}

impl<C: ColorSpace> Rgb<C> {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red,
            green,
            blue,
            phantom: PhantomData::default(),
        }
    }
}
