use std::ops;
use crate::vec2d::Vec2d;

#[derive(Clone, Copy)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        return Vec3d { x, y, z };
    }
    pub fn from_2d(x: f64, vec2d: Vec2d) -> Vec3d {
        return Vec3d { x, y: vec2d.x, z: vec2d.y };
    }

    pub fn normalize(&mut self) {
        let length: f64 = self.len();

        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn abs(&self) -> Vec3d {
        return Vec3d {
            x: f64::abs(self.x),
            y: f64::abs(self.y),
            z: f64::abs(self.z),
        };
    }

    pub fn sign(&self) -> Vec3d {
        return Vec3d {
            x: Self::sign_n(self.x),
            y: Self::sign_n(self.y),
            z: Self::sign_n(self.z),
        }
    }

    pub fn step(&self, vec3d: &Vec3d) -> Vec3d {
        return Vec3d {
            x: Self::bool_to_f54(vec3d.x > self.x),
            y: Self::bool_to_f54(vec3d.y > self.y),
            z: Self::bool_to_f54(vec3d.z > self.z),
        };
    }

    fn sign_n(n: f64) -> f64 {
        return Self::bool_to_f54(0.0 < n) - Self::bool_to_f54(n < 0.0);
    }

    fn bool_to_f54(value: bool) -> f64 {
        if value {
            1.0
        } else {
            0.0
        }
    }

    pub fn rotate_x(&mut self, angle: f64) {
        let old_y: f64 = self.y;
        let old_z: f64 = self.z;

        self.z = old_z * f64::cos(angle) - old_y * f64::sin(angle);
        self.y = old_z * f64::sin(angle) + old_y * f64::cos(angle);
    }

    pub fn rotate_y(&mut self, angle: f64) {
        let old_x: f64 = self.x;
        let old_z: f64 = self.z;

        self.x = old_x * f64::cos(angle) - old_z * f64::sin(angle);
        self.z = old_z * f64::sin(angle) + old_z * f64::cos(angle);
    }

    pub fn rotate_z(&mut self, angle: f64) {
        let old_x: f64 = self.x;
        let old_y: f64 = self.y;

        self.x = old_x * f64::cos(angle) - old_y * f64::sin(angle);
        self.y = old_x * f64::sin(angle) + old_y * f64::cos(angle);
    }

    fn len(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
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
