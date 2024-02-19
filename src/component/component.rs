use super::vec3::Vec3;

pub trait Named {
    fn name(&self) -> String;
}
//TODO: derives of traits + position rotation + camera
pub trait Value {
    fn get_val(&self) -> i32;
    fn set_val(&mut self, value: i32);
}
// TODO: implement a transform matrix? idfk
pub trait Transform {
    fn pos(&self) -> Vec3;
    fn set_pos(&self, new_pos: Vec3);
    fn rot(&self) -> Vec3; // TODO: or a Rot idk
    fn set_rot(&self, new_rot: Vec3);
    fn scale(&self) -> f64;
    fn set_scale(&self, new_scale: f64);
}
