# bitcoin

This is a simple implementation of the secp256k1 elliptic curve in Rust. It is a work in progress and is not intended for production use. The goal is to implement the elliptic curve and the digital signature algorithm (ECDSA) used in Bitcoin.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bitcoin = "0.1.0"
```

## Usage

```rust
use s256point::S256Point;

fn main() {
    let s256point: S256Point = S256Point::new();
    println!("{}", s256point.get_point());
    println!("{}", s256point.get_point().clone() * BigUint::from(2u32));

    let n = BigUint::parse_bytes(S256Point::BASE_ORDER, 16).unwrap();
    println!("{}", s256point.get_point().clone() * n);
}
```
