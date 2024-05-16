mod field_element;
mod point;

use field_element::FieldElement;
use point::Point;

fn main() {
    // FieldElement
    let prime = b"f70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055";
    let x = b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949";
    let y = b"c23051f0a7a42d04bd25d1d4f65b4e51a365d8df764ea0ad02f8f576008dec00";

    let a = FieldElement::from_bytes(x, prime);
    let b = FieldElement::from_bytes(y, prime);

    println!("{:x}", a.get_number());
    println!("{:x}", b.get_number());
    println!("{}", a == b);
    println!("{:x}", (a.clone() + b.clone()).get_number());
    println!("{:x}", (a.clone() * b.clone()).get_number());
    println!("{:x}", (a.clone() - b.clone()).get_number());
    println!("{:x}", (a.clone() - b.clone()).get_number());
    println!("{:x}", (a.clone() * 5).get_number());
    println!("{:x}", (5 * a.clone()).get_number());
    println!("{:x}", (a.clone() / b.clone()).get_number());
    println!("{:x}", a.clone().pow(5u32).get_number());
    /*
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

    println!(
        "{}, {}, {}, {}",
        x.get_x().unwrap(),
        x.get_y().unwrap(),
        x.get_a(),
        x.get_b()
    );

    let z = Point::new(
        Some(FieldElement::new(15, prime)),
        Some(FieldElement::new(86, prime)),
        a,
        b,
    );
    println!("{}", z * 6);
    println!("{}", 6 * z);
    println!("{}", z * 7);
    println!("{}", 7 * z);*/
}
