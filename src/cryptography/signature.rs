use anyhow::Result;
use num_bigint::BigUint;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn new(r: BigUint, s: BigUint) -> Signature {
        Signature { r, s }
    }

    pub fn from_bytes(r: &[u8], s: &[u8]) -> Result<Signature> {
        let r = BigUint::parse_bytes(r, 16)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse r from bytes"))?;
        let s = BigUint::parse_bytes(s, 16)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse s from bytes"))?;
        Ok(Self::new(r, s))
    }

    pub fn get_r(&self) -> &BigUint {
        &self.r
    }

    pub fn get_s(&self) -> &BigUint {
        &self.s
    }
}

impl Signature {
    pub fn to_der(&self) -> Vec<u8> {
        fn encode_integer(value: &BigUint) -> Vec<u8> {
            let mut bin = value.to_bytes_be(); // Convert BigUint to bytes (Big Endian)
            while bin.first() == Some(&0) {
                // Remove `0x00` bytes at the beginning
                bin.remove(0);
            }
            if bin.first().is_some_and(|&b| b & 0x80 != 0) {
                // If the first byte has the most significant bit set, add a `0x00`
                bin.insert(0, 0x00);
            }
            let mut encoded = vec![0x02, bin.len() as u8]; // Prefix for INTEGER
            encoded.extend(bin);
            encoded
        }

        let mut result = encode_integer(&self.r);
        result.extend(encode_integer(&self.s));

        let mut der_encoded = vec![0x30, result.len() as u8]; // SEQUENCE prefix
        der_encoded.extend(result);

        der_encoded
    }

    pub fn to_der_str(&self) -> String {
        let der = self.to_der();
        let mut der_str = String::new();
        for byte in der {
            der_str.push_str(&format!("{:02x}", byte));
        }
        der_str
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Signature({},{})", self.r, self.s)
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.s == other.s
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use crate::PrivateKey;

    use super::*;

    #[test]
    fn test_private_key_creation() -> Result<()> {
        let secret = b"1234567890abcdef1234567890abcdef";
        let private_key = PrivateKey::from_bytes(secret)?;
        assert_eq!(
            private_key.get_secret(),
            &BigUint::parse_bytes(secret, 16).unwrap()
        );
        Ok(())
    }

    #[test]
    fn test_private_key_display() -> Result<()> {
        let secret = b"1234567890abcdef1234567890abcdef";
        let private_key = PrivateKey::from_bytes(secret)?;
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
        let private_key1 = PrivateKey::from_bytes(secret1)?;
        let private_key2 = PrivateKey::from_bytes(secret1)?;
        let private_key3 = PrivateKey::from_bytes(secret2)?;
        assert_eq!(private_key1, private_key2);
        assert_ne!(private_key1, private_key3);
        Ok(())
    }

    #[test]
    fn test_signature_der() -> Result<()> {
        let r = BigUint::parse_bytes(
            b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();
        let s = BigUint::parse_bytes(
            b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();

        let signature = Signature::new(r, s);
        let der_str = signature.to_der_str();
        assert_eq!(
            der_str,
            "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c\
             60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"
        );

        Ok(())
    }
}
