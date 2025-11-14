use crate::geometry::*;
use crate::operations::*;
use crate::drawing::*;


pub enum Objects{
    Cube(Cube),
}

#[derive(Clone)]
pub struct Cube{
    pub e1: Point3D,
    pub e2: Point3D,
    pub angle_x: f32,
    pub angle_y: f32,
}

pub trait Moveable {
    fn move_trans(&mut self, x: f32, y: f32);
}

impl Moveable for Cube{
    fn move_trans(&mut self, x: f32, y: f32) {
        self.e1.x += x;
        self.e2.x += x;

        self.e1.y += y;
        self.e2.y += y;
    }
}

pub trait Rotatable {
    fn rotate(&mut self, angle_x: f32, angle_y: f32);
}

impl Rotatable for Cube{
    fn rotate(&mut self, angle_y: f32, angle_x: f32) {
        self.angle_x += angle_x;
        self.angle_y += angle_y;
    }
}

impl Objects {
    pub fn rotate(&mut self, angle_x: f32, angle_y: f32) {
        match self {
            Objects::Cube(cube) => cube.rotate(angle_x, angle_y),
        }
    }
    pub fn move_trans(&mut self, x: f32, y: f32) {
        match self {
            Objects::Cube(cube) => cube.move_trans(x, y),
        }
    }
}

pub fn draw(cube: Cube, light: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
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

pub fn plane(e1: Point3D, e2: Point3D, light: Point3D, angle_x: f32, angle_y: f32, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    
    let side_length = (e1.x-e2.x).abs();
    let center = Point3D::new(e1.x + side_length/2.0, e1.y + side_length/2.0, e1.z + side_length/2.0);

    let p1 = rotate_and_translate(Point3D::new(e1.x, e1.y, e1.z), angle_x, angle_y, center);
    let p2 = rotate_and_translate(Point3D::new(e2.x, e1.y, (e1.z-e2.z).abs()), angle_x, angle_y, center);
    let p3 = rotate_and_translate(Point3D::new(e1.x, e2.y, (e1.z-e2.z).abs()), angle_x, angle_y, center);
    let p4 = rotate_and_translate(Point3D::new(e2.x, e2.y, e2.z), angle_x, angle_y, center);
    
    triangle_3d_fill(Triangle3D::new(p3, p2, p1), light, framebuffer, zbuffer);
    triangle_3d_fill(Triangle3D::new(p4, p3, p1), light, framebuffer, zbuffer);
}