use crate::field_element::FieldElement;
use crate::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct S256Point(Point);

impl S256Point {
    pub const A: &'static [u8; 1] = b"0";
    pub const B: &'static [u8; 1] = b"7";
    pub const BASE_ORDER: &'static [u8; 64] =
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
    // prime = 2^256 - 2^32 - 977
    pub const PRIME: &'static [u8; 64] =
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
    pub const G_X: &'static [u8; 64] =
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    pub const G_Y: &'static [u8; 64] =
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

    pub fn new(x: Option<&[u8]>, y: Option<&[u8]>) -> Self {
        let a = FieldElement::from_bytes(Self::A, Self::PRIME).unwrap();
        let b = FieldElement::from_bytes(Self::B, Self::PRIME).unwrap();

        match (x, y) {
            (Some(x), Some(y)) => {
                let gx = FieldElement::from_bytes(x, Self::PRIME).unwrap();
                let gy = FieldElement::from_bytes(y, Self::PRIME).unwrap();

                S256Point(Point::new(Some(gx), Some(gy), a, b))
            }
            (None, None) => S256Point(Point::new(None, None, a, b)),
            _ => panic!("Incomplete point coordinates"),
        }
    }

    pub fn generator() -> Self {
        S256Point::new(Some(Self::G_X), Some(Self::G_Y))
    }

    pub fn get_point(&self) -> &Point {
        &self.0
    }
}
