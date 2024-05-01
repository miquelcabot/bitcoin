fn modulo(a: i32, b: i32) -> i32 {
    let r = a % b;
    if r < 0 {
        r + b.abs()
    } else {
        r
    }
}

#[derive(Debug)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    // Constructs a new FieldElement, ensuring the value is within the field range
    pub fn new(num: i32, prime: i32) -> Result<Self, String> {
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
        let num = modulo(self.num + other.num, self.prime);
        FieldElement::new(num, self.prime)
    }

    // Subtracts two FieldElement values
    pub fn sub(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot subtract two numbers in different Fields".to_string());
        }
        let num = modulo(self.num - other.num + self.prime, self.prime); // Ensuring positive result
        FieldElement::new(num, self.prime)
    }

    // Multiplies two FieldElement values
    pub fn mul(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot multiply two numbers in different Fields".to_string());
        }
        let num = modulo(self.num * other.num, self.prime);
        FieldElement::new(num, self.prime)
    }

    // Exponentiates a FieldElement value
    pub fn pow(&self, exponent: i32) -> Result<Self, String> {
        let n = modulo(exponent, self.prime - 1);
        let num = modulo(self.num.pow(n as u32), self.prime);
        FieldElement::new(num, self.prime)
    }

    // Divides one FieldElement by another using Fermat's Little Theorem
    pub fn div(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot divide two numbers in different Fields".to_string());
        }
        // Calculate other's multiplicative inverse using Fermat's Little Theorem
        let inv = modulo(other.num.pow((self.prime - 2) as u32), self.prime);
        let num = modulo(self.num * inv, self.prime);
        FieldElement::new(num, self.prime)
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
    /*
    #[test]
    fn test_pow() {
      let a = FieldElement::new(7, 13).unwrap();
      let exponent = 3;
      let c = FieldElement::new(5, 13).unwrap();
      assert_eq!(a.pow(exponent).unwrap(), c);
    }

    #[test]
    fn test_div() {
      let a = FieldElement::new(7, 13).unwrap();
      let b = FieldElement::new(12, 13).unwrap();
      let c = FieldElement::new(8, 13).unwrap();
      assert_eq!(a.div(&b).unwrap(), c);
    }*/
}
