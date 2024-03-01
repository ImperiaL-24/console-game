use crate::entity::entity::Entity;
use crate::game::entities::camera::Camera;
use crate::termren::ticker::TickCode::{self, Success};
use crate::termren::ticker::Tickable;

use core::time::Duration;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
//TODO: resource hashmap
pub struct Scene {
    active_camera: Arc<Mutex<Camera>>,
    entities: HashMap<String, Arc<Mutex<dyn Entity>>>,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene { entities: HashMap::new(), active_camera: Arc::new(Mutex::new(camera)) }
    }
    pub fn add_entity(&mut self, entity: impl Entity + 'static) {
        self.entities.insert(entity.name(), Arc::new(Mutex::new(entity)));
    }
    pub fn get_entity(&self, id: String) -> Option<Arc<Mutex<dyn Entity>>> {
        if let Some(val) = self.entities.get(&id) {
            return Some(val.clone());
        }
        None
    }
    pub fn active_camera(&self) -> Arc<Mutex<Camera>> {
        self.active_camera.clone()
    }

    pub fn entities(&self) -> &HashMap<String, Arc<Mutex<dyn Entity>>> {
        &self.entities
    }
}

impl Tickable<Scene> for Scene {
    fn tick(&mut self, _origin: Option<&mut Scene>, delta_time: Duration) -> TickCode {
        self.active_camera.clone().lock().unwrap().tick(Some(self), delta_time);
        //TODO: find a way to somehow not clone this
        for (_key, value) in self.entities.clone() {
            value.lock().unwrap().tick(Some(self), delta_time);
        }

        Success
    }
}
