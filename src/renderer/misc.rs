use crate::{Mat4, Vec2, Vec3, Vec4};

pub type IndexType = u32;

pub struct Vertex {
    pub p: Vec3,
    pub uv: Vec2,
    pub c: Vec4,
}

impl Vertex {
    pub fn new(p: Vec3,
               uv: Vec2,
               c: Vec4) -> Self {
        Self {
            p,
            uv,
            c,
        }
    }
}

pub struct Setup {
    pub view: Mat4,
    pub proj: Mat4,
    pub mvp: Mat4,
}

impl Setup {
    pub fn new(view: Mat4, proj: Mat4) -> Self {
        Self {
            view,
            proj,
            mvp: proj * view,
        }
    }
}

pub struct Quad {
    pub v00: Vec3,
    pub v01: Vec3,
    pub v10: Vec3,
    pub v11: Vec3,
    pub c00: Vec4,
    pub c01: Vec4,
    pub c10: Vec4,
    pub c11: Vec4,
    pub uv00: Vec2,
    pub uv01: Vec2,
    pub uv10: Vec2,
    pub uv11: Vec2,
}

impl Quad {
    pub fn v4c4(v00: Vec3,
                v01: Vec3,
                v10: Vec3,
                v11: Vec3,
                c00: Vec4,
                c01: Vec4,
                c10: Vec4,
                c11: Vec4) -> Self {
        Self {
            v00,
            v01,
            v10,
            v11,
            c00,
            c01,
            c10,
            c11,
            uv00: Vec2::new(0.0, 0.0),
            uv01: Vec2::new(0.0, 1.0),
            uv10: Vec2::new(1.0, 0.0),
            uv11: Vec2::new(0.0, 1.0),
        }
    }

    pub fn v4c(v00: Vec3,
               v01: Vec3,
               v10: Vec3,
               v11: Vec3, c: Vec4) -> Self {
        Self {
            v00,
            v01,
            v10,
            v11,
            c00: c,
            c01: c,
            c10: c,
            c11: c,
            uv00: Vec2::new(0.0, 0.0),
            uv01: Vec2::new(0.0, 1.0),
            uv10: Vec2::new(1.0, 0.0),
            uv11: Vec2::new(0.0, 1.0),
        }
    }

    pub fn v4(v00: Vec3,
              v01: Vec3,
              v10: Vec3,
              v11: Vec3) -> Self {
        Self::v4c(v00, v01, v10, v11, Vec4::new(1.0, 1.0, 1.0, 1.0))
    }
}