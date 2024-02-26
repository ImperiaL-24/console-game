use crate::component::component::{Pos, Rot, Scale};
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
        Some(RenderData::point(self.pos.clone(), self.color))
    }
}

impl Tickable<Scene> for PointEntity {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, delta_time: std::time::Duration) -> TickCode {
        let mut val = self.pos[0] + 5. * delta_time.as_secs_f64();
        if val > 85. {
            val -= 85.
        }
        self.pos[0] = val;
        TickCode::Success
    }
}
