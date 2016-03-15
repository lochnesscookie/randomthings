
use factor::factorial;

// Finds the nth lexigraphical permutation of digits from 0 to x - 1
pub fn lexigraphicalpermutation(n: usize, x: usize) -> usize {
    let mut digits = (0..x).collect::<Vec<usize>>();
    let mut permutation = Vec::new();
    let mut fact = factorial(x);
    let mut num = n - 1;
    for i in 0..x {
        fact /= x - i;
        permutation.push(digits[num / fact]);
        digits.remove(num / fact);
        num %= fact;
    }
    let mut number = 0;
    for i in 0..x {
        number += permutation[i] * 10f64.powi((x - i - 1) as i32) as usize;
    }
    number
}
