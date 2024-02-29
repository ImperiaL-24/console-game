use ecs_derive::entity;

use crate::{
    component::component::{Named, Value},
    entity::entity::Entity,
    scene::scene::Scene,
    termren::ticker::{TickCode, Tickable},
};
//TODO: generic resource + special place in scene
#[entity]
pub struct Resource {
    value: i32,
}

impl Resource {
    pub fn new(name: String, value: i32) -> Resource {
        Resource { value, name }
    }
}

impl Entity for Resource {}

impl Value for Resource {
    fn get_val(&self) -> i32 {
        self.value
    }

    fn set_val(&mut self, value: i32) {
        self.value = value;
    }
}

impl Tickable<Scene> for Resource {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, _delta_time: std::time::Duration) -> TickCode {
        TickCode::Success
    }
}
