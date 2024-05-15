mod field_element;
mod point;

use field_element::{FieldElement, U256};
use point::Point;

fn main() {
    // FieldElement
    let prime = "0xf70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055";
    let x = "0x5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949";
    let y = "0x923051f0a7a42d04bd25d1d4f65b4e51a365d8df764ea0ad02f8f576008dec00";

    let a = FieldElement::new(x, prime);
    let b = FieldElement::new(y, prime);
    println!("{}", a);
    println!("{}", b);
    println!("{}", a == b);
    println!("{}", a + b);
    println!("{}", a.get_number() % a.get_prime());
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a.pow(U256::from(4)));
    println!("{}", a / b);

    // Point
    // let prime = 223;
    let a = FieldElement::new(
        "0x7020519de38c5ae65fba0a6173ce9cf44b0ff4e97aa582cb1f792159d0320c0d",
        prime,
    );
    let b = FieldElement::new(
        "0xa2503cc7fae4e233b3b8ef46ab61732784f2d4075301cfcce6761ac5b85f2603",
        prime,
    );
    let x = Point::new(
        Some(FieldElement::new(
            "0x619a9cf8d8463430cf75f60e5a58e537ff96835f9b2d7bbdc10d96e73e6c6ad",
            prime,
        )),
        Some(FieldElement::new(
            "0x14730726385fcece2e67856a72ad8e115ecbe2b4cb0a1e78fbc70e4c17047f75",
            prime,
        )),
        a,
        b,
    );
    let y = Point::new(
        Some(FieldElement::new(
            "0x619a9cf8d8463430cf75f60e5a58e537ff96835f9b2d7bbdc10d96e73e6c6ad",
            prime,
        )),
        Some(FieldElement::new(
            "0xeebc5f9bdc703113d71ba9fcb7267113dd34f007bf946248074bcd3bcb23b8a",
            prime,
        )),
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
        Some(FieldElement::new(
            "0xc900d3d6b4d74941ff9472c539defa758d0dd7e73029ffdd2d7cf21d5e1ad08d",
            prime,
        )),
        Some(FieldElement::new(
            "0xb83df48b972aa8e43bbd7bb5576e9ac3248b26a4c08441772b21da95d526af5f",
            prime,
        )),
        a,
        b,
    );
    println!("{}", z * 6);
    println!("{}", 6 * z);
    println!("{}", z * 7);
    println!("{}", 7 * z);
}
