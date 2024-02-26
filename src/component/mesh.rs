use super::vec3::Vec3;

#[derive(Clone)]
pub struct Mesh {
    tris: Vec<Tri>,
    color: (u8, u8, u8),
}

impl Mesh {
    pub fn new(tri: Tri) -> Mesh {
        Mesh { color: (0, 0, 0), tris: vec![tri] }
    }
    pub fn tris(&self) -> Vec<Tri> {
        //TODO: sketchy
        self.tris.clone()
    }
}
#[derive(Clone)]
pub struct Tri {
    verts: [Vec3; 3],
}

impl Tri {
    pub fn verts(&self) -> [Vec3; 3] {
        self.verts
    }
}
