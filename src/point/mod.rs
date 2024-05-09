use std::{fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: Option<i64>,
    y: Option<i64>,
    a: i64,
    b: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.x {
            None => write!(f, "Point(infinity)"),
            Some(x) => write!(f, "Point({},{})_{}_{}", x, self.y.unwrap(), self.a, self.b),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

// Point addition
impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not on the same curve");
        }

        match (self.x, other.x) {
            (None, _) => return other.clone(),
            (_, None) => return self.clone(),
            (Some(x1), Some(x2)) if x1 == x2 => {
                if self.y != other.y {
                    return Point::new(None, None, self.a, self.b); // Point at infinity
                }
                // Handling the doubling case
                if self == other {
                    let num = 3 * x1.pow(2) + self.a;
                    let denom = 2 * self.y.unwrap();
                    let s = num / denom;
                    let x3 = s.pow(2) - 2 * x1;
                    let y3 = s * (x1 - x3) - self.y.unwrap();
                    return Point::new(Some(x3), Some(y3), self.a, self.b);
                }
            }
            (Some(x1), Some(x2)) => {
                let s = (other.y.unwrap() - self.y.unwrap()) / (x2 - x1);
                let x3 = s.pow(2) - x1 - x2;
                let y3 = s * (x1 - x3) - self.y.unwrap();
                return Point::new(Some(x3), Some(y3), self.a, self.b);
            }
        }
        panic!("Unhandled point addition case");
    }
}

impl Point {
    pub fn get_x(&self) -> Option<i64> {
        self.x
    }

    pub fn get_y(&self) -> Option<i64> {
        self.y
    }

    pub fn get_a(&self) -> i64 {
        self.a
    }

    pub fn get_b(&self) -> i64 {
        self.b
    }

    // Constructs a new Point
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Self {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                if y_val.pow(2) != x_val.pow(3) + a * x_val + b {
                    panic!("({}, {}) is not on the curve", x_val, y_val);
                }
            }
            (None, None) => return Point { x, y, a, b },
            _ => panic!("Incomplete point coordinates"),
        }
        Point { x, y, a, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = Point::new(Some(2), Some(5), 5, 7);
        assert_eq!(a.x, Some(2));
        assert_eq!(a.y, Some(5));
        assert_eq!(a.a, 5);
        assert_eq!(a.b, 7);
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        Point::new(Some(2), Some(4), 5, 7);
    }

    #[test]
    fn test_display() {
        let a = Point::new(Some(2), Some(5), 5, 7);
        assert_eq!(format!("{}", a), "Point(2,5)_5_7");
        let b = Point::new(None, None, 5, 7);
        assert_eq!(format!("{}", b), "Point(infinity)");
    }

    #[test]
    fn test_eq() {
        let a = Point::new(Some(3), Some(-7), 5, 7);
        let b = Point::new(Some(18), Some(77), 5, 7);
        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_add() {
        let inf = Point::new(None, None, 5, 7);
        let p1 = Point::new(Some(2), Some(5), 5, 7);
        let p2 = Point::new(Some(2), Some(-5), 5, 7);
        assert_eq!(p1 + inf, p1);
        assert_eq!(inf + p1, p1);
        assert_eq!(p1 + p2, inf);
    }
}
