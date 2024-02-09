use core::time::Duration;
use std::collections::HashMap;
use std::sync::Arc;
use crate::termren::ticker::Ticker;
use crate::termren::ticker::Tickable;
use crate::termren::ticker::TickCode::{self, Success};
use crate::component::entity::Entity;
pub struct Scene {
    entities: HashMap<i32,Entity>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { entities: HashMap::new() }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        entity.set_scene(Arc::new(self));
        self.entities.insert(1, entity);
    }
    pub fn get_entity(&self, id: &i32) -> Option<&Entity> {
        self.entities.get(id)
    }
}

impl Tickable for Scene {
    fn tick(&mut self, ticker: &Ticker, delta_time: &Duration) -> TickCode {
        for (key, entity) in self.entities.iter_mut() {
            entity.tick(ticker, delta_time);
        }
        Success
    }
}
