use std::ops;

pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Vec2d {
        return Vec2d { x, y };
    }

    fn len(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn normalize(&mut self) {
        let length: f64 = self.len();

        self.x /= length;
        self.y /= length;
    }
}

impl ops::Neg for Vec2d {
    type Output = Vec2d;

    fn neg(self) -> Vec2d {
        return Vec2d {
            x: -self.x,
            y: -self.y,
        };
    }
}


impl ops::Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, _rhs: Vec2d) -> Vec2d {
        return Vec2d {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}

impl ops::Add<f64> for Vec2d {
    type Output = Vec2d;

    fn add(self, _rhs: f64) -> Vec2d {
        return Vec2d {
            x: self.x + _rhs,
            y: self.y + _rhs,
        };
    }
}


impl ops::Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, _rhs: Vec2d) -> Vec2d {
        return Vec2d {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        };
    }
}

impl ops::Sub<f64> for Vec2d {
    type Output = Vec2d;

    fn sub(self, _rhs: f64) -> Vec2d {
        return Vec2d {
            x: self.x - _rhs,
            y: self.y - _rhs,
        };
    }
}


impl ops::Mul<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn mul(self, _rhs: Vec2d) -> Vec2d {
        return Vec2d {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        };
    }
}

impl ops::Mul<f64> for Vec2d {
    type Output = Vec2d;

    fn mul(self, _rhs: f64) -> Vec2d {
        return Vec2d {
            x: self.x * _rhs,
            y: self.y * _rhs,
        };
    }
}


impl ops::Div<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn div(self, _rhs: Vec2d) -> Vec2d {
        return Vec2d {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        };
    }
}

impl ops::Div<f64> for Vec2d {
    type Output = Vec2d;

    fn div(self, _rhs: f64) -> Vec2d {
        return Vec2d {
            x: self.x / _rhs,
            y: self.y / _rhs,
        };
    }
}
