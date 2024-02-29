use crate::component::component::Pos;
use crate::component::transform_matrix::TransformMatrix;
use crate::termren::renderer::RenderData;
use crate::{
    component::{
        component::{Named, Transform},
        vec3::Vec3,
    },
    entity::entity::Entity,
    scene::scene::Scene,
    termren::{
        renderer::RenderPrimitive,
        ticker::{TickCode, Tickable},
    },
};
use ecs_derive::entity;

#[entity(Pos)]
pub struct PointEntity {
    color: (u8, u8, u8),
}

impl PointEntity {
    pub fn new(name: String, pos: Vec3, color: (u8, u8, u8)) -> PointEntity {
        PointEntity { name, color, pos }
    }
}

impl Entity for PointEntity {
    fn render_data(&self) -> Option<RenderData> {
        Some(RenderData::new(self.as_transform(), RenderPrimitive::point(self.pos), self.color))
    }
}

impl Tickable<Scene> for PointEntity {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, delta_time: std::time::Duration) -> TickCode {
        // let mut val = self.pos[0] + 5. * delta_time.as_secs_f64();
        // if val > 85. {
        //     val -= 85.
        // }
        // self.pos[0] = val;
        TickCode::Success
    }
}
