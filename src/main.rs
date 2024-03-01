// use crate::game::entities::resource::Resource;
use crate::scene::scene::Scene;
use crate::termren::input_reader::InputReader;
use crate::termren::renderer::Renderer;
use crate::termren::ticker::Ticker;
pub mod component;
pub mod entity;
pub mod game;
pub mod scene;
pub mod termren;
use component::mesh::{Mesh, Tri};
use component::vec3::Vec3;
use game::entities::camera::Camera;
use game::entities::mesh::MeshEntity;
// use game::entities::point::PointEntity;
use std::io::{self};
use std::sync::Arc;
use std::sync::Mutex;
use termion::raw::IntoRawMode;

const RENDER_TICKRATE: f64 = 1000.0;
const WORLD_TICKRATE: f64 = 40.0;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    let camera = Camera::new("cam".to_string(), 90.);
    let mut scene = Scene::new(camera);
    // let entity = TestEntity::new("original".to_string(), 10);
    // scene.add_entity(entity);

    // let increase_value = Resource::new("res_1".to_string(), 100);
    // scene.add_entity(increase_value);
    // for i in 0..400 {
    //     let mut position = Vec3::new((f64::cos(i as f64 * std::f64::consts::PI / 200.), f64::sin(i as f64 * std::f64::consts::PI / 200.), 0.));
    //     position = position * 40.;
    //     position[1] += 40.;
    //     position[0] += 40.;
    //     position[1] *= 0.50;
    //     let point = PointEntity::new(format!("point_{}", i), position, (255, 255, 255));
    //     scene.add_entity(point);
    // }
    // let point = Point::new("p1".to_string(), (1., 1., 0.).into(), (255, 127, 55));
    // let point2 = Point::new("p2".to_string(), (1., 2., 0.).into(), (127, 255, 55));
    // scene.add_entity(point);
    // scene.add_entity(point2);
    // let point1 = PointEntity::new("point_1".to_string(), (158.,17.,0.).into(), (255, 0, 0));
    // let point2 = PointEntity::new("point_2".to_string(), (105.,38.,0.).into(), (255, 0, 0));
    // let point3 = PointEntity::new("point_3".to_string(), (211.,44.,0.).into(), (255, 0, 0));

    // scene.add_entity(point1);
    // scene.add_entity(point2);
    // scene.add_entity(point3);

    // let tri = Tri::new((1.5, 0.5, 0.).into(), (0.6, -0.5, 0.).into(), (2., 0.5, 0.5).into());
    // let mesh = Mesh::new(vec![tri]);
    // let mesh_ent = MeshEntity::new("mesh".to_string(), (1., 0., 0.).into(), mesh, (255, 255, 255));
    // scene.add_entity(mesh_ent);

    let tri1 = Tri::new((-1., -1., -1.).into(), (1., -1., -1.).into(), (1., -1., 1.).into());
    let tri2 = Tri::new((-1., -1., -1.).into(), (-1., -1., 1.).into(), (1., -1., 1.).into());
    let tri3 = Tri::new((-1., 1., -1.).into(), (1., 1., -1.).into(), (1., 1., 1.).into());
    let tri4 = Tri::new((-1., 1., -1.).into(), (-1., 1., 1.).into(), (1., 1., 1.).into());
    let tri5 = Tri::new((1., -1., -1.).into(), (1., 1., -1.).into(), (1., 1., 1.).into());
    let tri6 = Tri::new((1., -1., -1.).into(), (1., -1., 1.).into(), (1., 1., 1.).into());
    let tri7 = Tri::new((-1., -1., -1.).into(), (-1., 1., -1.).into(), (-1., 1., 1.).into());
    let tri8 = Tri::new((-1., -1., -1.).into(), (-1., -1., 1.).into(), (-1., 1., 1.).into());
    let tri9 = Tri::new((-1., -1., 1.).into(), (-1., 1., 1.).into(), (1., 1., 1.).into());
    let tri10 = Tri::new((-1., -1., 1.).into(), (1., -1., 1.).into(), (1., 1., 1.).into());
    let tri11 = Tri::new((-1., -1., -1.).into(), (-1., 1., -1.).into(), (1., 1., -1.).into());
    let tri12 = Tri::new((-1., -1., -1.).into(), (1., -1., -1.).into(), (1., 1., -1.).into());
    let mesh = Mesh::new(vec![tri1, tri2, tri3, tri4, tri5, tri6, tri7, tri8, tri9, tri10, tri11, tri12]);
    let mesh_ent = MeshEntity::new("mesh".to_string(), (6., 0., -1.).into(), mesh, (255, 255, 255));
    scene.add_entity(mesh_ent);

    let scene_ref = Arc::new(Mutex::new(scene));
    let renderer = Renderer::new(&scene_ref);
    let input = InputReader::new();

    let render_ticker = Ticker::new(RENDER_TICKRATE, (Arc::new(Mutex::new(renderer)), Arc::new(Mutex::new(input)))).start();
    let _world_ticker = Ticker::new(WORLD_TICKRATE, scene_ref).start();
    let _ = render_ticker.join();
}
