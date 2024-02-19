use ecs_derive::entity;

use crate::component::component::Value;
use crate::termren::renderer::RenderData;
use crate::{
    component::component::Named,
    entity::entity::Entity,
    scene::scene::Scene,
    termren::ticker::{TickCode, Tickable},
};

use super::resource::Resource;

#[entity]
pub struct TestEntity {
    value: i32,
}

impl TestEntity {
    pub fn new(name: String, value: i32) -> TestEntity {
        TestEntity { value, name }
    }
}

//TODO: change i32 to smth meaningful -> enum (screen -> anchor + offset + size + text whatever / world -> a mesh + material + position)
impl Entity for TestEntity {
    fn render_data(&self) -> Option<RenderData> {
        None
    }
}

impl Value for TestEntity {
    fn get_val(&self) -> i32 {
        self.value
    }

    fn set_val(&mut self, value: i32) {
        self.value = value;
    }
}

impl Tickable<Scene> for TestEntity {
    fn tick(&mut self, scene_opt: Option<&mut Scene>, _delta_time: std::time::Duration) -> TickCode {
        let scene = scene_opt.unwrap();

        if scene.entities().len() < 30 {
            let test1 = TestEntity::new(scene.entities().len().to_string(), 5);
            scene.add_entity(test1);
        }

        let res = scene.get_entity("res_1".to_string()).unwrap().lock().unwrap().downcast_ref::<Resource>().unwrap().get_val();
        self.value = self.value + res;
        TickCode::Success
    }
}
