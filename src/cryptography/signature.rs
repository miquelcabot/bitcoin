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
        Ok(Signature { r, s })
    }

    pub fn get_r(&self) -> &BigUint {
        &self.r
    }

    pub fn get_s(&self) -> &BigUint {
        &self.s
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
        let private_key = PrivateKey::new(secret)?;
        assert_eq!(
            private_key.get_secret(),
            &BigUint::parse_bytes(secret, 16).unwrap()
        );
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
}
