
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2D { x, y }
    }

    pub fn unit() -> Self {
        Vec2D { x: 1.0, y: 1.0 }
    }

    pub fn is_equal(&self, other: &Vec2D) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn dot(&self, other: &Vec2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, other: Vec2D) -> Self {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Vec2D) -> Self {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2D;

    #[test]
    fn test_add() {
        let a = Vec2D::new(1.0, 2.0);
        let b = Vec2D::new(3.0, 4.0);
        let c = a + b;
        assert!(c.is_equal(&Vec2D::new(4.0, 6.0)));
    }

    #[test]
    fn test_sub() {
        let a = Vec2D::new(5.0, 6.0);
        let b = Vec2D::new(3.0, 2.0);
        let c = a - b;
        assert!(c.is_equal(&Vec2D::new(2.0, 4.0)));
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vec2D::new(2.0, -3.0);
        let c = a * 2.0;
        assert!(c.is_equal(&Vec2D::new(4.0, -6.0)));
    }

    #[test]
    fn test_dot() {
        let a = Vec2D::new(1.0, 2.0);
        let b = Vec2D::new(3.0, 4.0);
        assert!((a.dot(&b) - 11.0).abs() < 1e-9);
    }

    #[test]
    fn test_display_and_equal() {
        let a = Vec2D::new(1.0, 1.0);
        let s = format!("{}", a);
        assert_eq!(s, "(1, 1)");
        assert!(a.is_equal(&Vec2D::new(1.0, 1.0)));
    }
}