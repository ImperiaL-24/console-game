use super::vec3::Vec3;

#[derive(Clone)]
pub struct Mesh {
    tris: Vec<Tri>,
}
//todo: pass vec of tris -> bruhaps read a file for mesh data
impl Mesh {
    pub fn new(tris: Vec<Tri>) -> Mesh {
        Mesh { tris }
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
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3) -> Tri {
        Tri { verts: [p1, p2, p3] }
    }
    pub fn verts(&self) -> [Vec3; 3] {
        self.verts.clone()
    }
    pub fn normal(&self) -> Vec3 {

        //TODO: check that this works
        let d = self.verts[1] - self.verts[0];
        let e = self.verts[2] - self.verts[1];
        d.cross(&e).normalize()
    }
}
