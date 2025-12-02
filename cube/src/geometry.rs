use std::ops::Mul;

#[derive(Clone,Copy,Debug)]
pub struct Point{
    pub x: u32,
    pub y: u32,
}


#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D{
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
}

impl Mul<f32> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f32) -> Point3D {
        Point3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

pub struct Triangle3D {
    pub p1: Point3D,
    pub p2: Point3D,
    pub p3: Point3D,
    pub color: [u8; 3],
}

impl Triangle3D {
    pub fn new(p1: Point3D, p2: Point3D, p3: Point3D, color: [u8; 3]) -> Self{
        Self {p1, p2, p3, color}
    }
}


#[derive(Clone,Copy,Debug)]
pub struct Triangle{
    pub p1: Point,
    pub p2: Point,
    pub p3: Point
}

impl Triangle{
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self{
        Self {p1, p2, p3}
    }
}