use num_bigint::BigUint;
use std::fmt::Display;

use crate::point::Point;
use crate::s256point::S256Point;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    secret: BigUint,
    point: Point,
}

impl PrivateKey {
    pub fn new(secret: &[u8]) -> PrivateKey {
        let secret = BigUint::parse_bytes(secret, 16).unwrap();
        PrivateKey {
            secret: secret.clone(),
            point: S256Point::generator().get_point().clone() * secret,
        }
    }

    pub fn get_secret(&self) -> &BigUint {
        &self.secret
    }

    pub fn get_point(&self) -> &Point {
        &self.point
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PrivateKey({:0>64x})", self.secret)
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.secret == other.secret
    }
}
