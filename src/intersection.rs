use crate::vec2d::Vec2d;
use crate::vec3d::Vec3d;

pub fn dot(vec1: &Vec3d, vec2: &Vec3d) -> f64 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
}

pub fn sphere_intersection(ro: &Vec3d, rd: &Vec3d, radius: f64) -> Vec2d {
    let b: f64 = dot(ro, rd);
    let c: f64 = dot(ro, ro) - radius * radius;
    let mut h: f64 = b * b - c;

    if h < 0.0 {
        return Vec2d::new(-1.0, -1.0);
    }

    h = f64::sqrt(h);

    Vec2d::new(-b - h, -b + h)
}
