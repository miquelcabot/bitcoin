use super::field_element::FieldElement;
use std::{fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct FieldElementPoint {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl fmt::Display for FieldElementPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.x {
            None => write!(f, "FieldElementPoint(infinity)"),
            Some(x) => write!(
                f,
                "FieldElementPoint({},{})_{}_{} in F_{}",
                x.get_num(),
                self.y.unwrap().get_num(),
                self.a.get_num(),
                self.b.get_num(),
                self.a.get_prime()
            ),
        }
    }
}

impl PartialEq for FieldElementPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

// Point addition
impl ops::Add for FieldElementPoint {
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
                    return FieldElementPoint::new(None, None, self.a, self.b); // Point at infinity
                }
                // Handling the doubling case
                if self == other {
                    let num = FieldElement::new(3, x1.get_prime()) * x1.pow(2) + self.a;
                    let denom = FieldElement::new(2, x1.get_prime()) * self.y.unwrap();
                    let s = num / denom;
                    let x3 = s.pow(2) - FieldElement::new(2, x1.get_prime()) * x1;
                    let y3 = s * (x1 - x3) - self.y.unwrap();
                    return FieldElementPoint::new(Some(x3), Some(y3), self.a, self.b);
                }
            }
            (Some(x1), Some(x2)) => {
                let s = (other.y.unwrap() - self.y.unwrap()) / (x2 - x1);
                let x3 = s.pow(2) - x1 - x2;
                let y3 = s * (x1 - x3) - self.y.unwrap();
                return FieldElementPoint::new(Some(x3), Some(y3), self.a, self.b);
            }
        }
        panic!("Unhandled point addition case");
    }
}

impl FieldElementPoint {
    pub fn get_x(&self) -> Option<FieldElement> {
        self.x
    }

    pub fn get_y(&self) -> Option<FieldElement> {
        self.y
    }

    pub fn get_a(&self) -> FieldElement {
        self.a
    }

    pub fn get_b(&self) -> FieldElement {
        self.b
    }

    // Constructs a new FieldElementPoint
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Self {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                if y_val.pow(2) != x_val.pow(3) + a * x_val + b {
                    panic!("({}, {}) is not on the curve", x_val, y_val);
                }
            }
            (None, None) => return FieldElementPoint { x, y, a, b },
            _ => panic!("Incomplete point coordinates"),
        }
        FieldElementPoint { x, y, a, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let valid_points = vec![
            (FieldElement::new(192, prime), FieldElement::new(105, prime)),
            (FieldElement::new(17, prime), FieldElement::new(56, prime)),
            (FieldElement::new(1, prime), FieldElement::new(193, prime)),
        ];
        for (x, y) in valid_points {
            let p = FieldElementPoint::new(Some(x), Some(y), a, b);
            assert_eq!(
                format!(
                    "FieldElementPoint({},{})_{}_{} in F_{}",
                    p.x.unwrap().get_num(),
                    p.y.unwrap().get_num(),
                    p.a.get_num(),
                    p.b.get_num(),
                    p.a.get_prime()
                ),
                p.to_string()
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        let prime = 223;
        FieldElementPoint::new(
            Some(FieldElement::new(200, prime)),
            Some(FieldElement::new(119, prime)),
            FieldElement::new(0, prime),
            FieldElement::new(7, prime),
        );
    }

    #[test]
    fn test_display() {
        let prime = 223;
        let a = FieldElementPoint::new(
            Some(FieldElement::new(192, prime)),
            Some(FieldElement::new(105, prime)),
            FieldElement::new(0, prime),
            FieldElement::new(7, prime),
        );
        assert_eq!(format!("{}", a), "FieldElementPoint(192,105)_0_7 in F_223");
        let b = FieldElementPoint::new(
            None,
            None,
            FieldElement::new(0, prime),
            FieldElement::new(7, prime),
        );
        assert_eq!(format!("{}", b), "FieldElementPoint(infinity)");
    }

    #[test]
    fn test_eq() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let p1 = FieldElementPoint::new(
            Some(FieldElement::new(192, prime)),
            Some(FieldElement::new(105, prime)),
            a,
            b,
        );
        let p2 = FieldElementPoint::new(
            Some(FieldElement::new(192, prime)),
            Some(FieldElement::new(105, prime)),
            a,
            b,
        );
        assert_eq!(p1, p2);
    }
}
