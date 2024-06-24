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

    pub fn from_bytes(r: &[u8], s: &[u8]) -> Signature {
        let r = BigUint::parse_bytes(r, 16).unwrap();
        let s = BigUint::parse_bytes(s, 16).unwrap();
        Signature { r, s }
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
