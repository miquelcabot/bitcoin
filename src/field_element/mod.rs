use num_bigint::BigUint;
use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone)]
pub struct FieldElement {
    number: BigUint,
    prime: BigUint,
}

impl FieldElement {
    // Constructs a new FieldElement, ensuring the value is within the field range
    pub fn new(number: impl Into<BigUint>, prime: impl Into<BigUint>) -> FieldElement {
        let number = number.into();
        let prime = prime.into();

        if &number >= &prime {
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
/*
// Subtracts two FieldElement values
impl ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }
        let num = Self::modulo(self.num + self.prime - other.num, self.prime); // Ensuring positive result
        FieldElement::new(num, self.prime)
    }
}

// Multiplies two FieldElement values
impl ops::Mul<Self> for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }
        let num = Self::modulo(self.num * other.num, self.prime);
        FieldElement::new(num, self.prime)
    }
}

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
    fn test_new() {
        let a = FieldElement::new(2u32, 31u32);
        assert_eq!(a.number, BigUint::from(2u32));
        assert_eq!(a.prime, BigUint::from(31u32));
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        FieldElement::new(32u32, 31u32);
    }

    #[test]
    fn test_eq() {
        let a = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let b = FieldElement::new(BigUint::from(2u32), BigUint::from(31u32));
        let c = FieldElement::new(BigUint::from(15u32), BigUint::from(31u32));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a == b);
        assert!(a != c);
    }
    /*
    #[test]
    fn test_add() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(15, 31);
        assert_eq!(a + b, FieldElement::new(17, 31));
        let a = FieldElement::new(17, 31);
        let b = FieldElement::new(21, 31);
        assert_eq!(a + b, FieldElement::new(7, 31));
    }

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
