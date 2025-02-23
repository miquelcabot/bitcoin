use anyhow::{bail, Result};
use num_bigint::BigUint;
use std::fmt::{Display, Formatter};
use std::ops::Mul;

use super::FieldElement;
use super::Point;
use super::Signature;

/// Elliptic curve point on secp256k1
#[derive(Debug, Clone, PartialEq)]
pub struct S256Point(Point);

impl S256Point {
    pub const A: &'static [u8; 1] = b"0";
    pub const B: &'static [u8; 1] = b"7";
    pub const BASE_ORDER: &'static [u8; 64] =
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";
    // prime = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1 = 2^256 - 2^32 - 977
    pub const PRIME: &'static [u8; 64] =
        b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
    pub const G_X: &'static [u8; 64] =
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    pub const G_Y: &'static [u8; 64] =
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

    /// Creates a new S256Point from x and y coordinates
    /// # Arguments
    /// * `x` - The x coordinate of the point
    /// * `y` - The y coordinate of the point
    /// # Returns
    /// * `S256Point` - The S256Point created from the coordinates
    pub fn new(x: Option<&[u8]>, y: Option<&[u8]>) -> Result<Self> {
        let a = FieldElement::from_bytes(Self::A, Self::PRIME).unwrap();
        let b = FieldElement::from_bytes(Self::B, Self::PRIME).unwrap();

        match (x, y) {
            (Some(x), Some(y)) => {
                let gx = FieldElement::from_bytes(x, Self::PRIME).unwrap();
                let gy = FieldElement::from_bytes(y, Self::PRIME).unwrap();

                Ok(S256Point(Point::new(Some(gx), Some(gy), a, b)?))
            }
            (None, None) => Ok(S256Point(Point::new(None, None, a, b)?)),
            _ => bail!("Incomplete point coordinates"),
        }
    }

    /// Returns the generator point of the curve
    /// # Returns
    /// * `S256Point` - The generator point of the curve
    pub fn generator() -> Self {
        S256Point::new(Some(Self::G_X), Some(Self::G_Y)).unwrap()
    }

    /// Returns the point
    pub fn get_point(&self) -> &Point {
        &self.0
    }

    /// Verifies a signature
    /// # Arguments
    /// * `z` - The hash of the message
    /// * `signature` - The signature to verify
    /// # Returns
    /// * `bool` - True if the signature is valid, false otherwise
    pub fn verify(&self, z: BigUint, signature: Signature) -> bool {
        let base_order = BigUint::parse_bytes(Self::BASE_ORDER, 16).unwrap();
        let s_inv = signature.get_s().modpow(&(&base_order - 2u32), &base_order);
        let u = (&z * &s_inv) % &base_order;
        let v = (signature.get_r() * &s_inv) % &base_order;
        let total =
            (u * S256Point::generator().get_point().clone()) + (v * self.get_point().clone());
        total.get_x().unwrap().get_number() == signature.get_r()
    }

    /// Returns the SEC format of the point
    /// # Arguments
    /// * `compressed` - Whether to use compressed format
    /// # Returns
    /// * `Vec<u8>` - The SEC format encoding of the point
    pub fn to_sec(&self, compressed: bool) -> Result<Vec<u8>> {
        match (self.0.get_x(), self.0.get_y()) {
            (Some(x), Some(y)) => {
                let x_bytes = x.get_number().to_bytes_be();
                let y_bytes = y.get_number().to_bytes_be();

                if compressed {
                    let prefix = if y.get_number() % 2u32 == BigUint::from(0u32) {
                        0x02
                    } else {
                        0x03
                    };
                    let mut result = vec![prefix];
                    result.extend(vec![0; 32 - x_bytes.len()]); // Ensure 32 bytes
                    result.extend(x_bytes);
                    Ok(result)
                } else {
                    let mut result = vec![0x04];
                    result.extend(vec![0; 32 - x_bytes.len()]); // Ensure 32 bytes
                    result.extend(x_bytes);
                    result.extend(vec![0; 32 - y_bytes.len()]); // Ensure 32 bytes
                    result.extend(y_bytes);
                    Ok(result)
                }
            }
            _ => bail!("Point at infinity cannot be serialized"),
        }
    }

