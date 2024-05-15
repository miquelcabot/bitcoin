mod field_element;
//mod point;

use field_element::FieldElement;
use num_bigint::BigUint;
//use point::Point;

fn main() {
    // FieldElement
    /* let prime = [
        // 111747865276470275289331353002924822637971113597819074632765509809656264564821
        // f70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055
        0x3b8aa055, 0xbb15d4f3, 0x966d4cb8, 0x73ac8260, 0x6b3dc012, 0x6faadba1, 0x18c335ec,
        0xf70f0ce4,
    ]; */
    let prime = BigUint::parse_bytes(b"f70f0ce418c335ec6faadba16b3dc01273ac8260966d4cb8bb15d4f33b8aa055", 16).unwrap();
    let x = BigUint::parse_bytes(b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949", 16).unwrap();
    let y = BigUint::parse_bytes(b"923051f0a7a42d04bd25d1d4f65b4e51a365d8df764ea0ad02f8f576008dec00", 16).unwrap();

    // let prime = BigUint::from_slice(&prime);

    let a = FieldElement::new(x, prime.clone());
    let b = FieldElement::new(y, prime.clone());

    println!("{:x}", a.get_number());
    println!("{:x}", b.get_number());
    println!("{}", a == b);
    println!("{:x}", (a.clone() + b.clone()).get_number());
    println!("{:X}", (b.clone() + b.clone()).get_number());
    println!("{:x}", (a.clone() + b.clone() + b.clone()).get_number());
    /*println!("{:x}", (prime * prime) % prime);
    println!("{}", b);
    println!("{}", a == b);
    println!("{}", a + b);
    println!("{}", b - a);
    println!("{}", (prime - 1));
    println!("{}", (prime - 10));
    println!("{}", (prime - 1) * (prime - 10));
    println!("{}", a.pow(U256::from(4)));
    println!("{}", a / b);

    let x = U256::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81797");
    let y = U256::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    0xca8a71300f7abe90b6f502095fd5f3546f932759268114914cc36861775185a8a3b474b2ed3d74af5131800f6b91104c60d50e23454567808f4c086c6868c46d

    let c = FieldElement::new(x, y);


    let c = c.pow(U256::from(2));

    println!("{}", c);*/
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
