
use bignum::*;

// Finds the index of the first fibonacci number with x digits
pub fn fibonacciwithdigits(x: usize) -> usize {
    let mut a = BigNum::new(1);
    let mut b = BigNum::new(1);
    let mut i = 2;
    while b.len() < 1000 {
        let c = b.clone();
        b = a + b;
        a = c;
        i += 1;
    }
    i
}
