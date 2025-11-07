use std::{cmp::max, thread::sleep, time::Duration};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Clone,Copy,Debug)]
struct Point{
    x: u32,
    y: u32,
}

impl Point{
    fn new(x: u32, y: u32) -> Self {
        Self {x, y}
    }
}

#[derive(Clone, Copy, Debug)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3D{
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
}

struct Triangle3D {
    p1: Point3D,
    p2: Point3D,
    p3: Point3D,
}

impl Triangle3D {
    fn new(p1: Point3D, p2: Point3D, p3: Point3D) -> Self{
        Self {p1, p2, p3}
    }
}


#[derive(Clone,Copy,Debug)]
struct Triangle{
    p1: Point,
    p2: Point,
    p3: Point
}

impl Triangle{
    fn new(p1: Point, p2: Point, p3: Point) -> Self{
        Self {p1, p2, p3}
    }
}


const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Klikací pixely")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    let mut framebuffer = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
    let mut z_move = 1.5;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        z_move += y * 0.1;
                        z_move = z_move.clamp(0.1, 10.0);
                    }
                    _ => {}
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position ,.. },
                ..
            } => {
                framebuffer.fill(0);

                let light_dir = Point3D::new(-(position.x-160 as f64) as f32, (position.y-120 as f64) as f32, z_move);
                println!("{}", z_move);
                /* 
                let t3d = Triangle3D::new(Point3D::new(20.0, 15.0, 3.0), 
                                        Point3D::new(5.2, -30.2, 3.5), 
                                        Point3D::new(-5.0, 0.0, 4.0));

                triangle_3d_fill(t3d, light_dir, &mut framebuffer);

                let t3d = Triangle3D::new(Point3D::new(20.0, 15.0, 3.0), 
                                                    Point3D::new(-40.2, 0.2, 2.5), 
                                                    Point3D::new(-5.0, 0.0, 4.0));
                
                triangle_3d_fill(t3d, light_dir, &mut framebuffer);

                let t3d = Triangle3D::new(Point3D::new(20.0, 15.0, 3.0), 
                                                    Point3D::new(-40.2, 0.2, 2.5), 
                                                    Point3D::new(-5.0, 30.0, 4.0));
                
                triangle_3d_fill(t3d, light_dir, &mut framebuffer);
                */

                let p1 = Point3D::new(-20.0, -20.0, 3.2);
                let p2 = Point3D::new(20.0, -20.0, 3.0);
                let p3 = Point3D::new(20.0, 20.0, 3.0);
                let p4 = Point3D::new(-20.0, 20.0, 3.2);
                let p5 = Point3D::new(60.0, -40.0, 4.0);
                let p6 = Point3D::new(60.0, 10.0, 4.0);
                let p7 = Point3D::new(0.0, -40.0, 4.2);


                // Dva trojúhelníky tvořící čtverec
                triangle_3d_fill(Triangle3D::new(p5, p3, p2), light_dir, &mut framebuffer);
                triangle_3d_fill(Triangle3D::new(p5, p6, p3), light_dir, &mut framebuffer);
                triangle_3d_fill(Triangle3D::new(p1, p2, p7), light_dir, &mut framebuffer);
                triangle_3d_fill(Triangle3D::new(p2, p5, p7), light_dir, &mut framebuffer);
                triangle_3d_fill(Triangle3D::new(p1, p2, p3), light_dir, &mut framebuffer);
                triangle_3d_fill(Triangle3D::new(p1, p3, p4), light_dir, &mut framebuffer);
                

                window.request_redraw();
            }

            Event::RedrawRequested(_) => {
                pixels.frame_mut().copy_from_slice(&framebuffer);
                pixels.render().unwrap();
            }
 
            _ => {}
        }
    });
}

fn set_pixel(buffer: &mut [u8], x: u32, y: u32, color: [u8; 4]) {
    if x > WIDTH || y > HEIGHT{
        return
    }
    let i = ((y * WIDTH + x) * 4) as usize;
    if i + 3 < buffer.len() {
        buffer[i..i + 4].copy_from_slice(&color);
    }
}

fn line(x_s: u32, y_s: u32, x_e: u32, y_e: u32, buffer: &mut [u8], color: [u8; 4]){
    let mut x: f64 = x_s as f64;
    let mut y: f64 = y_s as f64;

    let dx = x_s.abs_diff(x_e) as f64;
    let dy = y_s.abs_diff(y_e) as f64;
    let max = if dx > dy {dx} else {dy};

    let diff_x: f64 = (x_e as i32 - x_s as i32) as f64/max;
    let diff_y: f64 = (y_e as i32 - y_s as i32) as f64/max;

    for _ in 0..max as i32{
        set_pixel(buffer, x as u32, y as u32, color);
        x = x + diff_x;
        y = y + diff_y;
    }
}

