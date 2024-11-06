use num_bigint::BigUint;
use rand::prelude::*;
use std::fmt::Display;

use crate::point::Point;
use crate::s256point::S256Point;
use crate::signature::Signature;

/// PrivateKey represents a private key in the elliptic curve cryptography.
#[derive(Debug, Clone)]
pub struct PrivateKey {
    secret: BigUint,
    point: Point,
}

impl PrivateKey {
    /// Creates a new PrivateKey from a byte array.
    /// # Arguments
    /// * `secret` - A byte array representing the private key
    /// # Returns
    /// * `PrivateKey` - The PrivateKey created from the byte array
    pub fn new(secret: &[u8]) -> PrivateKey {
        let secret = BigUint::parse_bytes(secret, 16).unwrap();
        PrivateKey {
            secret: secret.clone(),
            point: S256Point::generator().get_point().clone() * secret,
        }
    }

    /// Returns the secret of the private key.
    /// # Returns
    /// * `&BigUint` - The secret of the private key
    pub fn get_secret(&self) -> &BigUint {
        &self.secret
    }

    /// Returns the point of the private key.
    /// # Returns
    /// * `&Point` - The point of the private key
    pub fn get_point(&self) -> &Point {
        &self.point
    }

    /// Signs a message with the private key.
    /// # Arguments
    /// * `z` - A byte array representing the message to sign
    /// # Returns
    /// * `Signature` - The signature of the message
    pub fn sign(&self, z: &[u8]) -> Signature {
        let _rng = rand::thread_rng();
        let k = generate_random_number(S256Point::BASE_ORDER);
        let r = (S256Point::generator().get_point().clone() * k.clone())
            .get_x()
            .unwrap()
            .get_number()
            .clone();
        let base_order = BigUint::parse_bytes(S256Point::BASE_ORDER, 16).unwrap();
        let k_inv = k.modpow(&(&base_order - 2u32), &base_order);
        let mut s = ((BigUint::parse_bytes(z, 16).unwrap() + (r.clone() * self.secret.clone()))
            * k_inv)
            % base_order.clone();
        if s > base_order.clone() / 2u32 {
            s = base_order - s;
        }

        Signature::new(r, s)
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

fn generate_random_number(max: &[u8]) -> BigUint {
    let mut rng = rand::thread_rng();

    let mut secret: [u8; 32] = [0u8; 32];
    rng.fill_bytes(&mut secret);

    let mut s = BigUint::from_bytes_be(&secret);

    let max = BigUint::parse_bytes(max, 16).unwrap();
    while s >= max {
        rng.fill_bytes(&mut secret);
        s = BigUint::from_bytes_be(&secret);
    }

    s
}
