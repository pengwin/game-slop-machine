use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance_to(self, other: Self) -> f32 {
        (self - other).length()
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        }
    }

    pub fn from_min_size(min: Vec2, size: Vec2) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn size(self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    pub fn center(self) -> Vec2 {
        (self.min + self.max) / 2.0
    }

    pub fn area(self) -> f32 {
        self.width() * self.height()
    }

    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    pub fn intersects(self, other: Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment2D {
    pub start: Vec2,
    pub end: Vec2,
}

impl LineSegment2D {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn length(self) -> f32 {
        self.start.distance_to(self.end)
    }

    pub fn midpoint(self) -> Vec2 {
        (self.start + self.end) / 2.0
    }

    pub fn is_horizontal(self) -> bool {
        (self.start.y - self.end.y).abs() < f32::EPSILON
    }

    pub fn is_vertical(self) -> bool {
        (self.start.x - self.end.x).abs() < f32::EPSILON
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    pub fn other(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_operations() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
        assert_eq!(b - a, Vec2::new(2.0, 2.0));
        assert_eq!(a * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_rect_dimensions() {
        let r = Rect::new(0.0, 0.0, 10.0, 8.0);
        assert_eq!(r.width(), 10.0);
        assert_eq!(r.height(), 8.0);
        assert_eq!(r.area(), 80.0);
        assert_eq!(r.center(), Vec2::new(5.0, 4.0));
    }

    #[test]
    fn test_rect_contains() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains(Vec2::new(5.0, 5.0)));
        assert!(r.contains(Vec2::new(0.0, 0.0)));
        assert!(!r.contains(Vec2::new(11.0, 5.0)));
    }

    #[test]
    fn test_rect_intersects() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(5.0, 5.0, 15.0, 15.0);
        let c = Rect::new(20.0, 20.0, 30.0, 30.0);
        assert!(a.intersects(b));
        assert!(!a.intersects(c));
    }

    #[test]
    fn test_line_segment() {
        let seg = LineSegment2D::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
        assert!(seg.is_horizontal());
        assert!(!seg.is_vertical());
        assert_eq!(seg.length(), 10.0);
        assert_eq!(seg.midpoint(), Vec2::new(5.0, 0.0));
    }
}
