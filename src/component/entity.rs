use crate::component::world::Scene;
use crate::{component::component::Value, termren::ticker::Tickable};
use crate::termren::ticker::TickCode::{self, Success};
use crate::termren::ticker::Ticker;
use std::sync::Arc;
//TODO: make a ID trait -> Entity = ID + Tickable
pub type Entity = TestEntity;


pub struct TestEntity {
    value: i32,
    scene: Option<Arc<&mut Scene>>
}

impl TestEntity {
    pub fn new(value: i32) -> TestEntity {
        TestEntity {value, scene: None}
    }

    pub fn get_scene(&self) -> Option<Arc<Scene>> {
        self.scene.clone()
    }

    pub(super) fn set_scene(&self, scene: Arc<&mut Scene>) {
        self.scene = Some(scene.clone());
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

impl Tickable for TestEntity {
    fn tick(&mut self, ticker: &Ticker, delta_time: &std::time::Duration) -> TickCode {
        self.set_val(self.get_val()+15);
        Success
    }
}