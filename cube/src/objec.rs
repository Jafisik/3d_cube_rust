use crate::geometry::*;
use crate::operations::*;
use crate::drawing::*;


pub enum Objects{
    Cube(Cube),
    Plane(Plane),
    Triangle(Triangle),
    Pyramid(Pyramid),
    Sphere(Sphere)
}

#[derive(Clone)]
pub struct Cube{
    pub e1: Point3D,
    pub e2: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
}

#[derive(Clone)]
pub struct Plane{
    pub e1: Point3D,
    pub e2: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
}

#[derive(Clone)]
pub struct Triangle{
    pub e1: Point3D,
    pub e2: Point3D,
    pub e3: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
}

#[derive(Clone)]
pub struct Sphere{
    pub center: Point3D,
    pub radius: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
}

#[derive(Clone)]
pub struct Pyramid{
    pub e1: Point3D,
    pub e2: Point3D,
    pub e3: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
}

impl Objects {
    pub fn rotate(&mut self, angle_x: f32, angle_y: f32) {
        match self {
            Objects::Cube(cube) => cube.rotate(angle_x, angle_y),
            Objects::Plane(plane) => plane.rotate(angle_x, angle_y),
            Objects::Triangle(triangle) => triangle.rotate(angle_x, angle_y),
            Objects::Pyramid(pyramid) => pyramid.rotate(angle_x, angle_y),
            Objects::Sphere(sphere) => sphere.rotate(angle_x, angle_y),
        }
    }
    pub fn move_trans(&mut self, x: f32, y: f32, z: f32) {
        match self {
            Objects::Cube(cube) => cube.move_trans(x, y, z),
            Objects::Plane(plane) => plane.move_trans(x, y, z),
            Objects::Triangle(triangle) => triangle.move_trans(x, y, z),
            Objects::Pyramid(pyramid) => pyramid.move_trans(x, y, z),
            Objects::Sphere(sphere) => sphere.move_trans(x, y, z),
        }
    }
}

pub trait Transformable {
    fn move_trans(&mut self, x: f32, y: f32, z: f32);
    fn rotate(&mut self, angle_x: f32, angle_y: f32);
}

impl Transformable for Cube {
    fn move_trans(&mut self, x: f32, y: f32, z: f32) {
        for p in [&mut self.e1, &mut self.e2] {
            p.x += x;
            p.y += y;
            p.z += z;
        }
    }
    fn rotate(&mut self, ax: f32, ay: f32) {
        self.angle_x += ax;
        self.angle_y += ay;
    }
}

impl Transformable for Plane {
    fn move_trans(&mut self, x: f32, y: f32, z: f32) {
        for p in [&mut self.e1, &mut self.e2] {
            p.x += x;
            p.y += y;
            p.z += z;
        }
    }
    fn rotate(&mut self, ax: f32, ay: f32) {
        self.angle_x += ax;
        self.angle_y += ay;
    }
}

impl Transformable for Triangle {
    fn move_trans(&mut self, x: f32, y: f32, z: f32) {
        for p in [&mut self.e1, &mut self.e2, &mut self.e3] {
            p.x += x;
            p.y += y;
            p.z += z;
        }
    }
    fn rotate(&mut self, ax: f32, ay: f32) {
        self.angle_x += ax;
        self.angle_y += ay;
    }
}


impl Transformable for Sphere {
    fn move_trans(&mut self, x: f32, y: f32, z: f32) {
        for p in [&mut self.center] {
            p.x += x;
            p.y += y;
            p.z += z;
        }
    }
    fn rotate(&mut self, ax: f32, ay: f32) {
        self.angle_x += ax;
        self.angle_y += ay;
    }
}

impl Transformable for Pyramid {
    fn move_trans(&mut self, x: f32, y: f32, z: f32) {
        for p in [&mut self.e1, &mut self.e2, &mut self.e3] {
            p.x += x;
            p.y += y;
            p.z += z;
        }
    }
    fn rotate(&mut self, ax: f32, ay: f32) {
        self.angle_x += ax;
        self.angle_y += ay;
    }
}


