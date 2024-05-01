struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    // Constructs a new FieldElement, ensuring the value is within the field range
    fn new(num: i32, prime: i32) -> Result<Self, String> {
        if num >= prime || num < 0 {
            let error = format!("Num {} not in field range 0 to {}", num, prime - 1);
            return Err(error);
        }
        Ok(Self { num, prime })
    }

    // Adds two FieldElement values
    fn add(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different Fields".to_string());
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    // Subtracts two FieldElement values
    fn sub(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot subtract two numbers in different Fields".to_string());
        }
        let num = (self.num - other.num + self.prime) % self.prime; // Ensuring positive result
        FieldElement::new(num, self.prime)
    }

    // Multiplies two FieldElement values
    fn mul(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot multiply two numbers in different Fields".to_string());
        }
        let num = (self.num * other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    // Exponentiates a FieldElement value
    fn pow(&self, exponent: i32) -> Result<Self, String> {
        let n = exponent % (self.prime - 1);
        let num = self.num.pow(n as u32) % self.prime;
        FieldElement::new(num, self.prime)
    }

    // Divides one FieldElement by another using Fermat's Little Theorem
    fn div(&self, other: &FieldElement) -> Result<Self, String> {
        if self.prime != other.prime {
            return Err("Cannot divide two numbers in different Fields".to_string());
        }
        // Calculate other's multiplicative inverse using Fermat's Little Theorem
        let inv = other.num.pow((self.prime - 2) as u32) % self.prime;
        let num = (self.num * inv) % self.prime;
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
