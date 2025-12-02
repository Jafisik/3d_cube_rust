
use crate::geometry::*;

impl Point3D {
    pub fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn dot(self, other: Point3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(self) -> Point3D {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Point3D {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

pub fn compute_light(tri: &Triangle3D, light_pos: Point3D) -> f32 {
    let u = tri.p2.sub(tri.p1);
    let v = tri.p3.sub(tri.p1);

    let normal = u.cross(v).normalize();

    let center = Point3D {
        x: (tri.p1.x + tri.p2.x + tri.p3.x) / 3.0,
        y: (tri.p1.y + tri.p2.y + tri.p3.y) / 3.0,
        z: (tri.p1.z + tri.p2.z + tri.p3.z) / 3.0,
    };

    let light_dir = (light_pos.sub(center)).normalize();

    normal.dot(light_dir).clamp(0.1, 1.0)
}

/* 
pub fn shaded_gray(intensity: f32) -> [u8; 4] {
    let value = (255.0 * intensity.clamp(0.1, 1.0)) as u8;
    [value, value, value, 255]
}
*/

pub fn shaded_color(intensity: f32, base_color: [u8; 3], selected: bool) -> [u8; 4] {
    let mut factor = intensity.clamp(0.2, 1.0);

    if selected {
        factor = (factor * 1.8).min(1.0);
    }

    [
        (base_color[0] as f32 * factor) as u8,
        (base_color[1] as f32 * factor) as u8,
        (base_color[2] as f32 * factor) as u8,
        255,
    ]
}
