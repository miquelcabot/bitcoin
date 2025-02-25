use num_bigint::BigUint;
use rand::prelude::*;
use std::fmt::Display;

use super::S256Point;
use super::Signature;

/// PrivateKey represents a private key in the elliptic curve cryptography.
#[derive(Debug, Clone)]
pub struct PrivateKey {
    secret: BigUint,
    point: S256Point,
}

impl PrivateKey {
    /// Creates a new PrivateKey from a byte array.
    /// # Arguments
    /// * `secret` - A byte array representing the private key
    /// # Returns
    /// * `PrivateKey` - The PrivateKey created from the byte array
    pub fn new(secret: &[u8]) -> anyhow::Result<PrivateKey> {
        let secret = BigUint::parse_bytes(secret, 16).unwrap();
        let point = S256Point::generator().get_point().clone() * secret.clone();
        let s256point = S256Point::new(
            Some(
                &point
                    .get_x()
                    .unwrap()
                    .get_number()
                    .to_str_radix(16)
                    .into_bytes(),
            ),
            Some(
                &point
                    .get_y()
                    .unwrap()
                    .get_number()
                    .to_str_radix(16)
                    .into_bytes(),
            ),
        )?;
        Ok(PrivateKey {
            secret,
            point: s256point,
        })
    }

    /// Generates a random PrivateKey.
    /// # Returns
    /// * `PrivateKey` - A randomly generated private key
    pub fn random() -> anyhow::Result<PrivateKey> {
        let secret = generate_random_number(S256Point::BASE_ORDER);
        let point = S256Point::generator().get_point().clone() * secret.clone();
        let s256point = S256Point::new(
            Some(
                &point
                    .get_x()
                    .unwrap()
                    .get_number()
                    .to_str_radix(16)
                    .into_bytes(),
            ),
            Some(
                &point
                    .get_y()
                    .unwrap()
                    .get_number()
                    .to_str_radix(16)
                    .into_bytes(),
            ),
        )?;
        Ok(PrivateKey {
            secret,
            point: s256point,
        })
    }

    /// Returns the secret of the private key.
    /// # Returns
    /// * `&BigUint` - The secret of the private key
    pub fn get_secret(&self) -> &BigUint {
        &self.secret
    }

    /// Returns the point of the private key.
    /// # Returns
    /// * `&S256Point` - The point of the private key
    pub fn get_point(&self) -> &S256Point {
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_private_key_creation() -> Result<()> {
        let secret = b"1234567890abcdef1234567890abcdef";
        let private_key = PrivateKey::new(secret)?;
        assert_eq!(
            private_key.get_secret(),
            &BigUint::parse_bytes(secret, 16).unwrap()
        );
        Ok(())
    }

    #[test]
    fn test_private_key_random() -> Result<()> {
        let private_key1 = PrivateKey::random()?;
        let private_key2 = PrivateKey::random()?;

        assert_ne!(private_key1, private_key2);
        Ok(())
    }

    #[test]
    fn test_private_key_display() -> Result<()> {
        let secret = b"1234567890abcdef1234567890abcdef";
        let private_key = PrivateKey::new(secret)?;
        assert_eq!(
            format!("{}", private_key),
            format!(
                "PrivateKey({:0>64x})",
                BigUint::parse_bytes(secret, 16).unwrap()
            )
        );
        Ok(())
    }

    #[test]
    fn test_private_key_equality() -> Result<()> {
        let secret1 = b"1234567890abcdef1234567890abcdef";
        let secret2 = b"fedcba0987654321fedcba0987654321";
        let private_key1 = PrivateKey::new(secret1)?;
        let private_key2 = PrivateKey::new(secret1)?;
        let private_key3 = PrivateKey::new(secret2)?;
        assert_eq!(private_key1, private_key2);
        assert_ne!(private_key1, private_key3);
        Ok(())
    }

    #[test]
    fn test_generate_random_number() {
        let max = b"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let max_biguint = BigUint::parse_bytes(max, 16).unwrap();
        for _ in 0..100 {
            let random_number = generate_random_number(max);
            assert!(random_number < max_biguint);
        }
    }
}
