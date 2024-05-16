use num_bigint::BigUint;
use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone)]
pub struct FieldElement {
    number: BigUint,
    prime: BigUint,
}

impl FieldElement {
    // Constructs a new FieldElement, ensuring the value is within the field range
    pub fn from_int(number: u32, prime: u32) -> FieldElement {
        if number >= prime {
            panic!(
                "Num {} not in field range 0 to {}",
                number,
                prime - BigUint::from(1u32)
            );
        }

        FieldElement {
            number: BigUint::from(number),
            prime: BigUint::from(prime),
        }
    }

    pub fn from_bytes(number: &[u8], prime: &[u8]) -> FieldElement {
        let number = BigUint::from_bytes_be(number);
        let prime = BigUint::from_bytes_be(prime);
        if number >= prime {
            panic!(
                "Num {} not in field range 0 to {}",
                number,
                prime - BigUint::from(1u32)
            );
        }

        FieldElement { number, prime }
    }

    pub fn get_number(&self) -> &BigUint {
        &self.number
    }

    pub fn get_prime(&self) -> &BigUint {
        &self.prime
    }

    pub fn pow(&self, exponent: impl Into<BigUint>) -> Self {
        FieldElement {
            number: self.number.modpow(&exponent.into(), &self.prime),
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
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if &self.prime != &other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }

        FieldElement {
            number: (&self.number + &other.number) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

// Subtracts two FieldElement values
impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }

        FieldElement {
            // Ensuring positive result
            number: (&self.number + &self.prime - &other.number) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}

// Multiplies two FieldElement values
impl Mul<Self> for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }

        FieldElement {
            number: (&self.number * &other.number) % &self.prime,
            prime: self.prime.clone(),
        }
    }
}
/*
// Multiplies a FieldElement by a scalar
impl ops::Mul<i32> for FieldElement {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        let num = Self::modulo(self.num * U256::from(other), self.prime);
        FieldElement::new(num, self.prime)
    }
}

impl ops::Mul<FieldElement> for i32 {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        other * self
    }
}

// Divides one FieldElement by another using Fermat's Little Theorem
impl ops::Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }
        // Calculate other's multiplicative inverse using Fermat's Little Theorem
        let inv = Self::mod_pow(other.num, self.prime - 2, self.prime);
        let num = Self::modulo(self.num * inv, self.prime);
        FieldElement::new(num, self.prime)
    }
} */

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
            BigUint::from_bytes_be(
                b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949"
            )
        );
        assert_eq!(
            a.prime,
            BigUint::from_bytes_be(
                b"f70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055"
            )
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
    }
    /*
    #[test]
    fn test_sub() {
        let a = FieldElement::new(29, 31);
        let b = FieldElement::new(4, 31);
        assert_eq!(a - b, FieldElement::new(25, 31));
        let a = FieldElement::new(15, 31);
        let b = FieldElement::new(30, 31);
        assert_eq!(a - b, FieldElement::new(16, 31));
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(24, 31);
        let b = FieldElement::new(19, 31);
        assert_eq!(a * b, FieldElement::new(22, 31));
    }

    #[test]
    fn test_scalarmul() {
        let a = FieldElement::new(24, 31);
        assert_eq!(2 * a, a + a);
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(17, 31);
        assert_eq!(a.pow(U256::from(3)), FieldElement::new(15, 31));
        let a = FieldElement::new(5, 31);
        let b = FieldElement::new(18, 31);
        assert_eq!(a.pow(U256::from(5)) * b, FieldElement::new(16, 31));
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(3, 31);
        let b = FieldElement::new(24, 31);
        assert_eq!(a / b, FieldElement::new(4, 31));
    } */
}
