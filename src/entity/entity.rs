use downcast_rs::{impl_downcast, Downcast};

use crate::scene::scene::Scene;
use crate::termren::renderer::RenderData;
use crate::{component::component::Named, termren::ticker::Tickable};

pub trait Entity: Named + Tickable<Scene> + Downcast {
    fn render_data(&self) -> Option<RenderData> {
        None
    }
}
impl_downcast!(Entity);
