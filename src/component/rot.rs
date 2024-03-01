use crate::Vec3;
use core::ops::Add;
use std::ops::{Index, IndexMut};
#[derive(PartialEq, Default, Clone)]
pub struct Rotation(f64, f64);

impl Index<usize> for Rotation {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => &self.0,
        }
    }
}

impl IndexMut<usize> for Rotation {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => &mut self.0,
        }
    }
}

impl Rotation {
    pub fn new(value: (f64, f64)) -> Rotation {
        Rotation(value.0, value.1)
    }

    pub fn normal(&self) -> Vec3 {
        let cos2 = f64::cos(self[1]);
        Vec3::new((f64::cos(self[0]) * cos2, f64::sin(self[0]) * cos2, f64::sin(self[1])))
    }
}

impl Into<Rotation> for (f64, f64) {
    fn into(self) -> Rotation {
        Rotation::new(self)
    }
}

impl Add<Rotation> for Rotation {
    type Output = Rotation;
    fn add(self, rhs: Rotation) -> Self::Output {
        (self[0] + rhs[0], self[1] + rhs[1]).into()
    }
}