pub fn draw_cube(cube: Cube, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    let e1 = cube.e1;
    let e2 = cube.e2;
    let angle_x = cube.angle_x;
    let angle_y = cube.angle_y;
    
    let side_length = (e1.x-e2.x).abs();
    let center = Point3D::new(e1.x + side_length/2.0, e1.y + side_length/2.0, e1.z + side_length/2.0);

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z), angle_x, angle_y, center);
    let p3 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z), angle_x, angle_y, center);
    let p4 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z), angle_x, angle_y, center);
    let p5 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z + side_length), angle_x, angle_y, center);
    let p6 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z + side_length), angle_x, angle_y, center);
    let p7 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z + side_length), angle_x, angle_y, center);
    let p8 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z + side_length), angle_x, angle_y, center);

    //Front
    triangle_3d_fill(Triangle3D::new(p3, p2, p1), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4, p3, p1), light, framebuffer, zbuffer);
    //Top
    triangle_3d_fill(Triangle3D::new(p1, p2, p6), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p6, p5, p1), light, framebuffer, zbuffer);
    //Left
    triangle_3d_fill(Triangle3D::new(p5, p4, p1), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p5,p8, p4), light, framebuffer, zbuffer);
    //Right
    triangle_3d_fill(Triangle3D::new(p2, p3, p7), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p7,p6, p2), light, framebuffer, zbuffer);
    //Bottom
    triangle_3d_fill(Triangle3D::new(p3, p4, p8), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p8,p7, p3), light, framebuffer, zbuffer);
    //Back
    triangle_3d_fill(Triangle3D::new(p5, p6, p7), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p5,p7, p8), light, framebuffer, zbuffer);
}

pub fn draw_plane(object: Plane, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    let e1 = object.e1;
    let e2 = object.e2;
    let angle_x = object.angle_x;
    let angle_y = object.angle_y;
    
    let side_length = (e1.x-e2.x).abs();
    let center = Point3D::new(e1.x + side_length/2.0, e1.y + side_length/2.0, e1.z);

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z), angle_x, angle_y, center);
    let p3 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z), angle_x, angle_y, center);
    let p4 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z), angle_x, angle_y, center);

    //Front
    triangle_3d_fill(Triangle3D::new(p3, p2, p1), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4, p3, p1), light, framebuffer, zbuffer);
}

pub fn draw_triangle(object: Triangle, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    let e1 = object.e1;
    let e2 = object.e2;
    let e3 = object.e3;
    let angle_x = object.angle_x;
    let angle_y = object.angle_y;
    
    let center = Point3D::new(
        (e1.x + e2.x + e3.x) / 3.0,
        (e1.y + e2.y + e3.y) / 3.0,
        (e1.z + e2.z + e3.z) / 3.0,
    );

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e2.y, e2.z), angle_x, angle_y, center);
    let p3 = rotate_and_translate(Point3D::new(e3.x, e3.y, e3.z), angle_x, angle_y, center);

    triangle_3d_fill(Triangle3D::new(p1, p2, p3), light, framebuffer, zbuffer);
}

pub fn draw_pyramid(object: Pyramid, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    let e1 = object.e1;
    let e2 = object.e2;
    let e3 = object.e3;
    let angle_x = object.angle_x;
    let angle_y = object.angle_y;
    
    let side_length = (e1.x-e2.x).abs();
    let center = Point3D::new(e1.x + side_length/2.0, e1.y + side_length/2.0, e1.z);

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, center);
    let p2 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z), angle_x, angle_y, center);
    let p3 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z), angle_x, angle_y, center);
    let p4 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z), angle_x, angle_y, center);
    let p5 = rotate_and_translate(Point3D::new(e3.x, e3.y, e3.z), angle_x, angle_y, center);

    triangle_3d_fill(Triangle3D::new(p1, p2, p3), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p3, p2, p4), light, framebuffer, zbuffer);

    triangle_3d_fill(Triangle3D::new(p5, p2, p1), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4, p2, p5), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p5, p3, p4), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p5, p1, p3), light, framebuffer, zbuffer);
}

pub fn draw_sphere(object: Sphere, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    let e1 = object.center;
    let angle_x = object.angle_x;
    let angle_y = object.angle_y;
    
    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, object.center);
}