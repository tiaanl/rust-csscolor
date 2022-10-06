use crate::Lab;
use euclid::default::{Transform3D, Vector3D};
use std::marker::PhantomData;

pub trait WhiteRef {}

pub struct XYZ<W: WhiteRef> {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    phantom: PhantomData<W>,
}

impl<W: WhiteRef> XYZ<W> {
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

        impl XYZ<$name> {
            pub fn $new_name(x: f32, y: f32, z: f32) -> XYZ<$name> {
                XYZ::<$name>::new(x, y, z)
            }
        }

        impl WhiteRef for $name {}
    };
}

declare_white_ref!(D50, d50);
declare_white_ref!(D65, d65);

impl From<Lab> for XYZ<D50> {
    /// Convert Lab to D50-adapted XYZ
    /// http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
    fn from(lab: Lab) -> Self {
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
        XYZ::d50(x * WHITE[0], y * WHITE[1], z * WHITE[2])
    }
}

impl From<XYZ<D50>> for XYZ<D65> {
    fn from(xyz: XYZ<D50>) -> Self {
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
