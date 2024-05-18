use crate::field_element::FieldElement;
use num_bigint::BigUint;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

#[derive(Debug, Clone)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.x {
            None => write!(f, "Point(infinity)"),
            Some(x) => write!(
                f,
                "Point({},{})_{}_{} in F_{}",
                x.get_number(),
                self.y.as_ref().unwrap().get_number(),
                self.a.get_number(),
                self.b.get_number(),
                self.a.get_prime()
            ),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

// Point addition
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not on the same curve");
        }

        match (self.x.clone(), other.x.clone()) {
            (None, _) => return other,
            (_, None) => return self,
            (Some(x1), Some(x2)) if x1 == x2 => {
                if self.y != other.y {
                    return Point::new(None, None, self.a, self.b); // Point at infinity
                }
                // Handling the doubling case
                if self == other {
                    let num = 3 * x1.clone().pow(2u32) + self.a.clone();
                    let denom = 2 * self.y.as_ref().unwrap().clone();
                    let s = num / denom;
                    let x3 = s.pow(2u32) - 2 * x1.clone();
                    let y3 = s * (x1.clone() - x3.clone()) - self.y.as_ref().unwrap().clone();
                    return Point::new(Some(x3), Some(y3), self.a, self.b);
                }
            }
            (Some(x1), Some(x2)) => {
                let s = (other.y.unwrap() - self.y.clone().unwrap()) / (x2.clone() - x1.clone());
                let x3 = s.pow(2u32) - x1.clone() - x2.clone();
                let y3 = s * (x1.clone() - x3.clone()) - self.y.unwrap();
                return Point::new(Some(x3), Some(y3), self.a, self.b);
            }
        }
        panic!("Unhandled point addition case");
    }
}

impl Mul<BigUint> for Point {
    type Output = Self;

    fn mul(self, other: BigUint) -> Self {
        let mut coef = other;
        let mut current = self.clone();
        let mut result = Point::new(None, None, self.a.clone(), self.b.clone());
        while coef > BigUint::ZERO {
            if &coef & BigUint::from(1u32) == BigUint::from(1u32) {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        result
    }
}

impl Mul<Point> for BigUint {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        other * self
    }
}

impl Point {
    pub fn get_x(&self) -> Option<&FieldElement> {
        self.x.as_ref()
    }

    pub fn get_y(&self) -> Option<&FieldElement> {
        self.y.as_ref()
    }

    pub fn get_a(&self) -> &FieldElement {
        &self.a
    }

    pub fn get_b(&self) -> &FieldElement {
        &self.b
    }

    // Constructs a new Point
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Self {
        match (&x, &y) {
            (Some(x_val), Some(y_val)) => {
                if y_val.pow(2u32) != x_val.pow(3u32) + a.clone() * x_val.clone() + b.clone() {
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
        let prime = 223;
        let a = FieldElement::from_int(0, prime);
        let b = FieldElement::from_int(7, prime);
        let valid_points = vec![
            (
                FieldElement::from_int(192, prime),
                FieldElement::from_int(105, prime),
            ),
            (
                FieldElement::from_int(17, prime),
                FieldElement::from_int(56, prime),
            ),
            (
                FieldElement::from_int(1, prime),
                FieldElement::from_int(193, prime),
            ),
        ];
        for (x, y) in valid_points {
            let p = Point::new(Some(x), Some(y), a.clone(), b.clone());
            assert_eq!(
                format!(
                    "Point({},{})_{}_{} in F_{}",
                    p.x.clone().unwrap().get_number(),
                    p.y.clone().unwrap().get_number(),
                    p.a.get_number(),
                    p.b.get_number(),
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
        Point::new(
            Some(FieldElement::from_int(200, prime)),
            Some(FieldElement::from_int(119, prime)),
            FieldElement::from_int(0, prime),
            FieldElement::from_int(7, prime),
        );
    }

    #[test]
    fn test_display() {
        let prime = 223;
        let a = Point::new(
            Some(FieldElement::from_int(192, prime)),
            Some(FieldElement::from_int(105, prime)),
            FieldElement::from_int(0, prime),
            FieldElement::from_int(7, prime),
        );
        assert_eq!(format!("{}", a), "Point(192,105)_0_7 in F_223");
        let b = Point::new(
            None,
            None,
            FieldElement::from_int(0, prime),
            FieldElement::from_int(7, prime),
        );
        assert_eq!(format!("{}", b), "Point(infinity)");
    }

    #[test]
    fn test_eq() {
        let prime = 223;
        let a = FieldElement::from_int(0, prime);
        let b = FieldElement::from_int(7, prime);
        let p1 = Point::new(
            Some(FieldElement::from_int(192, prime)),
            Some(FieldElement::from_int(105, prime)),
            a.clone(),
            b.clone(),
        );
        let p2 = Point::new(
            Some(FieldElement::from_int(192, prime)),
            Some(FieldElement::from_int(105, prime)),
            a,
            b,
        );
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_add() {
        let prime = 223;
        let a = FieldElement::from_int(0, prime);
        let b = FieldElement::from_int(7, prime);

        let additions = vec![
            (192, 105, 17, 56, 170, 142),
            (47, 71, 117, 141, 60, 139),
            (143, 98, 76, 66, 47, 71),
        ];

        for (x1_raw, y1_raw, x2_raw, y2_raw, x3_raw, y3_raw) in additions {
            let x1 = FieldElement::from_int(x1_raw, prime);
            let y1 = FieldElement::from_int(y1_raw, prime);
            let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone());

            let x2 = FieldElement::from_int(x2_raw, prime);
            let y2 = FieldElement::from_int(y2_raw, prime);
            let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone());

            let x3 = FieldElement::from_int(x3_raw, prime);
            let y3 = FieldElement::from_int(y3_raw, prime);
            let p3 = Point::new(Some(x3), Some(y3), a.clone(), b.clone());

            assert_eq!(p1.clone() + p2.clone(), p3);
        }
    }

    #[test]
    fn test_rmul() {
        let prime = 223;
        let a = FieldElement::from_int(0, prime);
        let b = FieldElement::from_int(7, prime);

        let multiplications = vec![
            (2, 192, 105, Some(49), Some(71)),
            (2, 143, 98, Some(64), Some(168)),
            (2, 47, 71, Some(36), Some(111)),
            (4, 47, 71, Some(194), Some(51)),
            (8, 47, 71, Some(116), Some(55)),
            (21, 47, 71, None, None),
        ];

        for (s, x1_raw, y1_raw, x2_raw, y2_raw) in multiplications {
            let x1 = FieldElement::from_int(x1_raw, prime);
            let y1 = FieldElement::from_int(y1_raw, prime);
            let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone());

            let p2 = match (x2_raw, y2_raw) {
                (Some(x2), Some(y2)) => {
                    let x2 = FieldElement::from_int(x2, prime);
                    let y2 = FieldElement::from_int(y2, prime);
                    Point::new(Some(x2), Some(y2), a.clone(), b.clone())
                }
                _ => Point::new(None, None, a.clone(), b.clone()),
            };

            let mut result = p1.clone();
            for _ in 1..s {
                result = result + p1.clone();
            }

            assert_eq!(result, p2);
        }
    }
}
