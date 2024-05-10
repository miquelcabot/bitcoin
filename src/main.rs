mod field_element;
mod point;

use field_element::FieldElement;
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
    let prime = 223;
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);
    let x = Point::new(
        Some(FieldElement::new(192, prime)),
        Some(FieldElement::new(105, prime)),
        a,
        b,
    );
    let y = Point::new(
        Some(FieldElement::new(17, prime)),
        Some(FieldElement::new(56, prime)),
        a,
        b,
    );
    println!("{}", x);
    println!("{}", y);
    println!("{}", x + y);

    // Point multiplication
    /* let x = Point::new(
        Some(FieldElement::new(47, prime)),
        Some(FieldElement::new(71, prime)),
        a,
        b,
    );
    for s in 1..21 {
        let result = x * s;
        println!(
            "{}*(47,71)=({},{})",
            s,
            result.get_x().unwrap(),
            result.get_y().unwrap()
        );
    } */
}
