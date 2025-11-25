use crate::geometry::*;
use crate::HEIGHT;
use crate::WIDTH;

pub fn rotate_and_translate(offset: Point3D, angle_x: f32, angle_y: f32, scale: f32, center: Point3D) -> Point3D {
    let rotated = rotate_x(rotate_y(offset, angle_y, center), angle_x, center);
    let translated = Point3D::new( rotated.x + center.x, rotated.y + center.y, rotated.z + center.z,);
    scale_object(translated, scale, center)
}


pub fn project(p: Point3D) -> Point {
    let fov = 2.0;
    let scale = WIDTH as f32 / (fov * p.z.max(0.01));

    Point {
        x: (p.x * scale + WIDTH as f32 / 2.0) as u32,
        y: (p.y * scale + HEIGHT as f32 / 2.0) as u32,
    }
}


pub fn rotate_y(p: Point3D, angle: f32, center: Point3D) -> Point3D {
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

pub fn rotate_x(p: Point3D, angle: f32, center: Point3D) -> Point3D {
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

pub fn scale_object(p: Point3D, scale: f32, center: Point3D) -> Point3D {
    //println!("{}", center.x + (p.x - center.x) * scale);
    Point3D {
        x: center.x + (p.x - center.x) * scale,
        y: center.y + (p.y - center.y) * scale,
        z: center.z + (p.z - center.z) * scale,
    }
}
