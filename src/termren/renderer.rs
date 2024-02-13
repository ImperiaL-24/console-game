use crate::scene::scene::Scene;
use crate::termren::ticker::TickCode::{self, Success};
use crate::termren::ticker::Tickable;

use core::time::Duration;

use super::input_reader::InputReader;
use std::fmt::Write;
use std::sync::Arc;
use std::sync::Mutex;

pub struct RenderData {
    coord: (usize, usize),
    color: (u8, u8, u8),
}

impl RenderData {
    pub fn new(coord: (usize, usize), color: (u8, u8, u8)) -> RenderData {
        RenderData { coord, color }
    }
}

pub struct Renderer {
    scene: Arc<Mutex<Scene>>,
}

impl Renderer {
    pub fn new(scene: &Arc<Mutex<Scene>>) -> Renderer {
        Renderer { scene: scene.clone() }
    }
}

impl Tickable<(Arc<Mutex<Renderer>>, Arc<Mutex<InputReader>>)> for Renderer {
    fn tick(&mut self, _ticker: Option<&mut (Arc<Mutex<Renderer>>, Arc<Mutex<InputReader>>)>, delta_time: &Duration) -> TickCode {
        const SCREEN_HEIGHT: usize = 10;
        const SCREEN_WIDTH: usize = 10;

        let mut state: [[(u8, u8, u8); SCREEN_WIDTH]; SCREEN_HEIGHT] = [[(0, 0, 0); SCREEN_WIDTH]; SCREEN_HEIGHT];

        let mut buffer = String::new();
        for (id, entity) in self.scene.lock().unwrap().entities() {
            let entity = entity.lock().unwrap();
            if let Some(render_data) = entity.render_data() {
                state[render_data.coord.1][render_data.coord.0] = render_data.color;
            }
        }
        for pixel_line in state {
            for pixel in pixel_line {
                write!(buffer, "\x1b[38;2;{};{};{}m█\x1b[0m", pixel.0, pixel.1, pixel.2).unwrap();
            }
            write!(buffer, "\r\n").unwrap();
        }
        println!("{}{}{}FPS: {}\r", termion::clear::All, termion::cursor::Goto(1, 1), buffer, 1000000000 / delta_time.as_nanos());
        Success
    }
}
