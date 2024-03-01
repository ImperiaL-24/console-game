use crate::component::component::Pos;
use crate::component::component::Rot;
use crate::component::mesh::Mesh;
use crate::component::rot::Rotation;
use crate::component::transform_matrix::TransformMatrix;
use crate::termren::renderer::RenderPrimitive;
use crate::{
    component::{
        component::{Named, Transform},
        vec3::Vec3,
    },
    entity::entity::Entity,
    scene::scene::Scene,
    termren::{
        renderer::RenderData,
        ticker::{TickCode, Tickable},
    },
};
use ecs_derive::entity;

#[entity(Pos, Rot)]
pub struct MeshEntity {
    mesh: Mesh,
    color: (u8, u8, u8),
}

impl MeshEntity {
    pub fn new(name: String, pos: Vec3, mesh: Mesh, color: (u8, u8, u8)) -> MeshEntity {
        MeshEntity { name, mesh, pos, color, rot: Rotation::new((0., 0.)) }
    }
}

impl Entity for MeshEntity {
    fn render_data(&self) -> Option<RenderData> {
        Some(RenderData::new(self.as_transform(), RenderPrimitive::mesh(&self.mesh), self.color))
    }
}

impl Tickable<Scene> for MeshEntity {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, _delta_time: std::time::Duration) -> TickCode {
        self.rot[0] += 0.5 * _delta_time.as_secs_f64();
        // self.rot[1] += 0.5 * _delta_time.as_secs_f64();
        TickCode::Success
    }
}
