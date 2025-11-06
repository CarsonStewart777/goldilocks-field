# goldilocks-field

A minimal Rust implementation of arithmetic over the **Goldilocks prime field**  
(p = 2⁶⁴ − 2³² + 1), used in modern zero-knowledge proof systems and high-performance cryptographic protocols.

---

## Overview

This crate provides:
- Modular arithmetic for the Goldilocks prime field
- Safe and normalized `FieldElement` type
- Implementations of basic field operations:
  - Addition and subtraction
  - Multiplication
  - Negation and inversion (via Fermat’s little theorem)
  - Exponentiation by squaring

The implementation prioritizes correctness and clarity over optimization.  
It is suitable for educational use, prototyping, or as a reference for more advanced systems.

---

## Example

```rust
use goldilocks_field::{FieldElement, GOLDILOCKS_P};

fn main() {
    let a = FieldElement::new(6);
    let b = FieldElement::new(5);
    let result = a + b;
    println!("Result: {:?}", result);
}
