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
    let mut zbuffer = vec![f32::INFINITY; (WIDTH * HEIGHT) as usize];
    let mut z_move = 0.6;
    let mut angle_x = -0.6f32;
    let mut angle_y = 0.4f32;
    let mut light_dir = Point3D::new(-(0 as f64) as f32, (0 as f64) as f32, z_move);

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
                        z_move = z_move.clamp(-10.0, 10.0);
                    }
                    _ => {}
                }
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
                } => {
                if let Some(keycode) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        match keycode {
                            winit::event::VirtualKeyCode::Left => angle_x -= 0.04,
                            winit::event::VirtualKeyCode::Right => angle_x += 0.04,
                            winit::event::VirtualKeyCode::Up => angle_y -= 0.04,
                            winit::event::VirtualKeyCode::Down => angle_y += 0.04,
                            _ => {}
                        }
                    }
                    framebuffer.fill(0);
                    zbuffer.fill(f32::INFINITY);

                    let p1 = Point3D::new(-1.0, -1.0, 2.0);
                    let p2 = Point3D::new(1.0, 1.0, 4.0);

                    cube(p1, p2, light_dir, angle_y, angle_x, &mut framebuffer, &mut zbuffer);

                    window.request_redraw();
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position ,.. },
                ..
            } => {
                light_dir = Point3D::new((position.x-(160+35) as f64) as f32, (position.y-(120+30) as f64) as f32, z_move);
            
                //println!("x: {}, y: {}, z: {}", light_dir.x, light_dir.y, light_dir.z);
                framebuffer.fill(0);
                zbuffer.fill(f32::INFINITY);
                
                let p1 = Point3D::new(-1.0, -1.0, 2.0);
                let p2 = Point3D::new(1.0, 1.0, 2.0);

                cube(p1, p2, light_dir, angle_y, angle_x, &mut framebuffer, &mut zbuffer);

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

fn set_pixel(buffer: &mut [u8], zbuffer: &mut [f32], x: u32, y: u32, z: f32, color: [u8; 4]) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }
    let i = (y * WIDTH + x) as usize;
    if z < zbuffer[i] {
        zbuffer[i] = z;
        let pixel_i = i * 4;
        buffer[pixel_i..pixel_i + 4].copy_from_slice(&color);
    }
}

fn line(x_s: u32, y_s: u32, x_e: u32, y_e: u32, z_s: f32, z_e: f32, buffer: &mut [u8], zbuffer: &mut [f32], color: [u8; 4]) {
    let mut x = x_s as f64;
    let mut y = y_s as f64;
    let dx = x_s.abs_diff(x_e) as f64;
    let dy = y_s.abs_diff(y_e) as f64;
    let max = dx.max(dy);
    let diff_x = (x_e as i32 - x_s as i32) as f64 / max;
    let diff_y = (y_e as i32 - y_s as i32) as f64 / max;
    let diff_z = (z_e - z_s) / max as f32;
    let mut z = z_s;

    for _ in 0..max as i32 {
        set_pixel(buffer, zbuffer, x as u32, y as u32, z, color);
        x += diff_x;
        y += diff_y;
        z += diff_z;
    }
}

/* 
fn triangle_outline(t: Triangle, buffer: &mut [u8], color: [u8; 4]){
    line(t.p1.x, t.p1.y, t.p2.x, t.p2.y, buffer, color);
    line(t.p2.x, t.p2.y, t.p3.x, t.p3.y, buffer, color);
    line(t.p1.x, t.p1.y, t.p3.x, t.p3.y, buffer, color);
}
*/

fn rotate_and_translate(offset: Point3D, angle_x: f32, angle_y: f32, center: Point3D) -> Point3D {
    let rotated = rotate_x(rotate_y(offset, angle_y, center), angle_x, center);
    Point3D::new(
        rotated.x + center.x,
        rotated.y + center.y,
        rotated.z + center.z,
    )
}


