# field_element_point

This is a simple implementation of a field element point in Rust. It is a point on an elliptic curve over a finite field. The field is defined by a prime number and the curve is defined by two coefficients. The point is defined by its x and y coordinates.

The point can be added to another point, negated, or multiplied by a scalar. The scalar multiplication is implemented using the double-and-add algorithm.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
field_element_point = "0.1.0"
```

## Usage

```rust
use field_element_point::FieldElementPoint;

fn main() {
    let p = FieldElementPoint::new(2, 3, 5, 7, 13);
    let q = FieldElementPoint::new(2, 3, 5, 7, 13);
    let r = p + q;
    println!("{:?}", r);
}
```
