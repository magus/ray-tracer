use crate::vec3::Vec3;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(Vec3);
impl Color {
    pub(crate) fn new(rf: f64, gf: f64, bf: f64) -> Self {
        Color(Vec3::new(rf, gf, bf))
    }
}

impl std::ops::Deref for Color {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let r = (255.999 * self.x()) as u32;
        let g = (255.999 * self.y()) as u32;
        let b = (255.999 * self.z()) as u32;

        write!(f, "{} {} {}", r, g, b)
    }
}
