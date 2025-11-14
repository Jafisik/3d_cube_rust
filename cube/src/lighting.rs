
use crate::geometry::*;

pub fn compute_light(tri: &Triangle3D, light_dir: Point3D) -> f32 {
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

pub fn shaded_gray(intensity: f32) -> [u8; 4] {
    let boosted = ((intensity - 0.25) * 0.01).clamp(0.1, 1.0);
    let value = (255.0 * boosted) as u8;
    [value, value, value, 255]
}