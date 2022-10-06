pub struct LAB {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
}

impl LAB {
    pub fn new(lightness: f32, a: f32, b: f32) -> Self {
        Self { lightness, a, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let lab = LAB::new(20.0, 0.0, 10.0);
        assert_eq!(lab.lightness, 20.0);
        assert_eq!(lab.a, 0.0);
        assert_eq!(lab.b, 10.0);
    }
}
