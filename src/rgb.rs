use std::marker::PhantomData;

pub trait ColorSpace {}

macro_rules! declare_color_space {
    ($name:ident) => {
        #[allow(non_camel_case_types)]
        pub struct $name;

        impl RGB<$name> {
            pub fn $name(red: f32, green: f32, blue: f32) -> RGB<$name> {
                RGB::<$name>::new(red, green, blue)
            }
        }

        impl ColorSpace for $name {}
    };
}

declare_color_space!(srgb);
declare_color_space!(srgb_linear);
declare_color_space!(display_p3);
declare_color_space!(a98_rgb);
declare_color_space!(prophoto_rgb);
declare_color_space!(rec2020);

#[allow(clippy::upper_case_acronyms)]
pub struct RGB<C: ColorSpace> {
    pub red: f32,
    pub green: f32,
    pub blue: f32,

    phantom: PhantomData<C>,
}

impl<C: ColorSpace> RGB<C> {
    fn new(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red,
            green,
            blue,
            phantom: PhantomData::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let rgb = RGB::srgb(0.1, 0.2, 0.3);

        assert_eq!(rgb.red, 0.1);
        assert_eq!(rgb.green, 0.2);
        assert_eq!(rgb.blue, 0.3);
    }
}
