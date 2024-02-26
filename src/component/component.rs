use super::{transform_matrix::TransformMatrix, vec3::Vec3};

pub trait Named {
    fn name(&self) -> String;
}
//TODO: camera
pub trait Value {
    fn get_val(&self) -> i32;
    fn set_val(&mut self, value: i32);
}

pub trait Pos: Transform {
    fn pos(&self) -> Vec3;
    fn set_pos(&mut self, new_pos: Vec3);
}

pub trait Rot: Transform {
    fn rot(&self) -> Vec3; // TODO: or a Rot idk
    fn set_rot(&mut self, new_rot: Vec3);
}

pub trait Scale: Transform {
    fn scale(&self) -> f64;
    fn set_scale(&mut self, new_scale: f64);
}

pub trait Transform {
    fn as_transform(&self) -> TransformMatrix;
}
