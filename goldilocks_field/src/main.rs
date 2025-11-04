// define Goldilocks Prime
pub const GOLDILOCKS_P: u64 = 0xFFFFFFFF00000001;

// Create Field Element struct

#[derive(Debug)]
pub struct FieldElement(pub u64);

// Constructor to ensure that the Element is within the Prime Field

impl FieldElement {
    pub fn new(value: u64) -> Self {
        Self(value % GOLDILOCKS_P)
    }
}

// Addition in the field

use std::ops::Add;

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let sum = self.0 + other.0;

        // wrap around overflow

        let reduced = if sum >= GOLDILOCKS_P {
            sum - GOLDILOCKS_P
        } else {
            sum
        };
        FieldElement(reduced)
    }
}

fn main() {
    //test wrapping
    let x = FieldElement::new(GOLDILOCKS_P);
    let y = FieldElement::new(9);
    let result = x + y;
    println!("Result {:?}", result);
}