use std::ops::{Index, IndexMut, Mul, Div, Add, Sub};

#[derive(PartialEq, Default, Clone, Copy)]
// TODO: switch from tuple to array
pub struct Vec3(f64, f64, f64);

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

impl Vec3 {
    pub fn new(value: (f64, f64, f64)) -> Vec3 {
        Vec3(value.0, value.1, value.2)
    }

    pub fn dot(&self, vec: &Vec3) -> f64 {
        self[0] * vec[0] + self[1] * vec[1] + self[2] * vec[2]
    }

    pub fn normalize(&mut self) {
        let norm = f64::sqrt(self[0] * self[0] + self[1] * self[1] + self[2] * self[2]);
        self[0] /= norm;
        self[1] /= norm;
        self[2] /= norm;
    }
}

impl Into<Vec3> for (f64, f64, f64) {
    fn into(self) -> Vec3 {
        Vec3::new(self)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        (self[0] * rhs, self[1] * rhs, self[2] * rhs).into()
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (self[0] / rhs, self[1] / rhs, self[2] / rhs).into()
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.dot(&rhs)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        (self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]).into()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        (self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]).into()
    }
}