
use crate::termren::renderer::Renderer;
use crate::termren::ticker::Ticker;
use crate::termren::input_reader::InputReader;
use crate::component::world::Scene;
use crate::component::entity::TestEntity;
pub mod termren;
pub mod component;
use std::io::{self};
use termion::raw::IntoRawMode;
use std::sync::Arc;
use std::sync::Mutex;

const RENDER_TICKRATE: f64 = 30.0;
const WORLD_TICKRATE: f64 = 5.0;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let mut ticker: Ticker = Ticker::new(RENDER_TICKRATE);
    let mut world_ticker: Ticker = Ticker::new(WORLD_TICKRATE);
    
    let world = Arc::new(Mutex::new(Scene::new()));

    let entity = TestEntity::new(10);
    {
        world.lock().unwrap().add_entity(entity);
    }
    let renderer = Renderer::new(&world);
    let input = InputReader::new();
    ticker.add_ticked(renderer);
    ticker.add_ticked(input);

    world_ticker.add_ref_ticked(&world);

    
    
    
    let ticker_thread = ticker.start();
    let _world_thread = world_ticker.start();
    ticker_thread.join();
}