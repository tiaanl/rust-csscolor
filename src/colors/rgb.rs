use crate::{Hsl, Hwb, Lab, Lch, Oklab, Oklch, Xyz, D50, D65};
use euclid::default::{Transform3D, Vector3D};
use std::marker::PhantomData;

pub trait ColorSpace {}

macro_rules! declare_color_space {
    ($name:ident, $new_name:ident) => {
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

impl From<Xyz<D65>> for Rgb<SrgbLinear> {
    /// Convert XYZ with D65 white point to linear sRGB.
    fn from(xyz: Xyz<D65>) -> Self {
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

impl From<Rgb<SrgbLinear>> for Rgb<Srgb> {
    fn from(from: Rgb<SrgbLinear>) -> Self {
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

impl From<Hsl> for Rgb<Srgb> {
    fn from(from: Hsl) -> Self {
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

impl From<Hwb> for Rgb<Srgb> {
    fn from(from: Hwb) -> Self {
        if from.whiteness + from.blackness >= 1.0 {
            let gray = from.whiteness / (from.whiteness + from.blackness);
            Self::new(gray, gray, gray)
        } else {
            let rgb: Rgb<Srgb> = Hsl::new(from.hue, 1.0, 0.5).into();
            let x = 1.0 - from.whiteness - from.blackness;
            let red = rgb.red * x + from.whiteness;
            let green = rgb.green * x + from.whiteness;
            let blue = rgb.blue * x + from.whiteness;
            Self::new(red, green, blue)
        }
    }
}

impl From<Lab> for Rgb<Srgb> {
    fn from(from: Lab) -> Self {
        let xyz_d50 = Xyz::<D50>::from(from);
        let xyz_d65 = Xyz::<D65>::from(xyz_d50);
        let linear = Rgb::<SrgbLinear>::from(xyz_d65);
        Self::from(linear)
    }
}

impl From<Lch> for Rgb<Srgb> {
    fn from(_: Lch) -> Self {
        todo!()
    }
}

impl From<Oklab> for Rgb<Srgb> {
    fn from(_: Oklab) -> Self {
        todo!()
    }
}

impl From<Oklch> for Rgb<Srgb> {
    fn from(_: Oklch) -> Self {
        todo!()
    }
}

declare_color_space!(SrgbLinear, srgb_linear);

impl From<Rgb<Srgb>> for Rgb<SrgbLinear> {
    fn from(from: Rgb<Srgb>) -> Self {
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
