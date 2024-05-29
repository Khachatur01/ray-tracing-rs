use std::ops;

pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        return Vec2D { x, y };
    }
}

impl ops::Neg for Vec2D {
    type Output = Vec2D;

    fn neg(self) -> Vec2D {
        return Vec2D {
            x: -self.x,
            y: -self.y,
        };
    }
}


impl ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, _rhs: Vec2D) -> Vec2D {
        return Vec2D {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}

impl ops::Add<f64> for Vec2D {
    type Output = Vec2D;

    fn add(self, _rhs: f64) -> Vec2D {
        return Vec2D {
            x: self.x + _rhs,
            y: self.y + _rhs,
        };
    }
}


impl ops::Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, _rhs: Vec2D) -> Vec2D {
        return Vec2D {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        };
    }
}

impl ops::Sub<f64> for Vec2D {
    type Output = Vec2D;

    fn sub(self, _rhs: f64) -> Vec2D {
        return Vec2D {
            x: self.x - _rhs,
            y: self.y - _rhs,
        };
    }
}


impl ops::Mul<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn mul(self, _rhs: Vec2D) -> Vec2D {
        return Vec2D {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
        };
    }
}

impl ops::Mul<f64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, _rhs: f64) -> Vec2D {
        return Vec2D {
            x: self.x * _rhs,
            y: self.y * _rhs,
        };
    }
}


impl ops::Div<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn div(self, _rhs: Vec2D) -> Vec2D {
        return Vec2D {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
        };
    }
}

impl ops::Div<f64> for Vec2D {
    type Output = Vec2D;

    fn div(self, _rhs: f64) -> Vec2D {
        return Vec2D {
            x: self.x / _rhs,
            y: self.y / _rhs,
        };
    }
}
