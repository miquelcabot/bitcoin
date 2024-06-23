use num_bigint::BigUint;

use crate::field_element::FieldElement;
use crate::point::Point;
use crate::signature::Signature;

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
        let a = FieldElement::from_bytes(Self::A, Self::PRIME);
        let b = FieldElement::from_bytes(Self::B, Self::PRIME);

        match (x, y) {
            (Some(x), Some(y)) => {
                let gx = FieldElement::from_bytes(x, Self::PRIME);
                let gy = FieldElement::from_bytes(y, Self::PRIME);

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

    pub fn verify(&self, z: BigUint, signature: Signature) -> bool {
        let base_order = BigUint::parse_bytes(Self::BASE_ORDER, 16).unwrap();
        let s_inv = signature.get_s().modpow(&(&base_order - 2u32), &base_order);
        let u = (&z * &s_inv) % &base_order;
        let v = (signature.get_r() * &s_inv) % &base_order;
        let total =
            (u * S256Point::generator().get_point().clone()) + (v * self.get_point().clone());
        total.get_x().unwrap().get_number() == signature.get_r()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify() {
        let signature = Signature::from_bytes(
            b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
        );

        let point = S256Point::new(
            Some(b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574"),
            Some(b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4"),
        );

        let z = BigUint::parse_bytes(
            b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
            16,
        )
        .unwrap();

        assert!(point.verify(z, signature));
    }
}
