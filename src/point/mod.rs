use std::fmt;

#[derive(Debug, Clone)]
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
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Result<Self, String> {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                if y_val.pow(2) != x_val.pow(3) + a * x_val + b {
                    return Err(format!("({}, {}) is not on the curve", x_val, y_val));
                }
            }
            (None, None) => return Ok(Point { x, y, a, b }),
            _ => return Err("Incomplete point coordinates".to_string()),
        }
        Ok(Point { x, y, a, b })
    }

    // Point addition
    pub fn add(&self, other: &Point) -> Result<Self, String> {
        if self.a != other.a || self.b != other.b {
            return Err("Points are not on the same curve".to_string());
        }

        match (self.x, other.x) {
            (None, _) => return Ok(other.clone()),
            (_, None) => return Ok(self.clone()),
            (Some(x1), Some(x2)) if x1 == x2 => {
                if self.y != other.y {
                    return Ok(Point::new(None, None, self.a, self.b)?); // Point at infinity
                }
                // Handling the doubling case
                if self == other {
                    let num = 3 * x1.pow(2) + self.a;
                    let denom = 2 * self.y.unwrap();
                    let s = num / denom;
                    let x3 = s.pow(2) - 2 * x1;
                    let y3 = s * (x1 - x3) - self.y.unwrap();
                    return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
                }
            }
            (Some(x1), Some(x2)) => {
                let s = (other.y.unwrap() - self.y.unwrap()) / (x2 - x1);
                let x3 = s.pow(2) - x1 - x2;
                let y3 = s * (x1 - x3) - self.y.unwrap();
                return Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?);
            }
        }
        Err("Unhandled point addition case".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_equality() {
        let a = Point::new(Some(3), Some(-7), 5, 7).unwrap();
        let b = Point::new(Some(18), Some(77), 5, 7).unwrap();
        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_point_addition() {
        let inf = Point::new(None, None, 5, 7).unwrap();
        let p1 = Point::new(Some(2), Some(5), 5, 7).unwrap();
        let p2 = Point::new(Some(2), Some(-5), 5, 7).unwrap();
        assert_eq!(p1.add(&inf).unwrap(), p1);
        assert_eq!(inf.add(&p1).unwrap(), p1);
        assert_eq!(p1.add(&p2).unwrap(), inf);
    }
}
