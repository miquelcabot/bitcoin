mod field_element;

use field_element::FieldElement;

fn main() {
    let a = FieldElement::new(15, 31).unwrap();
    let b = FieldElement::new(5, 31).unwrap();
    println!("{}", a.add(&b).unwrap());
    println!("{}", a.sub(&b).unwrap());
    println!("{}", a.mul(&b).unwrap());
    println!("{}", a.pow(4).unwrap());
    println!("{}", a.div(&b).unwrap());
}
