use std::ops;
use crate::vec2d::Vec2D;

pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        return Vec3d { x, y, z };
    }
    pub fn from_2d(x: f64, vec2d: Vec2D) -> Vec3d {
        return Vec3d { x, y: vec2d.x, z: vec2d.y };
    }
}

impl ops::Neg for Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Vec3d {
        return Vec3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn add(self, _rhs: Vec3d) -> Vec3d {
        return Vec3d {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        };
    }
}

impl ops::Add<f64> for Vec3d {
    type Output = Vec3d;

    fn add(self, _rhs: f64) -> Vec3d {
        return Vec3d {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
        };
    }
}


impl ops::Sub<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn sub(self, _rhs: Vec3d) -> Vec3d {
        return Vec3d {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        };
    }
}

impl ops::Sub<f64> for Vec3d {
    type Output = Vec3d;

    fn sub(self, _rhs: f64) -> Vec3d {
        return Vec3d {
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs,
        };
    }
}


impl ops::Mul<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn mul(self, _rhs: Vec3d) -> Vec3d {
        return Vec3d {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        };
    }
}

impl ops::Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, _rhs: f64) -> Vec3d {
        return Vec3d {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        };
    }
}


impl ops::Div<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn div(self, _rhs: Vec3d) -> Vec3d {
        return Vec3d {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        };
    }
}

impl ops::Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, _rhs: f64) -> Vec3d {
        return Vec3d {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        };
    }
}
