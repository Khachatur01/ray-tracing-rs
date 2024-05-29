use std::ops::Sub;
use std::thread::sleep;
use std::time::Duration;
use crate::intersection::{dot, sphere_intersection};
use crate::screen::Screen;
use crate::vec2d::Vec2d;
use crate::vec3d::Vec3d;

mod screen;
mod vec2d;
mod vec3d;
mod intersection;
mod rotation;

fn main() {
    std::process::Command::new("clear").status().unwrap();

    let mut screen: Screen = Screen::new(110, 28, 8.0 / 15.0);

    for t in 0..100000 {
        let mut light: Vec3d = Vec3d::new(f64::sin(t as f64 * 0.001), f64::cos(t as f64 * 0.001), -1.0);
        light.normalize();

        for i in 0..screen.width {
            for j in 0..screen.height {
                let mut uv: Vec2d = Vec2d::new(i as f64, j as f64) / Vec2d::new(screen.width as f64, screen.height as f64) * 2.0 - 1.0;
                uv.x *= screen.aspect;
                uv.x += f64::sin((t as f64) * 0.001);

                let mut ro: Vec3d = Vec3d::new(-2.0, 0.0, 0.0);
                let mut rd: Vec3d = Vec3d::from_2d(1.0, uv);
                rd.normalize();

                let intersection: Vec2d = sphere_intersection(&ro, &rd, 1.0);

                if intersection.x > 0.0 {
                    let mut point: Vec3d = ro + rd * intersection.x;
                    point.normalize();

                    let diff: f64 = dot(&point, &light);

                    let mut color: f64 = diff * 20.0;

                    if color > 255.0 {
                        color = 255.0;
                    }

                    screen.set_pixel(i, j, color as u8);
                } else {
                    screen.set_pixel(i, j, 0);
                }
            }
        }
        sleep(Duration::new(0, 100000));

        println!("{}", screen.get_screen());
    }
}