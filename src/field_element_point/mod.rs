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

impl FieldElementPoint {
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
        let invalid_points = vec![
            (FieldElement::new(200, prime), FieldElement::new(119, prime)),
            (FieldElement::new(42, prime), FieldElement::new(99, prime)),
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
}
