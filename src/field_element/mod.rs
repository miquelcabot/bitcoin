use std::{fmt, ops};
use uint::construct_uint;

// U256 with 256 bits consisting of 4 x 64-bit words
construct_uint! {
    pub struct U256(4);
}

#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    num: U256,
    prime: U256,
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement({} in F_{})", self.num, self.prime)
    }
}

// Equality is automatically derived and does not need to be manually implemented
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

// Adds two FieldElement values
impl ops::Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot operate with two numbers in different Fields");
        }
        let num = Self::modulo(self.num + other.num, self.prime);
        FieldElement::new(num, self.prime)
    }
}

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
}

impl FieldElement {
    // Getter for num
    pub fn get_num(&self) -> U256 {
        self.num
    }

    // Getter for prime
    pub fn get_prime(&self) -> U256 {
        self.prime
    }

    // Constructs a new FieldElement, ensuring the value is within the field range
    pub fn new<T: Into<U256>>(num: T, prime: T) -> Self {
        let num_u256 = num.into();
        let prime_u256 = prime.into();

        if num_u256 >= prime_u256 || num_u256 < U256::from(0) {
            panic!(
                "Num {} not in field range 0 to {}",
                num_u256,
                prime_u256 - 1
            );
        }
        Self {
            num: num_u256,
            prime: prime_u256,
        }
    }

    // Exponentiates a FieldElement value
    pub fn pow(&self, exponent: U256) -> Self {
        let n = Self::modulo(exponent, self.prime - 1);
        let num = Self::mod_pow(self.num, n, self.prime);
        FieldElement::new(num, self.prime)
    }

    fn modulo(a: U256, b: U256) -> U256 {
        let r = a % b;
        if r < U256::from(0) {
            r + Self::abs(b)
        } else {
            r
        }
    }

    fn abs(a: U256) -> U256 {
        if a < U256::from(0) {
          a.overflowing_neg().0
        } else {
            a
        }
    }

    fn mod_pow(x: U256, y: U256, z: U256) -> U256 {
        if z == U256::from(1) {
            return U256::from(0);
        }
        let mut result = U256::from(1);
        let mut base = x % z;
        let mut exponent = y;

        while exponent > U256::from(0) {
            if exponent % 2 == U256::from(1) {
                result = result * base % z;
            }
            exponent >>= 1;
            base = base * base % z;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = FieldElement::new(2, 31);
        assert_eq!(a.num, U256::from(2));
        assert_eq!(a.prime, U256::from(31));
        assert_eq!(a.get_num(), U256::from(2));
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
