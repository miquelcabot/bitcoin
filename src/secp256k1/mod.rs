use crate::field_element::FieldElement;
use crate::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Secp256k1(Point);

impl Secp256k1 {
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

    pub fn new() -> Self {
        let gx = FieldElement::from_bytes(Self::G_X, Self::PRIME);
        let gy = FieldElement::from_bytes(Self::G_Y, Self::PRIME);
        let a = FieldElement::from_bytes(Self::A, Self::PRIME);
        let b = FieldElement::from_bytes(Self::B, Self::PRIME);

        Secp256k1(Point::new(Some(gx), Some(gy), a, b))
    }

    pub fn get_point(&self) -> &Point {
        &self.0
    }
}
