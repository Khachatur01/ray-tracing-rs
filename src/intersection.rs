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

pub fn cube_intersection(ro: &Vec3d, rd: &Vec3d, size: Vec3d, out_normal: &mut Vec3d) -> Vec2d {
    let mut m: Vec3d = Vec3d::new(1.0, 1.0, 1.0) / rd.clone();
    let n: Vec3d = m * ro.clone();

    let k: Vec3d = m.abs() * size.clone();
    let t1: Vec3d = -n - k;
    let t2: Vec3d = -n + k;

    let t_n: f64 = f64::max(f64::max(t1.x, t1.y), t1.z);
    let t_f: f64 = f64::min(f64::min(t2.x, t2.y), t2.z);

    if t_n > t_f || t_f < 0.0 {
        return Vec2d::new(-1.0, -1.0);
    }

    let yzx: Vec3d = Vec3d::new(t1.y, t1.z, t1.x);
    let zxy: Vec3d = Vec3d::new(t1.z, t1.x, t1.y);

    let result: Vec3d = -rd.sign() * yzx.step(&t1) * zxy.step(&t1);

    out_normal.x = result.x;
    out_normal.y = result.y;
    out_normal.z = result.z;

    Vec2d::new(t_n, t_f)
}

pub fn plane_intersection(ro: Vec3d, rd: Vec3d, p: Vec3d, w: f64) -> Vec2d {
    let coordinate: f64 = -(dot(&ro, &p) + w) / dot(&rd, &p);

    Vec2d::new(coordinate, coordinate)
}

pub fn reflect(rd: Vec3d, n: Vec3d) -> Vec3d {
    rd - n * (2.0 * dot(&n, &rd))
}