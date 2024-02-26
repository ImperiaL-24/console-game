use crate::component::mesh::Mesh;
use crate::component::vec3::Vec3;
use crate::game::entities::point::PointEntity;
use crate::scene::scene::Scene;
use crate::termren::ticker::TickCode::{self, Success};
use crate::termren::ticker::Tickable;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::vec;
use termion::terminal_size;

use core::time::Duration;
use std::cmp::max;
use std::io;

use super::input_reader::InputReader;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

trait PushSpace<T> {
    unsafe fn push_space(&mut self);
}

impl PushSpace<char> for String {
    #[inline]
    unsafe fn push_space(&mut self) {
        let len = self.len();
        if self.capacity() == len {
            self.reserve(1);
        }
        let ptr = self.as_mut_vec().as_mut_ptr().add(len);
        core::ptr::write(ptr, ' ' as u8);
        self.as_mut_vec().set_len(len + 1);
    }
}

pub struct Point {
    pos: Vec3,
    color: (u8, u8, u8),
}

pub struct Line {
    pos1: Vec3,
    pos2: Vec3,
    color: (u8, u8, u8),
}

pub enum RenderData<'a> {
    Point(Point),
    Line(Line),
    Mesh(&'a Mesh),
}

impl RenderData<'_> {
    pub fn point(pos: Vec3, color: (u8, u8, u8)) -> RenderData<'static> {
        RenderData::Point(Point { pos, color })
    }
    pub fn line(pos1: Vec3, pos2: Vec3, color: (u8, u8, u8)) -> RenderData<'static> {
        RenderData::Line(Line { pos1, pos2, color })
    }
    pub fn mesh(mesh: &Mesh) -> RenderData {
        RenderData::Mesh(mesh)
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
        //TODO: matrix struct or smth -> implement transparency
        let mut state: Vec<Vec<(u8, u8, u8)>> = vec![vec![(0, 0, 0); width]; height];

        for (id, entity) in self.scene.lock().unwrap().entities() {
            let entity = entity.lock().unwrap();
            if let Some(render_data) = entity.render_data() {
                match render_data {
                    RenderData::Point(point) => {
                        state[point.pos[1] as usize][point.pos[0] as usize] = point.color;
                    }
                    RenderData::Mesh(mesh) => {
                        let tris = mesh.tris();
                        for tri in tris {
                            let verts = tri.verts();
                            let maxy = f64::max(verts[0][1], f64::max(verts[1][1], verts[2][1])) as usize;
                            let miny = f64::min(verts[0][1], f64::min(verts[1][1], verts[2][1])) as usize;
                            let ysum = (verts[0][1] + verts[1][1] + verts[2][1]) as usize;
                            let midy = ysum - maxy - miny;

                            //TODO: get equation of line and fill in with color

                            for i in maxy..midy {
                                for j in 
                            }
                            for i in midy..=miny {}
                        }
                    }
                    RenderData::Line(line) => (),
                }
            }
        }

        //TODO: try to multithread better -> bruhaps create the buffer in a separate thread and print in separate thread and not join

        let mut last_color = state[0][0];
        let mut buffer = format!("\x1b[48;2;{};{};{}m", last_color.0, last_color.1, last_color.2);
        buffer.reserve_exact(width * height);
        for pixel_line in state {
            for pixel in pixel_line {
                if last_color == pixel {
                    unsafe { buffer.push_space() };
                    // buffer.push(' ');
                } else {
                    buffer.push_str(&format!("\x1b[48;2;{};{};{}m ", pixel.0, pixel.1, pixel.2));
                    last_color = pixel;
                }
            }
        }
        let mut out = io::stdout().lock();
        writeln!(out, "{}{}{}\x1b[0mFPS: {}\r", termion::clear::All, termion::cursor::Goto(1, 1), buffer, 1000000000 / delta_time.as_nanos()).unwrap_or(());
        Success
    }
}
