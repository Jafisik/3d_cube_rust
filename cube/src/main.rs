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

use crate::objec::{Cube, Objects, Plane, Pyramid, Sphere, Transformable, Triangle};

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
    let mut light_dir = Point3D::new((0 as f64) as f32, (0 as f64) as f32, z_move);
    let mut obj_num: usize = 0;


    let p1 = Point3D::new(-0.7, -0.7, 2.0);
    let p2 = Point3D::new(0.7, 0.7, 4.0);
    let p3 = Point3D::new(0.0, 0.0, 4.0);

    let mut scene: Vec<Objects> = vec![];


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
                            winit::event::VirtualKeyCode::Up => if obj_num < scene.len() {scene[obj_num].rotate(-0.04, 0.0);},
                            winit::event::VirtualKeyCode::Down => if obj_num < scene.len() {scene[obj_num].rotate(0.04, 0.0)},
                            winit::event::VirtualKeyCode::Left => if obj_num < scene.len() {scene[obj_num].rotate(0.0, -0.04)},
                            winit::event::VirtualKeyCode::Right => if obj_num < scene.len() {scene[obj_num].rotate(0.0, 0.04)},
                            winit::event::VirtualKeyCode::Numpad0 => {obj_num = 0; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad1 => {obj_num = 1; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad2 => {obj_num = 2; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad3 => {obj_num = 3; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad4 => {obj_num = 4; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad5 => {obj_num = 5; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad6 => {obj_num = 6; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad7 => {obj_num = 7; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad8 => {obj_num = 8; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Numpad9 => {obj_num = 9; select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::W => { if obj_num < scene.len() {scene[obj_num].move_trans(0.0, -0.01, 0.0)}},
                            winit::event::VirtualKeyCode::S => { if obj_num < scene.len() {scene[obj_num].move_trans(0.0, 0.01, 0.0)}},
                            winit::event::VirtualKeyCode::A => { if obj_num < scene.len() {scene[obj_num].move_trans(-0.01, 0.0, 0.0)}},
                            winit::event::VirtualKeyCode::D => { if obj_num < scene.len() {scene[obj_num].move_trans(0.01, 0.0, 0.0)}},
                            winit::event::VirtualKeyCode::Q => { if obj_num < scene.len() {scene[obj_num].move_trans(0.0, 0.0, -0.01)}},
                            winit::event::VirtualKeyCode::E => { if obj_num < scene.len() {scene[obj_num].move_trans(0.0, 0.0, 0.01)}},
                            winit::event::VirtualKeyCode::C => { scene.push(Objects::Cube(Cube::new(p1, p2, [100,200,50])));   
                                                                    obj_num = scene.len()-1;
                                                                    select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::P => { scene.push(Objects::Plane(Plane::new(p1, p2,[100,0,50])));
                                                                    obj_num = scene.len()-1;
                                                                    select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::T => { scene.push(Objects::Triangle(Triangle::new(p1, p2, p3,[200,200,50])));
                                                                    obj_num = scene.len()-1;
                                                                    select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::Y => { scene.push(Objects::Pyramid(Pyramid::new(p1, p2, p3,[100,200,200])));
                                                                    obj_num = scene.len()-1;
                                                                    select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::O => { scene.push(Objects::Sphere(Sphere::new(p1, 1.0, 8, 16,[100,100,0])));
                                                                    obj_num = scene.len()-1;
                                                                    select_object(obj_num, &mut scene);},
                            winit::event::VirtualKeyCode::R => { if obj_num < scene.len() {scene[obj_num].scale(0.01); }},
                            winit::event::VirtualKeyCode::F => { if obj_num < scene.len() {scene[obj_num].scale(-0.01); }},
                            winit::event::VirtualKeyCode::Escape => { scene[obj_num].deselect();},
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
                light_dir = Point3D::new(-(position.x-(WIDTH/2) as f64) as f32, -(position.y-(HEIGHT/2) as f64) as f32, z_move);
            
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
                objec::draw_cube(obj, light_dir, framebuffer, zbuffer);
            }
            Objects::Plane(obj) => {
                objec::draw_plane(obj, light_dir, framebuffer, zbuffer);
            }
            Objects::Triangle(obj) => {
                objec::draw_triangle(obj, light_dir, framebuffer, zbuffer);
            }
            Objects::Pyramid(obj) => {
                objec::draw_pyramid(obj, light_dir, framebuffer, zbuffer);
            }
            Objects::Sphere(obj) => {
                objec::draw_sphere(obj, light_dir, framebuffer, zbuffer);
            }
        }
    }
}

fn select_object(obj_num: usize, scene: &mut Vec<Objects>){
    for i in 0..scene.len(){
        scene[i].deselect();
    }
    scene[obj_num].select();
}


