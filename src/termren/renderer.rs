use crate::component::mesh::Mesh;
use crate::component::transform_matrix::TransformMatrix;
use crate::component::vec3::Vec3;
use crate::scene::scene::Scene;
use crate::termren::ticker::TickCode::{self, Success};
use crate::termren::ticker::Tickable;   
use termion::terminal_size;
use crate::component::range::inv_range;
use core::time::Duration;
use std::io;

use super::input_reader::InputReader;
use super::line_render::LineRenderer;
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

pub struct Line {
    pos1: Vec3,
    pos2: Vec3,
}
pub enum RenderPrimitive<'a> {
    Point(Vec3),
    Line(Line),
    Mesh(&'a Mesh),
}

pub struct RenderData<'a> {
    matrix: TransformMatrix,
    primitive: RenderPrimitive<'a>,
    //TODO: materials
    material: (u8,u8,u8)
}

impl RenderData<'_> {
    pub fn new(matrix: TransformMatrix, primitive: RenderPrimitive, material: (u8,u8,u8)) -> RenderData<'_>{
        RenderData{ material, matrix, primitive }
    }
}

//TODO: render data -> material + render primitive + transform matrix

impl RenderPrimitive<'_> {
    pub fn point(pos: Vec3) -> RenderPrimitive<'static> {
        RenderPrimitive::Point(pos)
    }
    pub fn line(pos1: Vec3, pos2: Vec3) -> RenderPrimitive<'static> {
        RenderPrimitive::Line(Line { pos1, pos2 })
    }
    pub fn mesh(mesh: &Mesh) -> RenderPrimitive {
        RenderPrimitive::Mesh(mesh)
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
        
        let scene = self.scene.lock().unwrap();
        let binding = scene.active_camera();
        let camera = binding.lock().unwrap();
        let mut verts: [(i32, i32);3] = [(0,0);3];
        for (id, entity) in scene.entities() {
            let entity = entity.lock().unwrap();
            if let Some(render_data) = entity.render_data() {
                match render_data.primitive {
                    RenderPrimitive::Point(point) => {
                        state[point[1] as usize][point[0] as usize] = render_data.material;
                    }
                    RenderPrimitive::Mesh(mesh) => {
                        //TODO: BETTER TYPES IN EVERYTHING
                        let tris = mesh.tris();
                        for tri in tris {
                            if camera.occluded(&tri,render_data.matrix.clone()) {
                                continue;
                            }
                            let space_verts: [Vec3;3] = tri.verts();
                            for i in 0..3 {
                                let t_vert = render_data.matrix*space_verts[i];
                                verts[i] = camera.project(&t_vert);
                            }
                            verts.sort_by(|a,b| a.1.cmp(&b.1));

                            let l20 = LineRenderer::new(verts[2], verts[0]);
                            let l21 = LineRenderer::new(verts[2], verts[1]);
                            let l10 = LineRenderer::new(verts[1], verts[0]);

                            //TODO: multithread?
                            for i in verts[1].1..=verts[2].1 {
                                if i < 0 || i >= (size.1-2).into() {
                                    continue;
                                } 
                                for j in inv_range(l21.end(i),l20.end(i)) {
                                    if j<0 || j >= size.0.into() {
                                        continue;
                                    }
                                    //TODO: instead of a material get a fragment shader going on here!
                                    state[i as usize][j as usize] = render_data.material;
                                }
                            }
                            for i in verts[0].1..=verts[1].1 {
                                if i < 0 || i >= (size.1-2).into() {
                                    continue;
                                } 
                                for j in inv_range(l20.start(i),l10.end(i))  {
                                    if j<0 || j >= size.0.into() {
                                        continue;
                                    }
                                    state[i as usize][j as usize] = render_data.material;
                                }
                            }
                        }
                    }
                    RenderPrimitive::Line(line) => (),
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
        writeln!(out, "{}{}\x1b[0mW: {} H: {} FPS: {} CAMFL: {} VERTS: {:?}{:?}{:?}\r", termion::cursor::Goto(1, 1), buffer,size.0, size.1, 1000000000 / delta_time.as_nanos(), camera.focal_length(), verts[0],verts[1],verts[2]).unwrap_or(() );
        Success
    }
}
