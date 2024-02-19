use crate::game::entities::point::Point;
use crate::scene::scene::Scene;
use crate::termren::ticker::TickCode::{self, Success};
use crate::termren::ticker::Tickable;
use termion::terminal_size;

use core::time::Duration;
use std::slice::Windows;
use std::{io, thread};

use super::input_reader::InputReader;
use std::io::Write;
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
    fn tick(&mut self, _ticker: Option<&mut (Arc<Mutex<Renderer>>, Arc<Mutex<InputReader>>)>, delta_time: Duration) -> TickCode {
        let size: (u16, u16) = terminal_size().unwrap();
        let width = size.0 as usize;
        let height = size.1 as usize - 2;
        //TODO: matrix struct or smth
        let mut state: Vec<Vec<(u8, u8, u8)>> = vec![vec![(0, 0, 0); width]; height];

        for (id, entity) in self.scene.lock().unwrap().entities() {
            let entity = entity.lock().unwrap();
            if let Some(render_data) = entity.render_data() {
                state[render_data.coord.1][render_data.coord.0] = render_data.color;
            }
        }

        //TODO: try to multithread better -> bruhaps create the buffer in a separate thread and print in separate thread and not join

        let mut last_color = state[0][0];
        let mut buffer = format!("\x1b[38;2;{};{};{}m", last_color.0, last_color.1, last_color.2);
        // let mut buffer = String::new();
        buffer.reserve_exact(width * height);

        // let mut thread_handles = vec![];

        for pixel_line in state {
            // thread_handles.push(thread::spawn(move || {
            // let mut buffer = String::new();

            // for pixel in pixel_line.iter() {
            //     if last_color == *pixel {
            //         buffer.push('█');
            //     } else {
            //         buffer.push_str(&format!("\x1b[38;2;{};{};{}m█", pixel.0, pixel.1, pixel.2));
            //         last_color = *pixel;
            //     }
            // }
            for pixel in pixel_line.iter() {
                if last_color == *pixel {
                    buffer.push(' ');
                } else {
                    buffer.push_str(&format!("\x1b[48;2;{};{};{}m ", pixel.0, pixel.1, pixel.2));
                    last_color = *pixel;
                }
            }
            // buffer
            //}));
        }

        // for thread in thread_handles.into_iter() {
        //     let buf = thread.join().unwrap();
        //     buffer.push_str(buf.as_str());
        // }
        // thread::spawn(move || {
        let mut out = io::stdout().lock();
        writeln!(out, "{}{}{}\x1b[0mFPS: {}\r", termion::clear::All, termion::cursor::Goto(1, 1), buffer, 1000000000 / delta_time.as_nanos()).unwrap_or(());
        // });
        Success
    }
}
