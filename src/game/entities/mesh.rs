use crate::component::component::Pos;
use crate::component::mesh::{self, Mesh};
use crate::component::transform_matrix::TransformMatrix;
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
}

impl MeshEntity {
    pub fn new(name: String, pos: Vec3, mesh: Mesh) -> MeshEntity {
        MeshEntity { name, mesh, pos }
    }
}

impl Entity for MeshEntity {
    fn render_data(&self) -> Option<RenderData> {
        Some(RenderData::mesh(&self.mesh))
    }
}

impl Tickable<Scene> for MeshEntity {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, delta_time: std::time::Duration) -> TickCode {
        TickCode::Success
    }
}
