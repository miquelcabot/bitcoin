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
use field_element::FieldElement;

fn main() {
    let a = FieldElement::new(5, 7);
    let b = FieldElement::new(3, 7);
    let c = FieldElement::new(1, 7);
    let d = FieldElement::new(1, 7);

    let p1 = FieldElementPoint::new(a, b, c, d);
    let p2 = FieldElementPoint::new(a, b, c, d);

    let p3 = p1 + p2;
    let p4 = p1 * 2;
    let p5 = -p1;
}
```
