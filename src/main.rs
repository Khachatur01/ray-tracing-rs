use std::env;
use std::ops::Sub;
use std::thread::sleep;
use std::time::Duration;
use crate::intersection::{cube_intersection, dot, plane_intersection, reflect, sphere_intersection};
use crate::screen::Screen;
use crate::vec2d::Vec2d;
use crate::vec3d::Vec3d;

mod screen;
mod vec2d;
mod vec3d;
mod intersection;

fn main() {
    match env::consts::FAMILY {
        "unix" => {
            std::process::Command::new("clear").status().unwrap();
        }
        "windows" => {
            std::process::Command::new("cls").status().unwrap();
        }
        _ => {}
    }

    let mut screen: Screen = Screen::new(236, 64, 8.0 / 15.0);

    for t in 0..10000 {
        let mut light: Vec3d = Vec3d::new(-0.5, 0.5, -1.0);
        light.normalize();

        let sphere_position: Vec3d = Vec3d::new(0.0, 3.0, 0.0);

        for i in 0..screen.width {
            for j in 0..screen.height {
                let mut uv: Vec2d = Vec2d::new(i as f64, j as f64) / Vec2d::new(screen.width as f64, screen.height as f64) * 2.0 - 1.0;
                uv.x *= screen.aspect;

                let mut ro: Vec3d = Vec3d::new(-6.0, 0.0, 0.0);
                let mut rd: Vec3d = Vec3d::from_2d(2.0, uv);
                rd.normalize();

                ro.rotate_z(t as f64 * 0.01);
                rd.rotate_z(t as f64 * 0.01);

                let mut diff: f64 = 1.0;

                for _ in 0..5 {
                    let mut n: Vec3d = Vec3d::new(0.0, 0.0, 0.0);
                    let mut min_it: f64 = 99_999.0;
                    let mut albedo: f64 = 1.0;

                    let intersection: Vec2d = sphere_intersection(&(ro - sphere_position), &rd, 1.0);

                    if intersection.x > 0.0 {
                        let mut point: Vec3d = ro - sphere_position + rd * intersection.x;

                        min_it = intersection.x;

                        point.normalize();
                        n = point;
                    }

                    let mut box_n: Vec3d = Vec3d::new(0.0, 0.0, 0.0);
                    let intersection: Vec2d = cube_intersection(&ro, &rd, Vec3d::new(1.0, 1.0, 1.0), &mut box_n);

                    if intersection.x > 0.0 && intersection.x < min_it {
                        min_it = intersection.x;
                        n = box_n;
                    }

                    let intersection: Vec2d = plane_intersection(ro, rd, Vec3d::new(0.0, 0.0, -1.0), 1.0);

                    if intersection.x > 0.0 && intersection.x < min_it {
                        min_it = intersection.x;
                        n = Vec3d::new(0.0, 0.0, -1.0);
                        albedo = 0.5;
                    }

                    if min_it < 99_999.0 {
                        diff *= (dot(&n, &light) * 0.5 + 0.5) * albedo;
                        ro = ro + rd * (min_it - 0.01);
                        rd = reflect(rd, n);
                    }
                }

                let mut color: f64 = diff * 20.0;
                if color > 255.0 {
                    color = 255.0;
                }

                if diff == 1.0 {
                    color = 0.0;
                }

                screen.set_pixel(i, j, color as u8);
            }
        }
        sleep(Duration::new(0, 5000000));

        println!("{}", screen.get_screen());
    }
}
