use bitcoin::FieldElement;
use bitcoin::Point;
use bitcoin::PrivateKey;
use bitcoin::S256Point;
use bitcoin::Signature;
use num_bigint::BigUint;

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

    // S256Point
    let s256point: S256Point = S256Point::generator();
    println!("{}", s256point.get_point());
    println!("{}", s256point.get_point().clone() * BigUint::from(2u32));

    let n = BigUint::parse_bytes(S256Point::BASE_ORDER, 16).unwrap();
    println!("{}", s256point.get_point().clone() * n);

    // Verifying signature
    let z = b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423";
    let r = b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6";
    let s = b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
    let px = b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574";
    let py = b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4";
    let point = S256Point::new(Some(px), Some(py));

    // Private key
    let private_key =
        PrivateKey::new(b"5a3028a13c7c5b0b455c155198de1a4b3a75a9009b972cd17577c0bd6a3a0949");
    println!("Private Key: {}", private_key);
    println!("Signature: {}", private_key.sign(b"55"));
}