fn triangle_fill_z(t: Triangle, z_values: [f32; 3], buffer: &mut [u8], zbuffer: &mut [f32], color: [u8; 4]) {
    let mut points = [(t.p1, z_values[0]), (t.p2, z_values[1]), (t.p3, z_values[2])];
    points.sort_by_key(|(p, _)| p.y);
    let ((v0, z0), (v1, z1), (v2, z2)) = (points[0], points[1], points[2]);

    let first_diff = v1.y.saturating_sub(v0.y);
    let second_diff = v2.y.saturating_sub(v1.y);

    if first_diff > 0 {
        for i in 0..first_diff {
            let dy1 = v1.y.saturating_sub(v0.y) as f64;
            let dy2 = v2.y.saturating_sub(v0.y) as f64;
            if dy1 == 0.0 || dy2 == 0.0 { continue; }

            let y_percent1 = i as f64 / dy1;
            let y_percent2 = i as f64 / dy2;

            let x1 = v0.x as f64 + y_percent1 * (v1.x as f64 - v0.x as f64);
            let x2 = v0.x as f64 + y_percent2 * (v2.x as f64 - v0.x as f64);
            let z1 = z0 + y_percent1 as f32 * (z1 - z0);
            let z2 = z0 + y_percent2 as f32 * (z2 - z0);

            let y = v0.y + i;
            if y < HEIGHT {
                draw_scanline_z(x1, x2, y, z1, z2, buffer, zbuffer, color);
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

            let x1 = v1.x as f64 + y_percent1 * (v2.x as f64 - v1.x as f64);
            let x2 = v0.x as f64 + y_percent2 * (v2.x as f64 - v0.x as f64);
            let z1 = z1 + y_percent1 as f32 * (z2 - z1);
            let z2 = z0 + y_percent2 as f32 * (z2 - z0);

            let y = v1.y + i;
            if y < HEIGHT {
                draw_scanline_z(x1, x2, y, z1, z2, buffer, zbuffer, color);
            }
        }
    }
}

fn draw_scanline_z(x1: f64, x2: f64, y: u32, z1: f32, z2: f32, buffer: &mut [u8], zbuffer: &mut [f32], color: [u8; 4]) {
    let (x_start, x_end, z_start, z_end) = if x1 < x2 {
        (x1, x2, z1, z2)
    } else {
        (x2, x1, z2, z1)
    };

    let dx = x_end - x_start;
    if dx == 0.0 { return; }

    for i in 0..(dx as u32) {
        let x = (x_start + i as f64).clamp(0.0, (WIDTH - 1) as f64) as u32;
        let z = z_start + (i as f32 / dx as f32) * (z_end - z_start);
        set_pixel(buffer, zbuffer, x, y, z, color);
    }
}


fn project(p: Point3D) -> Point {
    let fov = 2.0; // čím větší, tím menší perspektiva
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
    let boosted = ((intensity - 0.25) * 0.01).clamp(0.1, 1.0);
    let value = (255.0 * boosted) as u8;
    [value, value, value, 255]
}

fn triangle_3d_fill(triangle: Triangle3D, light_dir: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]) {
    let intensity = compute_light(&triangle, light_dir);
    let color_gray = shaded_gray(intensity);

    let p1 = triangle.p1;
    let p2 = triangle.p2;
    let p3 = triangle.p3;

    let t = Triangle::new(
        project(p1),
        project(p2),
        project(p3),
    );

    triangle_fill_z(t, [p1.z, p2.z, p3.z], framebuffer, zbuffer, color_gray);
}



fn rotate_y(p: Point3D, angle: f32, center: Point3D) -> Point3D {
    let sin = angle.sin();
    let cos = angle.cos();
    let dx = p.x - center.x;
    let dz = p.z - center.z;

    Point3D {
        x: dx * cos - dz * sin + center.x,
        y: p.y,
        z: dx * sin + dz * cos + center.z,
    }
}

fn rotate_x(p: Point3D, angle: f32, center: Point3D) -> Point3D {
    let sin = angle.sin();
    let cos = angle.cos();
    let dy = p.y - center.y;
    let dz = p.z - center.z;

    Point3D {
        x: p.x,
        y: dy * cos - dz * sin + center.y,
        z: dy * sin + dz * cos + center.z,
    }
}

fn cube(e1: Point3D, e2: Point3D, light: Point3D, angle_x: f32, angle_y: f32, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    
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

