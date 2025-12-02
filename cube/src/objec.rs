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
    pub color: [u8; 3],
    pub selected: bool,
}

impl Cube{
    pub fn new(e1: Point3D, e2: Point3D, color: [u8; 3]) -> Self{
        Cube{
            e1,
            e2,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
            color,
            selected: false,
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
    pub color: [u8; 3],
    pub selected: bool,
}

impl Plane{
    pub fn new(e1: Point3D, e2: Point3D, color: [u8; 3]) -> Self{
        Plane{
            e1,
            e2,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
            color,
            selected: false,
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
    pub color: [u8; 3],
    pub selected: bool,
}

impl Triangle{
    pub fn new(e1: Point3D, e2: Point3D, e3: Point3D, color: [u8; 3]) -> Self{
        Triangle{
            e1,
            e2,
            e3,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
            color,
            selected: false,
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
    pub color: [u8; 3],
    pub selected: bool,

}

impl Sphere{
    pub fn new(center: Point3D, radius: f32, lat_steps: i32, lon_steps: i32, color: [u8; 3]) -> Self{
        Sphere{
            center,
            radius,
            lat_steps,
            lon_steps,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
            color,
            selected: false,
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
    pub color: [u8; 3],
    pub selected: bool,
}

impl Pyramid{
    pub fn new(e1: Point3D, e2: Point3D, e3: Point3D, color: [u8; 3]) -> Self{

        Pyramid{
            e1,
            e2,
            e3,
            angle_x: -0.6,
            angle_y: 0.4,
            scale: 1.0,
            color,
            selected: false,
        }
    }
}

impl Transformable for Objects {
    fn move_trans(&mut self, x: f32, y: f32, z: f32) {
        match self {
            Objects::Cube(cube) => cube.move_trans(x, y, z),
            Objects::Plane(plane) => plane.move_trans(x, y, z),
            Objects::Triangle(triangle) => triangle.move_trans(x, y, z),
            Objects::Pyramid(pyramid) => pyramid.move_trans(x, y, z),
            Objects::Sphere(sphere) => sphere.move_trans(x, y, z),
        }
    }
    fn rotate(&mut self, ax: f32, ay: f32) {
        match self {
            Objects::Cube(cube) => cube.rotate(ax, ay),
            Objects::Plane(plane) => plane.rotate(ax, ay),
            Objects::Triangle(triangle) => triangle.rotate(ax, ay),
            Objects::Pyramid(pyramid) => pyramid.rotate(ax, ay),
            Objects::Sphere(sphere) => sphere.rotate(ax, ay),
        }
    }
    fn scale(&mut self, s: f32) {
        match self {
            Objects::Cube(cube) => cube.scale(s),
            Objects::Plane(plane) => plane.scale(s),
            Objects::Triangle(triangle) => triangle.scale(s),
            Objects::Pyramid(pyramid) => pyramid.scale(s),
            Objects::Sphere(sphere) => sphere.scale(s),
        }
    }
    fn select(&mut self) {
        match self {
            Objects::Cube(cube) => cube.select(),
            Objects::Plane(plane) => plane.select(),
            Objects::Triangle(triangle) => triangle.select(),
            Objects::Pyramid(pyramid) => pyramid.select(),
            Objects::Sphere(sphere) => sphere.select(),
        }
    }
    fn deselect(&mut self) {
        match self {
            Objects::Cube(cube) => cube.deselect(),
            Objects::Plane(plane) => plane.deselect(),
            Objects::Triangle(triangle) => triangle.deselect(),
            Objects::Pyramid(pyramid) => pyramid.deselect(),
            Objects::Sphere(sphere) => sphere.deselect(),
        }
    }
}

pub trait Transformable {
    fn move_trans(&mut self, x: f32, y: f32, z: f32);
    fn rotate(&mut self, angle_x: f32, angle_y: f32);
    fn scale(&mut self, scale_delta: f32);
    fn select(&mut self);
    fn deselect(&mut self);
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
    fn select(&mut self) {
        self.selected = true;
    }
    fn deselect(&mut self) {
        self.selected = false;
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
    fn select(&mut self) {
        self.selected = true;
    }
    fn deselect(&mut self) {
        self.selected = false;
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
    fn select(&mut self) {
        self.selected = true;
    }
    fn deselect(&mut self) {
        self.selected = false;
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
    fn select(&mut self) {
        self.selected = true;
    }
    fn deselect(&mut self) {
        self.selected = false;
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
    fn select(&mut self) {
        self.selected = true;
    }
    fn deselect(&mut self) {
        self.selected = false;
    }
}


pub fn draw_cube(object: &Cube, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    let e1 = object.e1;
    let e2 = object.e2;
    let angle_x = object.angle_x;
    let angle_y = object.angle_y;
    
    let side_length = (e1.x-e2.x).abs();
    let center = Point3D::new(e1.x + side_length/2.0, e1.y + side_length/2.0, e1.z + side_length/2.0);

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z), angle_x, angle_y, object.scale, center);
    let p3 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);
    let p4 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z), angle_x, angle_y, object.scale, center);
    let p5 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z + side_length), angle_x, angle_y, object.scale, center);
    let p6 = rotate_and_translate(Point3D::new(e2.x, e1.y, e1.z + side_length), angle_x, angle_y, object.scale, center);
    let p7 = rotate_and_translate(Point3D::new(e2.x, e2.y, e1.z + side_length), angle_x, angle_y, object.scale, center);
    let p8 = rotate_and_translate(Point3D::new(e1.x, e2.y, e1.z + side_length), angle_x, angle_y, object.scale, center);

    //Front
    triangle_3d_fill(Triangle3D::new(p1, p2, p3, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p1, p3, p4, object.color), object.selected, light, framebuffer, zbuffer);
    //Top
    triangle_3d_fill(Triangle3D::new(p6, p2, p1, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p1, p5, p6, object.color), object.selected, light, framebuffer, zbuffer);
    //Left
    triangle_3d_fill(Triangle3D::new(p1, p4, p5, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4,p8, p5, object.color), object.selected, light, framebuffer, zbuffer);
    //Right
    triangle_3d_fill(Triangle3D::new(p7, p3, p2, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p2,p6, p7, object.color), object.selected, light, framebuffer, zbuffer);
    //Bottom
    triangle_3d_fill(Triangle3D::new(p8, p4, p3, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p3,p7, p8, object.color), object.selected, light, framebuffer, zbuffer);
    //Back
    triangle_3d_fill(Triangle3D::new(p7, p6, p5, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p8,p7, p5, object.color), object.selected, light, framebuffer, zbuffer);
}

pub fn draw_plane(object: &Plane, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
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

    triangle_3d_fill(Triangle3D::new(p1, p2, p3, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p1, p3, p4, object.color), object.selected, light, framebuffer, zbuffer);
}

pub fn draw_triangle(object: &Triangle, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
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

    triangle_3d_fill(Triangle3D::new(p3, p2, p1, object.color), object.selected, light, framebuffer, zbuffer);
}

pub fn draw_pyramid(object: &Pyramid, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
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

    //Bottom
    triangle_3d_fill(Triangle3D::new(p3, p2, p1, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4, p2, p3, object.color), object.selected, light, framebuffer, zbuffer);

    //Sides
    triangle_3d_fill(Triangle3D::new(p1, p2, p5, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p5, p2, p4, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4, p3, p5, object.color), object.selected, light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p3, p1, p5, object.color), object.selected, light, framebuffer, zbuffer);
}

pub fn draw_sphere(object: &Sphere,light: Point3D,framebuffer: &mut [u8],zbuffer: &mut [f32]) {
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

            triangle_3d_fill(Triangle3D::new(p1, p2, p3, object.color), object.selected, light, framebuffer, zbuffer);
            triangle_3d_fill(Triangle3D::new(p1, p3, p4, object.color), object.selected, light, framebuffer, zbuffer);
        }
    }
}
