use std::{fmt, ops};
use uint::construct_uint;

// U256 with 256 bits consisting of 4 x 64-bit words
construct_uint! {
    pub struct U256(4);
}

#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    number: U256,
    prime: U256,
}

impl FieldElement {
    // Getter for number
    pub fn get_number(&self) -> U256 {
        self.number
    }

    // Getter for prime
    pub fn get_prime(&self) -> U256 {
        self.prime
    }

    // Constructs a new FieldElement, ensuring the value is within the field range
    pub fn new<T: Into<U256>>(number: T, prime: T) -> Self {
        let num_u256 = number.into();
        let prime_u256 = prime.into();

        if num_u256 >= prime_u256 || num_u256 < U256::from(0) {
            panic!(
                "Num {} not in field range 0 to {}",
                num_u256,
                prime_u256 - 1
            );
        }
        Self {
            number: num_u256,
            prime: prime_u256,
        }
    }

    // Exponentiates a FieldElement value
    pub fn pow(&self, exponent: U256) -> Self {
        let number = modular_pow(self.number, exponent, self.prime);
        FieldElement::new(number, self.prime)
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement({} in F_{})", self.number, self.prime)
    }
}

// Equality is automatically derived and does not need to be manually implemented
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.prime == other.prime
    }
}

// Adds two FieldElement values
impl ops::Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }
        let number = modular_add(self.number, other.number, self.prime);
        FieldElement::new(number, self.prime)
    }
}

// Subtracts two FieldElement values
impl ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }
        let number = modular_sub(self.number, other.number, self.prime);
        FieldElement::new(number, self.prime)
    }
}

// Multiplies two FieldElement values
impl ops::Mul<Self> for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }
        let number = modular_mul(self.number, other.number, self.prime);
        FieldElement::new(number, self.prime)
    }
}

// Multiplies a FieldElement by a scalar
impl ops::Mul<i32> for FieldElement {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        let number = modular_mul(self.number, U256::from(other), self.prime);
        FieldElement::new(number, self.prime)
    }
}

// Multiplies a scalar by a FieldElement
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
        if let Some(inv) = modular_inverse(other.number, self.prime) {
            let number = modular_mul(self.number, inv, self.prime);
            FieldElement::new(number, self.prime)
        } else {
            panic!("No multiplicative inverse exists for {}", other.number);
        }
    }
}

// Modular addition
fn modular_add(a: U256, b: U256, modulus: U256) -> U256 {
    let (result, overflow) = a.overflowing_add(b);
    if overflow || result >= modulus {
        result.overflowing_sub(modulus).0
    } else {
        result
    }
}

// Modular subtraction
fn modular_sub(a: U256, b: U256, modulus: U256) -> U256 {
    if a >= b {
        a - b
    } else {
        let (result, _) = modulus.overflowing_sub(b - a);
        result
    }
}

// Modular multiplication
fn modular_mul(a: U256, b: U256, modulus: U256) -> U256 {
    let mut result = U256::zero();
    let mut temp_a = a;
    let mut temp_b = b;

    while !temp_b.is_zero() {
        if temp_b.low_u64() & 1 == 1 {
            let (new_result, overflow) = result.overflowing_add(temp_a);
            result = if overflow || new_result >= modulus {
                new_result.overflowing_sub(modulus).0
            } else {
                new_result
            };
        }
        temp_a = temp_a.overflowing_add(temp_a).0;
        if temp_a >= modulus {
            temp_a = temp_a.overflowing_sub(modulus).0;
        }
        temp_b = temp_b >> 1;
    }

    result
}

// Modular inverse using the Extended Euclidean Algorithm
fn modular_inverse(a: U256, modulus: U256) -> Option<U256> {
    let mut t = U256::zero();
    let mut new_t = U256::one();
    let mut r = modulus;
    let mut new_r = a;

    while !new_r.is_zero() {
        let quotient = r / new_r;

        let temp_t = t;
        t = new_t;
        new_t = modular_sub(temp_t, modular_mul(quotient, new_t, modulus), modulus);

        let temp_r = r;
        r = new_r;
        new_r = temp_r - quotient * new_r;
    }

    if r > U256::one() {
        return None; // No inverse exists
    }

    if t < U256::one() {
        return None; // No inverse exists
    }

    Some(t)
}

// Modular exponentiation
fn modular_pow(base: U256, exponent: U256, modulus: U256) -> U256 {
    let mut result = U256::one();
    let mut base = base % modulus;
    let mut exponent = exponent;

    while !exponent.is_zero() {
        if exponent.low_u64() & 1 == 1 {
            result = modular_mul(result, base, modulus);
        }
        exponent = exponent >> 1;
        base = modular_mul(base, base, modulus);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = FieldElement::new(2, 31);
        assert_eq!(a.number, U256::from(2));
        assert_eq!(a.prime, U256::from(31));
        assert_eq!(a.get_number(), U256::from(2));
        assert_eq!(a.get_prime(), U256::from(31));
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        FieldElement::new(32, 31);
    }

    #[test]
    fn test_eq() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(2, 31);
        let c = FieldElement::new(15, 31);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a == b);
        assert!(a != c);
    }

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
    }
}
