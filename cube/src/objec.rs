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
    pub scale: f32,
}

impl Cube{
    pub fn new(e1: Point3D, e2: Point3D) -> Self{
        Cube{
            e1,
            e2,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct Plane{
    pub e1: Point3D,
    pub e2: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
    pub scale: f32,
}

impl Plane{
    pub fn new(e1: Point3D, e2: Point3D) -> Self{
        Plane{
            e1,
            e2,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct Triangle{
    pub e1: Point3D,
    pub e2: Point3D,
    pub e3: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
    pub scale: f32,
}

impl Triangle{
    pub fn new(e1: Point3D, e2: Point3D, e3: Point3D) -> Self{
        Triangle{
            e1,
            e2,
            e3,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct Sphere{
    pub center: Point3D,
    pub radius: f32,
    pub lat_steps: i32,
    pub lon_steps: i32,
    pub angle_x: f32,
    pub angle_y: f32,
    pub scale: f32,
}

impl Sphere{
    pub fn new(center: Point3D, radius: f32, lat_steps: i32, lon_steps: i32) -> Self{
        Sphere{
            center,
            radius,
            lat_steps,
            lon_steps,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct Pyramid{
    pub e1: Point3D,
    pub e2: Point3D,
    pub e3: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
    pub scale: f32,
}

impl Pyramid{
    pub fn new(e1: Point3D, e2: Point3D, e3: Point3D) -> Self{
        Pyramid{
            e1,
            e2,
            e3,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
        }
    }
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
    pub fn scale(&mut self, scale: f32) {
        match self {
            Objects::Cube(cube) => cube.scale(scale),
            Objects::Plane(plane) => plane.scale(scale),
            Objects::Triangle(triangle) => triangle.scale(scale),
            Objects::Pyramid(pyramid) => pyramid.scale(scale),
            Objects::Sphere(sphere) => sphere.scale(scale),
        }
    }
}

pub trait Transformable {
    fn move_trans(&mut self, x: f32, y: f32, z: f32);
    fn rotate(&mut self, angle_x: f32, angle_y: f32);
    fn scale(&mut self, scale_delta: f32);
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
    fn scale(&mut self, scale: f32) {
        self.scale += scale;
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
    fn scale(&mut self, scale: f32) {
        self.scale += scale;
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
    fn scale(&mut self, scale: f32) {
        self.scale += scale;
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
    fn scale(&mut self, scale: f32) {
        self.scale += scale;
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
    fn scale(&mut self, scale: f32) {
        self.scale += scale;
    }
}


pub fn draw_cube(object: Cube, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    let e1 = object.e1;
    let e2 = object.e2;
    let angle_x = object.angle_x;
    let angle_y = object.angle_y;
    
    let side_length = (e1.x-e2.x).abs();
    let center = Point3D::new(e1.x + side_length/2.0, e1.y + side_length/2.0, e1.z + side_length/2.0);

    println!("{}", object.scale);

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p3 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);
    let p4 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);
    let p5 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z + side_length), angle_x, angle_y, object.scale, center);
    let p6 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z + side_length), angle_x, angle_y, object.scale, center);
    let p7 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z + side_length), angle_x, angle_y, object.scale, center);
    let p8 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z + side_length), angle_x, angle_y, object.scale, center);

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

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p3 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);
    let p4 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);

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

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e2.y, e2.z), angle_x, angle_y, object.scale, center);
    let p3 = rotate_and_translate(Point3D::new(e3.x, e3.y, e3.z), angle_x, angle_y, object.scale, center);

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

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p2 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);
    let p3 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p4 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);
    let p5 = rotate_and_translate(Point3D::new(e3.x, e3.y, e3.z), angle_x, angle_y, object.scale, center);

    triangle_3d_fill(Triangle3D::new(p1, p2, p3), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p3, p2, p4), light, framebuffer, zbuffer);

    triangle_3d_fill(Triangle3D::new(p5, p2, p1), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4, p2, p5), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p5, p3, p4), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p5, p1, p3), light, framebuffer, zbuffer);
}

pub fn draw_sphere(object: Sphere,light: Point3D,framebuffer: &mut [u8],zbuffer: &mut [f32]) {
    let angle_x = object.angle_x;
    let angle_y = object.angle_y;
    let center = object.center;
    let r = object.radius;

    for i in 0..object.lat_steps {
        let phi1 = std::f32::consts::PI * i as f32 / object.lat_steps as f32;
        let phi2 = std::f32::consts::PI * (i + 1) as f32 / object.lat_steps as f32;

        for j in 0..object.lon_steps {
            let theta1 = 2.0 * std::f32::consts::PI * j as f32 / object.lon_steps as f32;
            let theta2 = 2.0 * std::f32::consts::PI * (j + 1) as f32 / object.lon_steps as f32;

            let p1 = Point3D::new(
                center.x + r * theta1.cos() * phi1.sin(),
                center.y + r * theta1.sin() * phi1.sin(),
                center.z + r * phi1.cos(),
            );
            let p2 = Point3D::new(
                center.x + r * theta2.cos() * phi1.sin(),
                center.y + r * theta2.sin() * phi1.sin(),
                center.z + r * phi1.cos(),
            );
            let p3 = Point3D::new(
                center.x + r * theta2.cos() * phi2.sin(),
                center.y + r * theta2.sin() * phi2.sin(),
                center.z + r * phi2.cos(),
            );
            let p4 = Point3D::new(
                center.x + r * theta1.cos() * phi2.sin(),
                center.y + r * theta1.sin() * phi2.sin(),
                center.z + r * phi2.cos(),
            );

            let p1 = rotate_and_translate(p1, angle_x, angle_y, object.scale, center);
            let p2 = rotate_and_translate(p2, angle_x, angle_y, object.scale, center);
            let p3 = rotate_and_translate(p3, angle_x, angle_y, object.scale, center);
            let p4 = rotate_and_translate(p4, angle_x, angle_y, object.scale, center);

            triangle_3d_fill(Triangle3D::new(p1, p2, p3), light, framebuffer, zbuffer);
            triangle_3d_fill(Triangle3D::new(p1, p3, p4), light, framebuffer, zbuffer);
        }
    }
}
