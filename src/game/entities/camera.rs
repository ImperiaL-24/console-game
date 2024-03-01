use crate::entity::entity::Entity;
use crate::Tri;
use crate::{
    component::{
        component::{Named, Pos, Rot, Transform},
        rot::Rotation,
        transform_matrix::TransformMatrix,
        vec3::Vec3,
    },
    scene::scene::Scene,
    termren::ticker::{TickCode, Tickable},
};
use ecs_derive::entity;
use termion::terminal_size;
#[entity(Pos, Rot)]
pub struct Camera {
    fov: f64,
}
impl Entity for Camera {}

impl Camera {
    pub fn new(name: String, fov: f64) -> Camera {
        Camera { name, fov, pos: Vec3::new((-1., 0., 5.)), rot: Rotation::new((0., -40f64.to_radians())) }
    }
    pub fn focal_length(&self) -> f64 {
        f64::from(1. / (2. * f64::tan(self.fov.to_radians() / 2.)))
    }
    pub fn project(&self, point: &Vec3) -> (i32, i32) {
        let rel_point = *point - self.pos;
        let focal = self.focal_length();
        let normal = self.rot.normal();
        let coord = (rel_point / (normal * rel_point) - normal) * focal;

        let sin1 = f64::sin(self.rot[0]);
        let cos1 = f64::cos(self.rot[0]);
        let sin2 = f64::sin(self.rot[1]);
        let size: (u16, u16) = terminal_size().unwrap();
        let aspect = size.0 as f64 / size.1 as f64 / 2.02627;
        let cc0 = -sin1 * coord[0] + cos1 * coord[1];
        let cc1 = (-sin2 * cos1 * coord[0] - sin1 * sin2 * coord[1] + f64::cos(self.rot[1]) * coord[2]) * aspect;
        (((0.5 - cc0) * size.0 as f64) as i32, ((0.5 - cc1) * size.1 as f64) as i32)
    }
    pub fn occluded(&self, tri: &Tri, transform: TransformMatrix) -> bool {
        //TODO: more precise occlusion. rn just checking if it is behind the camera. we need to check if it is inside the fustrum -> similarity with the normal, angle > fov or vfov
        let normal = self.rot.normal();
        for vertex in tri.verts() {
            let rel_point = transform * vertex - self.pos;
            if normal * rel_point > 0. {
                return false;
            }
        }
        true
    }
}

impl Tickable<Scene> for Camera {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, _delta_time: std::time::Duration) -> TickCode {
        TickCode::Success
    }
}
