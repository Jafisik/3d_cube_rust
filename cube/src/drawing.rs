use crate::geometry::*;
use crate::lighting::*;
use crate::operations::*;
use crate::HEIGHT;
use crate::WIDTH;

pub fn triangle_3d_fill(triangle: Triangle3D, selected: bool, light_dir: Point3D, framebuffer: &mut [u8], zbuffer: &mut [f32]) {
    let intensity = compute_light(&triangle, light_dir);
    let color_gray = shaded_color(intensity, triangle.color, selected);

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

pub fn triangle_fill_z(t: Triangle, z_values: [f32; 3], buffer: &mut [u8], zbuffer: &mut [f32], color: [u8; 4]) {
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

pub fn draw_scanline_z(x1: f64, x2: f64, y: u32, z1: f32, z2: f32, buffer: &mut [u8], zbuffer: &mut [f32], color: [u8; 4]) {
    let (x_start, x_end, z_start, z_end) = if x1 < x2 {
        (x1, x2, z1, z2)
    } else {
        (x2, x1, z2, z1)
    };

    let dx = x_end - x_start;
    if dx == 0.0 { return; }

    for i in 0..=(dx as u32) {
        let x = (x_start + i as f64).clamp(0.0, (WIDTH - 1) as f64) as u32;
        let z = z_start + (i as f32 / dx as f32) * (z_end - z_start);
        set_pixel(buffer, zbuffer, x, y, z, color);
    }
}

pub fn set_pixel(buffer: &mut [u8], zbuffer: &mut [f32], x: u32, y: u32, z: f32, color: [u8; 4]) {
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
/*
pub fn line(x_s: u32, y_s: u32, x_e: u32, y_e: u32, z_s: f32, z_e: f32, buffer: &mut [u8], zbuffer: &mut [f32], color: [u8; 4]) {
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


pub fn triangle_outline(t: Triangle, buffer: &mut [u8], color: [u8; 4]){
    line(t.p1.x, t.p1.y, t.p2.x, t.p2.y, buffer, color);
    line(t.p2.x, t.p2.y, t.p3.x, t.p3.y, buffer, color);
    line(t.p1.x, t.p1.y, t.p3.x, t.p3.y, buffer, color);
}
    */
