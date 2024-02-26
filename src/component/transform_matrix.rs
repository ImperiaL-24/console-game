use std::{
    ops::{Index, IndexMut, Mul},
    process::Output,
};

use super::vec3::Vec3;

#[derive(PartialEq)]
pub struct TransformMatrix([f64; 16]);

impl TransformMatrix {
    pub fn new() -> TransformMatrix {
        let mut mat = [0.0; 16];
        for i in 0..4 {
            mat[5 * i] = 1.0;
        }
        TransformMatrix(mat)
    }
}

impl Index<(usize, usize)> for TransformMatrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[4 * index.0 + index.1]
    }
}

impl IndexMut<(usize, usize)> for TransformMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[4 * index.0 + index.1]
    }
}

impl Mul for TransformMatrix {
    type Output = TransformMatrix;
    fn mul(self, rhs: Self) -> Self {
        let mut mat = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    mat[4 * i + j] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        TransformMatrix(mat)
    }
}

impl Mul<Vec3> for TransformMatrix {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut vec = Vec3::new((0.0, 0.0, 0.0));
        for i in 0..4 {
            for k in 0..4 {
                vec[i] += self[(i, k)] * rhs[k];
            }
        }
        vec
    }
}
