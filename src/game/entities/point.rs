use crate::{
    component::component::Named,
    entity::entity::Entity,
    scene::scene::Scene,
    termren::{
        renderer::RenderData,
        ticker::{TickCode, Tickable},
    },
};
use ecs_derive::entity;

#[entity]
pub struct Point {
    coord: (usize, usize),
    color: (u8, u8, u8),
}

impl Point {
    pub fn new(name: String, coord: (usize, usize), color: (u8, u8, u8)) -> Point {
        Point { name, coord, color }
    }
}

impl Entity for Point {
    fn render_data(&self) -> Option<RenderData> {
        Some(RenderData::new(self.coord, self.color))
    }
}

impl Tickable<Scene> for Point {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, _delta_time: std::time::Duration) -> TickCode {
        let mut val = self.coord.0 + 1;
        if val > 9 {
            val = 0
        }
        self.coord.0 = val;
        TickCode::Success
    }
}
