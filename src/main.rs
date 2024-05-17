mod field_element;
mod point;

use field_element::FieldElement;
use num_bigint::BigUint;
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

    // Point
    let prime = 223;
    let a = FieldElement::from_int(0, prime);
    let b = FieldElement::from_int(7, prime);
    let x = Point::new(
        Some(FieldElement::from_int(192, prime)),
        Some(FieldElement::from_int(105, prime)),
        a.clone(),
        b.clone(),
    );
    let y = Point::new(
        Some(FieldElement::from_int(17, prime)),
        Some(FieldElement::from_int(56, prime)),
        a.clone(),
        b.clone(),
    );
    println!("{}", x);
    println!("{}", y);
    println!("{}", x.clone() + y.clone());

    println!(
        "{}, {}, {}, {}",
        x.get_x().unwrap(),
        x.get_y().unwrap(),
        x.get_a(),
        x.get_b()
    );

    let z = Point::new(
        Some(FieldElement::from_int(15, prime)),
        Some(FieldElement::from_int(86, prime)),
        a.clone(),
        b.clone(),
    );
    println!("{}", z.clone() * BigUint::from(6u32));
    println!("{}", BigUint::from(6u32) * z.clone());
    println!("{}", z.clone() * BigUint::from(7u32));
    println!("{}", BigUint::from(7u32) * z.clone());

    // secp256k1
    // prime = 2^256 - 2^32 - 977
    let prime = b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f";
    let x = b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let y = b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

    let gx = FieldElement::from_bytes(x, prime);
    let gy = FieldElement::from_bytes(y, prime);
    let a = FieldElement::from_bytes(b"0", prime);
    let b = FieldElement::from_bytes(b"7", prime);

    let g = Point::new(Some(gx), Some(gy), a, b);
    println!("{}", g);

    let n = BigUint::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();
    println!("{}", g * n);
}
