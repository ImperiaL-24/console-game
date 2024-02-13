use crate::{
    component::component::{Named, Value},
    entity::entity::Entity,
    scene::scene::Scene,
    termren::{
        renderer::RenderData,
        ticker::{TickCode, Tickable},
    },
};

pub struct Resource {
    value: i32,
    name: String,
}

impl Resource {
    pub fn new(name: String, value: i32) -> Resource {
        Resource { value, name }
    }
}

impl Entity for Resource {
    fn render_data(&self) -> Option<RenderData> {
        None
    }
}

impl Named for Resource {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Value for Resource {
    fn get_val(&self) -> i32 {
        self.value
    }

    fn set_val(&mut self, value: i32) {
        self.value = value;
    }
}

impl Tickable<Scene> for Resource {
    fn tick(&mut self, _scene_opt: Option<&mut Scene>, _delta_time: &std::time::Duration) -> TickCode {
        TickCode::Success
    }
}
