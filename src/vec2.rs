use std::iter::Sum;
use std::ops;

/// A 2-dimensional vector.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn zeros() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn norm_sqr(self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

impl ops::Add<f64> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self { x: 0.0, y: 0.0 }, |a, b| Self {
            x: a.x + b.x,
            y: a.y + b.y,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let result: Vec2 = vec![(1.0_f64, 1.0), (5.5, 0.0), (0.0, -1.1), (1.1, 0.11)]
            .iter()
            .map(|(x, y)| Vec2 { x: *x, y: *y })
            .sum();

        assert_eq!(
            result,
            Vec2 {
                x: 7.6,
                y: 0.009999999999999912,
            }
        );
    }
}