fn triangle_outline(t: Triangle, buffer: &mut [u8], color: [u8; 4]){
    line(t.p1.x, t.p1.y, t.p2.x, t.p2.y, buffer, color);
    line(t.p2.x, t.p2.y, t.p3.x, t.p3.y, buffer, color);
    line(t.p1.x, t.p1.y, t.p3.x, t.p3.y, buffer, color);
}

fn triangle_fill(t: Triangle, buffer: &mut [u8], color: [u8; 4]) {
    let mut points = [t.p1, t.p2, t.p3];
    points.sort_by_key(|p| p.y);
    let (v0, v1, v2) = (points[0], points[1], points[2]);

    let first_diff = v1.y.saturating_sub(v0.y);
    let second_diff = v2.y.saturating_sub(v1.y);

    if first_diff > 0 {
        for i in 0..first_diff {
            let dy1 = v1.y.saturating_sub(v0.y) as f64;
            let dy2 = v2.y.saturating_sub(v0.y) as f64;
            if dy1 == 0.0 || dy2 == 0.0 { continue; }

            let y_percent1 = i as f64 / dy1;
            let y_percent2 = i as f64 / dy2;

            let x_edge1 = (v0.x as f64 + y_percent1 * (v1.x as f64 - v0.x as f64)).clamp(0.0, (WIDTH - 1) as f64) as u32;
            let x_edge2 = (v0.x as f64 + y_percent2 * (v2.x as f64 - v0.x as f64)).clamp(0.0, (WIDTH - 1) as f64) as u32;

            let y = v0.y + i;
            if y < HEIGHT {
                line(x_edge1, y, x_edge2, y, buffer, color);
            }
        }
    }

    if second_diff > 0 {
        for i in 0..second_diff {
            let dy1 = v2.y.saturating_sub(v1.y) as f64;
            let dy2 = v2.y.saturating_sub(v0.y) as f64;
            if dy1 == 0.0 || dy2 == 0.0 { continue; }

            let y_percent1 = i as f64 / dy1;
            let y_percent2 = (v1.y + i - v0.y) as f64 / dy2;

            let x_edge1 = (v1.x as f64 + y_percent1 * (v2.x as f64 - v1.x as f64)).clamp(0.0, (WIDTH - 1) as f64) as u32;
            let x_edge2 = (v0.x as f64 + y_percent2 * (v2.x as f64 - v0.x as f64)).clamp(0.0, (WIDTH - 1) as f64) as u32;

            let y = v1.y + i;
            if y < HEIGHT {
                line(x_edge1, y, x_edge2, y, buffer, color);
            }
        }
    }
}


fn project(p: Point3D) -> Point {
    let fov = 90.0; // čím větší, tím menší perspektiva
    let scale = WIDTH as f32 / (fov * p.z.max(0.01)); // ochrana proti dělení nulou

    Point {
        x: (p.x * scale + WIDTH as f32 / 2.0) as u32,
        y: (p.y * scale + HEIGHT as f32 / 2.0) as u32,
    }
}

fn compute_light(tri: &Triangle3D, light_dir: Point3D) -> f32 {
    let u = Point3D {
        x: tri.p2.x - tri.p1.x,
        y: tri.p2.y - tri.p1.y,
        z: tri.p2.z - tri.p1.z,
    };
    let v = Point3D {
        x: tri.p3.x - tri.p1.x,
        y: tri.p3.y - tri.p1.y,
        z: tri.p3.z - tri.p1.z,
    };

    let normal = Point3D {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };

    let len = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    let norm = Point3D {
        x: normal.x / len,
        y: normal.y / len,
        z: normal.z / len,
    };

    let dot = norm.x * light_dir.x + norm.y * light_dir.y + norm.z * light_dir.z;

    dot.max(0.0)
}

fn shaded_gray(intensity: f32) -> [u8; 4] {
    let boosted = ((intensity - 0.25) * 0.2).clamp(0.1, 1.0);
    let value = (255.0 * boosted) as u8;
    [value, value, value, 255]
}

fn triangle_3d_fill(triangle: Triangle3D, light_dir: Point3D, framebuffer: &mut [u8]){

    let intensity = compute_light(&triangle, light_dir);
    let color_gray = shaded_gray(intensity);

    let t = Triangle::new(project(triangle.p1), 
                                    project(triangle.p2), 
                                    project(triangle.p3));
    triangle_fill(t.clone(), framebuffer, color_gray);
}

