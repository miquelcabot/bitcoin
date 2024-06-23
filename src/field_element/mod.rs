use num_bigint::BigUint;
use std::{
    fmt::{Display, Error},
    ops::{Add, Div, Mul, Sub},
    process::Output,
};

#[derive(Debug, Clone)]
pub struct FieldElement {
    number: BigUint,
    prime: BigUint,
}

impl FieldElement {
    pub fn from_int(number: u32, prime: u32) -> Result<FieldElement, String> {
        if number >= prime {
            return Err(format!(
                "Num {} not in field range 0 to {}",
                number,
                prime - BigUint::from(1u32)
            ));
        }

        Ok(FieldElement {
            number: BigUint::from(number),
            prime: BigUint::from(prime),
        })
    }

    pub fn from_bytes(number: &[u8], prime: &[u8]) -> Result<FieldElement, String> {
        let number = BigUint::parse_bytes(number, 16).unwrap();
        let prime = BigUint::parse_bytes(prime, 16).unwrap();
        if number >= prime {
            return Err(format!(
                "Num {} not in field range 0 to {}",
                number,
                prime - BigUint::from(1u32)
            ));
        }

        Ok(FieldElement { number, prime })
    }

    pub fn get_number(&self) -> &BigUint {
        &self.number
    }

    pub fn get_prime(&self) -> &BigUint {
        &self.prime
    }

    pub fn pow(&self, exponent: impl Into<BigUint>) -> Self {
        let n = &exponent.into() % (&self.prime - BigUint::from(1u32));
        FieldElement {
            number: self.number.modpow(&n, &self.prime),
            prime: self.prime.clone(),
        }
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement({} in F_{})", self.number, self.prime)
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.prime == other.prime
    }
}

// Adds two FieldElement values
impl Add for FieldElement {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Cannot operate with two numbers in different Fields".to_string());
        }

        Ok(FieldElement {
            number: (&self.number + &other.number) % &self.prime,
            prime: self.prime.clone(),
        })
    }
}

// Subtracts two FieldElement values
impl Sub for FieldElement {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Cannot operate with two numbers in different Fields".to_string());
        }

        Ok(FieldElement {
            // Ensuring positive result
            number: (&self.number + &self.prime - &other.number) % &self.prime,
            prime: self.prime.clone(),
        })
    }
}

// Multiplies two FieldElement values
impl Mul<Self> for FieldElement {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Cannot operate with two numbers in different Fields".to_string());
        }

        Ok(FieldElement {
            number: (&self.number * &other.number) % &self.prime,
            prime: self.prime.clone(),
        })
    }
}

// Multiplies a FieldElement by a scalar
impl Mul<u32> for FieldElement {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        FieldElement {
            number: (&self.number * BigUint::from(other)) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

impl Mul<FieldElement> for u32 {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        other * self
    }
}

// Divides one FieldElement by another using Fermat's Little Theorem
impl Div for FieldElement {
    type Output = Result<Self, String>;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            return Err("Cannot operate with two numbers in different Fields".to_string());
        }
        // Calculate other's multiplicative inverse using Fermat's Little Theorem
        let inv = &other
            .number
            .modpow(&(&self.prime - BigUint::from(2u32)), &self.prime);
        let num = (&self.number * inv) % &self.prime;
        Ok(FieldElement {
            number: num,
            prime: self.prime.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_int() {
        let a = FieldElement::from_int(2, 31);
        assert_eq!(a.number, BigUint::from(2u32));
        assert_eq!(a.prime, BigUint::from(31u32));
    }

    #[test]
    #[should_panic]
    fn test_from_int_panic() {
        FieldElement::from_int(32, 31);
    }

    #[test]
    fn test_from_bytes() {
        let a = FieldElement::from_bytes(
            b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949",
            b"f70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055",
        );
        assert_eq!(
            a.number,
            BigUint::parse_bytes(
                b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949",
                16
            )
            .unwrap()
        );
        assert_eq!(
            a.prime,
            BigUint::parse_bytes(
                b"f70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055",
                16
            )
            .unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_from_bytes_panic() {
        FieldElement::from_bytes(
            b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949",
            b"070f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055",
        );
    }

    #[test]
    fn test_eq() {
        let a = FieldElement::from_int(2, 31);
        let b = FieldElement::from_int(2, 31);
        let c = FieldElement::from_int(15, 31);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::from_int(2, 31);
        let b = FieldElement::from_int(15, 31);
        assert_eq!(a + b, FieldElement::from_int(17, 31));
        let a = FieldElement::from_int(17, 31);
        let b = FieldElement::from_int(21, 31);
        assert_eq!(a + b, FieldElement::from_int(7, 31));

        let prime = b"f70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055";
        let a = FieldElement::from_bytes(
            b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949",
            prime,
        );
        let b = FieldElement::from_bytes(
            b"c23051f0a7a42d04bd25d1d4f65b4e51a365d8df764ea0ad02f8f576008dec00",
            prime,
        );
        println!("{:x}", a.get_number());
        println!("{:x}", b.get_number());
        println!("{}", a.get_prime());
        assert_eq!(
            a + b,
            FieldElement::from_bytes(
                b"25516dadcb5d522392d70b8523fba88a6a2eff7f7b7880c5bd5ae1402f3d54f4",
                prime
            )
        );
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::from_int(29, 31);
        let b = FieldElement::from_int(4, 31);
        assert_eq!(a - b, FieldElement::from_int(25, 31));
        let a = FieldElement::from_int(15, 31);
        let b = FieldElement::from_int(30, 31);
        assert_eq!(a - b, FieldElement::from_int(16, 31));
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::from_int(24, 31);
        let b = FieldElement::from_int(19, 31);
        assert_eq!(a * b, FieldElement::from_int(22, 31));
    }

    #[test]
    fn test_scalarmul() {
        let a = FieldElement::from_int(24, 31);
        assert_eq!(2 * a.clone(), a.clone() + a.clone());
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::from_int(17, 31);
        assert_eq!(a.pow(3u32), FieldElement::from_int(15, 31));
        let a = FieldElement::from_int(5, 31);
        let b = FieldElement::from_int(18, 31);
        assert_eq!(a.pow(5u32) * b, FieldElement::from_int(16, 31));
    }

    #[test]
    fn test_div() {
        let a = FieldElement::from_int(3, 31);
        let b = FieldElement::from_int(24, 31);
        assert_eq!(a / b, FieldElement::from_int(4, 31));
    }
}
