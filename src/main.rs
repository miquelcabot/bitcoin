mod field_element;
//mod point;

use field_element::FieldElement;
use num_bigint::BigUint;
//use point::Point;

fn main() {
    // FieldElement
    let prime = [
        // 111747865276470275289331353002924822637971113597819074632765509809656264564821
        0x3b8aa055, 0xbb15d4f3, 0x966d4cb8, 0x73ac8260, 0x6b3dc012, 0x6faadba1, 0x18c335ec,
        0xf70f0ce4,
    ];
    let prime = BigUint::from_slice(&prime);

    let a = FieldElement::new(15u32, prime.clone());
    let b = FieldElement::new(5u32, prime.clone());

    println!("{}", a);
    println!("{}", b);
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
