use std::ops::{Index, IndexMut};

#[derive(PartialEq, Default)]
// switch from tuple to array
pub struct Vec3(f64, f64, f64);
// TODO: inline shit
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => &self.0,
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => &mut self.0,
        }
    }
}

//TODO: implement these
impl Vec3 {
    pub fn new(value: (f64, f64, f64)) -> Vec3 {
        Vec3(value.0, value.1, value.2)
    }
    fn relative_to(&self, reference: &Vec3) {}

    fn distance_to(&self, point: &Vec3) {}

    fn normalize(&self) {}
}
