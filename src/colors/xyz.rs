use crate::{convert::FromColor, Lab};
use euclid::default::{Transform3D, Vector3D};
use std::marker::PhantomData;

pub trait WhiteRef {}

/// XYZ color format.
///
/// ```rust
/// use rust_csscolor::{D65, Xyz};
///
/// // Create a color with a D50 white reference.
/// let d50 = Xyz::d50(0.1, 0.2, 0.3);
///
/// // Create a color with a D65 white reference.
/// let d65 = Xyz::d65(0.1, 0.2, 0.3);
///
/// // Convert between them.
/// let converted: Xyz<D65> = d65.into();
/// // or
/// let converted = Xyz::<D65>::from(d50);
/// ```
pub struct Xyz<W: WhiteRef> {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    phantom: PhantomData<W>,
}

impl<W: WhiteRef> Xyz<W> {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            phantom: PhantomData::default(),
        }
    }
}

macro_rules! declare_white_ref {
    ($name:ident, $new_name:ident) => {
        pub struct $name;

        impl Xyz<$name> {
            pub fn $new_name(x: f32, y: f32, z: f32) -> Xyz<$name> {
                Xyz::<$name>::new(x, y, z)
            }
        }

        impl WhiteRef for $name {}
    };
}

declare_white_ref!(D50, d50);
declare_white_ref!(D65, d65);

impl FromColor<Lab> for Xyz<D50> {
    /// Convert Lab to D50-adapted XYZ
    /// <http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html>
    fn from_color(lab: Lab) -> Self {
        const WHITE: [f32; 3] = [0.3457 / 0.3585, 1.00000, (1.0 - 0.3457 - 0.3585) / 0.3585];

        const KAPPA: f32 = 24389.0 / 27.0; // 29 ^ 3 / 3 ^ 3
        const EPSILON: f32 = 216.0 / 24389.0; // 6 ^ 3 / 29 ^ 3

        // Compute f, starting with the luminance-related term.
        let f1 = (lab.lightness + 16.0) / 116.0;
        let f0 = lab.a / 500.0 + f1;
        let f2 = f1 - lab.b / 200.0;

        let x = if f0.powf(3.0) > EPSILON {
            f0.powf(3.0)
        } else {
            (116.0 * f0 - 16.0) / KAPPA
        };

        let y = if lab.lightness > KAPPA * EPSILON {
            ((lab.lightness + 16.0) / 116.0).powf(3.0)
        } else {
            lab.lightness / KAPPA
        };

        let z = if f2.powf(3.0) > EPSILON {
            f2.powf(3.0)
        } else {
            (116.0 * f2 - 16.0) / KAPPA
        };

        // Compute XYZ by scaling xyz by reference white.
        Xyz::d50(x * WHITE[0], y * WHITE[1], z * WHITE[2])
    }
}

impl FromColor<Xyz<D50>> for Xyz<D65> {
    fn from_color(xyz: Xyz<D50>) -> Self {
        // Bradford chromatic adaptation from D50 to D65.
        #[allow(clippy::excessive_precision)]
        #[rustfmt::skip]
        const MAT: Transform3D<f32> = Transform3D::new(
             0.9554734527042182,   -0.023098536874261423,  0.0632593086610217,   0.0,
            -0.028369706963208136,  1.0099954580058226,    0.021041398966943008, 0.0,
             0.012314001688319899, -0.020507696433477912,  1.3303659366080753,   0.0,
             0.0,                   0.0,                   0.0,                  1.0,
        );

        let (x, y, z) = MAT
            .transform_vector3d(Vector3D::new(xyz.x, xyz.y, xyz.z))
            .into();

        Self::new(x, y, z)
    }
}
