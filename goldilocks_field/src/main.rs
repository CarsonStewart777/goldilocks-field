// define Goldilocks Prime
pub const GOLDILOCKS_P: u64 = 0xFFFFFFFF00000001;

// Create Field Element struct

#[derive(Debug, Clone, Copy)]
pub struct FieldElement(pub u64);

// Constructor to ensure that the Element is within the Prime Field

impl FieldElement {
    pub fn new(value: u64) -> Self {
        Self(value % GOLDILOCKS_P)
    }
}

// folding

impl FieldElement {
    fn reduce(prod: u128) -> u64 {
        // Split into lower and upper 64 bits
        let xl = prod as u64;
        let xh = (prod >> 64) as u64;

        // Step 1: fold low bits
        let (a, e) = xl.overflowing_add(xl << 32);

        // Step 2: adjust for carry
        let b = a.wrapping_sub(a >> 32).wrapping_sub(e as u64);

        // Step 3: subtract upper bits with borrow
        let (r, c) = xh.overflowing_sub(b);

        // Step 4: final adjustment to ensure 0 <= r < GOLDILOCKS_P
        r.wrapping_sub(0u32.wrapping_sub(c as u32) as u64)
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

// subtraction in the field

impl std::ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let diff = if self.0 >= other.0 {
            self.0 - other.0
        } else {
            // wrap around underflow
            GOLDILOCKS_P - (other.0 - self.0)
        };
        FieldElement(diff)
    }
}

// multiplication in the field

impl std::ops::Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
       let prod = (self.0 as u128) * (other.0 as u128);
       let reduced = FieldElement::reduce(prod);
       FieldElement(reduced)
    }
}

// power helper for modular inverse
impl FieldElement {
pub fn pow(self, mut exp: u64) -> Self {
    let mut base = self;
    let mut result = FieldElement::new(1);

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base;
        }
        base = base * base;
        exp >>= 1;
    }
    result
}
}

// modular inverse in the field
impl FieldElement {
    pub fn inverse(self) -> Self {
        self.pow(GOLDILOCKS_P - 2)
    }
}

// negation

use std::ops::Neg;
impl Neg for FieldElement {
    type Output = Self;
    fn neg(self) -> Self {
        if self.0 == 0 {
            self
        } else {
            FieldElement(GOLDILOCKS_P - self.0)
        }
    }
}

// division 

use std::ops::Div;
impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inverse()

    } 
}

fn main() {
    //test wrapping
    let x = FieldElement::new(GOLDILOCKS_P);
    let y = FieldElement::new(9);
    let res1 = x + y;
    println!("---Addition---");
    println!("The sum of X and Y is: {:?}", res1);
    let a = FieldElement::new(9);
    let b = FieldElement::new(10);
    let res2 = a - b;
    println!("\n---Subtraction---");
    println!("The difference between a and b is: {:?}", res2);
    println!("\n---Multiplication---");
    let n = FieldElement::new(GOLDILOCKS_P -1);
    let p = FieldElement::new(3);
    
    let res3 = n * p;
    println!("The product of n and p is: {:?}", res3);
    println!("\n---Modular Inverse---");
    let s = FieldElement::new(3);
    let inv_s = s.inverse();
    let res4 = s * inv_s;
    println!("s * s⁻¹ = {:?}", res4);
}