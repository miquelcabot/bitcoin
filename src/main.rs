mod field_element;
mod point;

use field_element::FieldElement;
use point::Point;

fn main() {
    let a = FieldElement::new(15, 31).unwrap();
    let b = FieldElement::new(5, 31).unwrap();
    println!("{}", a.add(&b).unwrap());
    println!("{}", a.sub(&b).unwrap());
    println!("{}", a.mul(&b).unwrap());
    println!("{}", a.pow(4).unwrap());
    println!("{}", a.div(&b).unwrap());

    let p1 = Point::new(Some(2), Some(5), 5, 7).unwrap();
    let p2 = Point::new(Some(2), Some(-5), 5, 7).unwrap();
    let p3 = Point::new(Some(3), Some(7), 5, 7).unwrap();
    println!("{}", p1);
    println!("{}", p2);
    println!("{}", p3);
    println!("{}", p1.add(&p2).unwrap());
    println!("{}", p2.add(&p3).unwrap());
}
