mod field_element;
mod field_element_point;
mod point;

use field_element::FieldElement;
use field_element_point::FieldElementPoint;
use point::Point;

fn main() {
    // FieldElement
    let a = FieldElement::new(15, 31);
    let b = FieldElement::new(5, 31);
    println!("{}", a);
    println!("{}", b);
    println!("{}", a == b);
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a.pow(4));
    println!("{}", a / b);

    // Point
    let p1 = Point::new(Some(2), Some(5), 5, 7);
    let p2 = Point::new(Some(2), Some(-5), 5, 7);
    let p3 = Point::new(Some(3), Some(7), 5, 7);
    println!("{}", p1);
    println!("{}", p2);
    println!("{}", p3);
    println!("{}", p1 + p2);
    println!("{}", p2 + p3);

    // FieldElementPoint
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let x = FieldElementPoint::new(
        Some(FieldElement::new(192, prime)),
        Some(FieldElement::new(105, prime)),
        a,
        b,
    );
    let y = FieldElementPoint::new(
        Some(FieldElement::new(17, prime)),
        Some(FieldElement::new(56, prime)),
        a,
        b,
    );
    println!("{}", x);
    println!("{}", y);
    println!("{}", x + y);
}
