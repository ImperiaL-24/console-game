use crate::component::component::Pos;
use crate::component::mesh::Mesh;
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

#[entity(Pos)]
pub struct MeshEntity {
    mesh: Mesh,
    color: (u8,u8,u8)
}

impl MeshEntity {
    pub fn new(name: String, pos: Vec3, mesh: Mesh, color: (u8,u8,u8)) -> MeshEntity {
        MeshEntity { name, mesh, pos, color }
    }
}

impl Entity for MeshEntity {
    fn render_data(&self) -> Option<RenderData> {
        Some(RenderData::new(self.as_transform(), RenderPrimitive::mesh(&self.mesh), self.color))
    }
}

impl Tickable<Scene> for MeshEntity {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, delta_time: std::time::Duration) -> TickCode {
        TickCode::Success
    }
}
