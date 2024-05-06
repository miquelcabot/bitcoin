use std::fmt;

#[derive(Debug)]
pub struct FieldElement {
    num: i64,
    prime: i64,
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement({} in F_{})", self.num, self.prime)
    }
}

impl FieldElement {
    // Constructs a new FieldElement, ensuring the value is within the field range
    pub fn new(num: i64, prime: i64) -> Result<Self, String> {
        if num >= prime || num < 0 {
            let error = format!("Num {} not in field range 0 to {}", num, prime - 1);
            return Err(error);
        }
        Ok(Self { num, prime })
    }

    // Adds two FieldElement values
    pub fn add(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different Fields".to_string());
        }
        let num = Self::modulo(self.num + other.num, self.prime);
        FieldElement::new(num, self.prime)
    }

    // Subtracts two FieldElement values
    pub fn sub(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot subtract two numbers in different Fields".to_string());
        }
        let num = Self::modulo(self.num - other.num + self.prime, self.prime); // Ensuring positive result
        FieldElement::new(num, self.prime)
    }

    // Multiplies two FieldElement values
    pub fn mul(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot multiply two numbers in different Fields".to_string());
        }
        let num = Self::modulo(self.num * other.num, self.prime);
        FieldElement::new(num, self.prime)
    }

    // Exponentiates a FieldElement value
    pub fn pow(&self, exponent: i64) -> Result<Self, String> {
        let n = Self::modulo(exponent, self.prime - 1);
        let num = Self::mod_pow(self.num, n, self.prime);
        FieldElement::new(num, self.prime)
    }

    // Divides one FieldElement by another using Fermat's Little Theorem
    pub fn div(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot divide two numbers in different Fields".to_string());
        }
        // Calculate other's multiplicative inverse using Fermat's Little Theorem
        let inv = Self::mod_pow(other.num, self.prime - 2, self.prime);
        let num = Self::modulo(self.num * inv, self.prime);
        FieldElement::new(num, self.prime)
    }

    fn modulo(a: i64, b: i64) -> i64 {
        let r = a % b;
        if r < 0 {
            r + b.abs()
        } else {
            r
        }
    }

    fn mod_pow(x: i64, y: i64, z: i64) -> i64 {
        if z == 1 {
            return 0;
        }
        let mut result = 1;
        let mut base = x % z;
        let mut exponent = y;

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result * base % z;
            }
            exponent = exponent >> 1;
            base = base * base % z;
        }
        result
    }
}

// Equality is automatically derived and does not need to be manually implemented
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Eq for FieldElement {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = FieldElement::new(2, 31).unwrap();
        let b = FieldElement::new(15, 31).unwrap();
        assert_eq!(a.add(&b).unwrap(), FieldElement::new(17, 31).unwrap());
        let a = FieldElement::new(17, 31).unwrap();
        let b = FieldElement::new(21, 31).unwrap();
        assert_eq!(a.add(&b).unwrap(), FieldElement::new(7, 31).unwrap());
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(29, 31).unwrap();
        let b = FieldElement::new(4, 31).unwrap();
        assert_eq!(a.sub(&b).unwrap(), FieldElement::new(25, 31).unwrap());
        let a = FieldElement::new(15, 31).unwrap();
        let b = FieldElement::new(30, 31).unwrap();
        assert_eq!(a.sub(&b).unwrap(), FieldElement::new(16, 31).unwrap());
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(24, 31).unwrap();
        let b = FieldElement::new(19, 31).unwrap();
        assert_eq!(a.mul(&b).unwrap(), FieldElement::new(22, 31).unwrap());
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(3).unwrap(), FieldElement::new(15, 31).unwrap());
        let a = FieldElement::new(5, 31).unwrap();
        let b = FieldElement::new(18, 31).unwrap();
        assert_eq!(
            a.pow(5).unwrap().mul(&b).unwrap(),
            FieldElement::new(16, 31).unwrap()
        );
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(24, 31).unwrap();
        assert_eq!(a.div(&b).unwrap(), FieldElement::new(4, 31).unwrap());
        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(-3), FieldElement::new(29, 31));
        let a = FieldElement::new(4, 31).unwrap();
        let b = FieldElement::new(11, 31).unwrap();
        assert_eq!(a.pow(-4).unwrap().mul(&b), FieldElement::new(13, 31));
    }
}