    /// Returns the SEC format of the point in string format
    /// # Arguments
    /// * `compressed` - Whether to use compressed format
    /// # Returns
    /// * `String` - The SEC format encoding of the point in string format
    pub fn to_sec_str(&self, compressed: bool) -> Result<String> {
        let sec_bytes = self.to_sec(compressed)?;
        let mut hex = String::with_capacity(2 + sec_bytes.len() * 2);
        hex.push_str("0x");
        for byte in sec_bytes {
            use std::fmt::Write;
            write!(&mut hex, "{:02x}", byte).unwrap();
        }
        Ok(hex)
    }
}

// Formats the S256Point
impl Display for S256Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.0.get_x() {
            None => write!(f, "S256Point(infinity)"),
            Some(x) => write!(
                f,
                "S256Point({},{})",
                x.get_number(),
                self.0.get_y().as_ref().unwrap().get_number()
            ),
        }
    }
}

// S256Point multiplication
impl Mul<BigUint> for S256Point {
    type Output = S256Point;

    fn mul(self, coefficient: BigUint) -> S256Point {
        let n = BigUint::parse_bytes(S256Point::BASE_ORDER, 16).unwrap();
        let coef = coefficient % n;

        let res = self.0 * coef;

        match res.get_x() {
            None => S256Point::new(None, None).unwrap(),
            Some(_) => S256Point::new(
                Some(&res.get_x().unwrap().get_number().to_bytes_be()),
                Some(&res.get_y().unwrap().get_number().to_bytes_be()),
            )
            .unwrap(),
        }
    }
}

// S256Point multiplication
impl Mul<S256Point> for BigUint {
    type Output = S256Point;

    fn mul(self, point: S256Point) -> S256Point {
        point * self
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_verify() -> Result<()> {
        let signature = Signature::from_bytes(
            b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
        )?;

        let point = S256Point::new(
            Some(b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574"),
            Some(b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4"),
        )?;

        let z = BigUint::parse_bytes(
            b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
            16,
        )
        .unwrap();

        assert!(point.verify(z, signature));
        Ok(())
    }

    #[test]
    fn test_verify2() -> Result<()> {
        let signatures: Vec<(&[u8; 64], &[u8; 64], &[u8; 64])> = vec![
            (
                b"ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
                b"ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
                b"068342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
            ),
            (
                b"7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
                b"00eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
                b"c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            ),
        ];

        let point = S256Point::new(
            Some(b"887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c"),
            Some(b"61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34"),
        )?;

        for (z, r, s) in signatures {
            let signature = Signature::from_bytes(r, s)?;
            let z = BigUint::parse_bytes(z, 16).unwrap();
            assert!(point.verify(z, signature));
        }
        Ok(())
    }

    #[test]
    fn test_mul_base_order_to_generator() -> Result<()> {
        let n = BigUint::parse_bytes(S256Point::BASE_ORDER, 16).unwrap();
        let point = S256Point::generator();
        assert_eq!(point * n, S256Point::new(None, None)?);
        Ok(())
    }

    #[test]
    fn test_to_sec_compressed() -> Result<()> {
        let point = S256Point::generator();
        let sec_bytes = point.to_sec(true)?;
        assert_eq!(sec_bytes.len(), 33);
        assert!(sec_bytes[0] == 0x02 || sec_bytes[0] == 0x03);
        Ok(())
    }

    #[test]
    fn test_to_sec_uncompressed() -> Result<()> {
        let point = S256Point::generator();
        let sec_bytes = point.to_sec(false)?;
        assert_eq!(sec_bytes.len(), 65);
        assert_eq!(sec_bytes[0], 0x04);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Point at infinity cannot be serialized")]
    fn test_to_sec_infinity() {
        let point = S256Point::new(None, None).unwrap();
        point.to_sec(true).unwrap();
    }

    #[test]
    fn test_to_sec_str_compressed() -> Result<()> {
        let point = S256Point::generator();
        let sec_str = point.to_sec_str(true)?;
        assert!(sec_str.starts_with("0x02") || sec_str.starts_with("0x03"));
        assert_eq!(sec_str.len(), 68);
        Ok(())
    }

    #[test]
    fn test_to_sec_str_uncompressed() -> Result<()> {
        let point = S256Point::generator();
        let sec_str = point.to_sec_str(false)?;
        assert!(sec_str.starts_with("0x04"));
        assert_eq!(sec_str.len(), 132);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Point at infinity cannot be serialized")]
    fn test_to_sec_str_infinity() {
        let point = S256Point::new(None, None).unwrap();
        point.to_sec_str(true).unwrap();
    }
}
