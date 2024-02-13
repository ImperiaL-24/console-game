use crate::game::entities::resource::Resource;
use crate::game::entities::test_entity::TestEntity;
use crate::scene::scene::Scene;
use crate::termren::input_reader::InputReader;
use crate::termren::renderer::Renderer;
use crate::termren::ticker::Ticker;
pub mod component;
pub mod entity;
pub mod game;
pub mod scene;
pub mod termren;
use game::entities::point::{self, Point};
use std::io::{self};
use std::sync::Arc;
use std::sync::Mutex;
use termion::raw::IntoRawMode;

const RENDER_TICKRATE: f64 = 60.0;
const WORLD_TICKRATE: f64 = 100.0;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();

    let mut scene = Scene::new();

    let entity = TestEntity::new("original".to_string(), 10);
    scene.add_entity(entity);

    let increase_value = Resource::new("res_1".to_string(), 100);
    scene.add_entity(increase_value);

    let point = Point::new("p1".to_string(), (1, 1), (255, 255, 255));
    scene.add_entity(point);

    let scene_ref = Arc::new(Mutex::new(scene));
    let renderer = Renderer::new(&scene_ref);
    let input = InputReader::new();

    let render_ticker = Ticker::new(RENDER_TICKRATE, (Arc::new(Mutex::new(renderer)), Arc::new(Mutex::new(input)))).start();
    let _world_ticker = Ticker::new(WORLD_TICKRATE, scene_ref).start();
    let _ = render_ticker.join();
}
