use std::vec;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod operations;
mod drawing;
mod lighting;
mod objec;
mod geometry;
use geometry::*;

use crate::objec::{Cube, Plane, Objects, Triangle, Pyramid, Sphere};


const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("3D Rendering")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    let mut framebuffer = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
    let mut zbuffer = vec![f32::INFINITY; (WIDTH * HEIGHT) as usize];
    let mut z_move = 0.6;
    let mut light_dir = Point3D::new(-(0 as f64) as f32, (0 as f64) as f32, z_move);
    let mut obj_num: usize = 0;

    let start_angle_x = -0.6f32;
    let start_angle_y = 0.4f32;

    let p1 = Point3D::new(-0.7, -0.7, 2.0);
    let p2 = Point3D::new(0.7, 0.7, 4.0);

    let p3 = Point3D::new(0.0, 0.0, 4.0);
    let p4 = Point3D::new(3.7, 3.7, 4.0);

    let p5 = Point3D::new(0.7, 0.7, 5.0);
    let p6 = Point3D::new(3.7, 3.7, 4.0);
    let mut scene: Vec<Objects> = vec![
        //Objects::Cube(Cube {e1:p1, e2:p2, angle_x:start_angle_x, angle_y:start_angle_y}),
        //Objects::Triangle(Triangle {e1:p3, e2:p4, e3:p5, angle_x:start_angle_x, angle_y:start_angle_y}),
        //Objects::Plane(Plane { e1:p5, e2:p6, angle_x:start_angle_x, angle_y:start_angle_y } ),
        //Objects::Pyramid(Pyramid { e1:p1, e2:p2, e3:p3, angle_x:start_angle_x, angle_y:start_angle_y } ),
        Objects::Sphere(Sphere { center: p3, radius: p1, angle_x: start_angle_x, angle_y: start_angle_y} ),
    ];


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
                            winit::event::VirtualKeyCode::Left => scene[obj_num].rotate(-0.04, 0.0),
                            winit::event::VirtualKeyCode::Right => scene[obj_num].rotate(0.04, 0.0),
                            winit::event::VirtualKeyCode::Up => scene[obj_num].rotate(0.0, -0.04),
                            winit::event::VirtualKeyCode::Down => scene[obj_num].rotate(0.0, 0.04),
                            winit::event::VirtualKeyCode::Numpad0 => obj_num = 0,
                            winit::event::VirtualKeyCode::Numpad1 => obj_num = 1,
                            winit::event::VirtualKeyCode::Numpad2 => obj_num = 2,
                            winit::event::VirtualKeyCode::W => scene[obj_num].move_trans(0.0, -0.01, 0.0),
                            winit::event::VirtualKeyCode::S => scene[obj_num].move_trans(0.0, 0.01, 0.0),
                            winit::event::VirtualKeyCode::A => scene[obj_num].move_trans(-0.01, 0.0, 0.0),
                            winit::event::VirtualKeyCode::D => scene[obj_num].move_trans(0.01, 0.0, 0.0),
                            winit::event::VirtualKeyCode::Q => scene[obj_num].move_trans(0.0, 0.0, -0.01),
                            winit::event::VirtualKeyCode::E => scene[obj_num].move_trans(0.0, 0.0, 0.01),
                            _ => {}
                        }
                    }
                    
                    render_scene(&scene, light_dir, &mut framebuffer, &mut zbuffer);
                    window.request_redraw();
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position ,.. },
                ..
            } => {
                light_dir = Point3D::new((position.x-(WIDTH/2+35) as f64) as f32, (position.y-(HEIGHT/2+35) as f64) as f32, z_move);
            
                render_scene(&scene, light_dir, &mut framebuffer, &mut zbuffer);
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

fn render_scene(scene: &Vec<Objects>, light_dir: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]){
    framebuffer.fill(0);
    zbuffer.fill(f32::INFINITY);

    for object in scene{
        match object{
            Objects::Cube(obj) => {
                objec::draw_cube(obj.clone(), light_dir, framebuffer, zbuffer);
            }
            Objects::Plane(obj) => {
                objec::draw_plane(obj.clone(), light_dir, framebuffer, zbuffer);
            }
            Objects::Triangle(obj) => {
                objec::draw_triangle(obj.clone(), light_dir, framebuffer, zbuffer);
            }
            Objects::Pyramid(obj) => {
                objec::draw_pyramid(obj.clone(), light_dir, framebuffer, zbuffer);
            }
            Objects::Sphere(obj) => {
                objec::draw_sphere(obj.clone(), light_dir, framebuffer, zbuffer);
            }
        }
    }
    
}


