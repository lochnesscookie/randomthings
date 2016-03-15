
use std::iter;

// Finds the amicable numbers up to x.
pub fn amicables(x: usize) -> Vec<usize> {
    let mut amicables: Vec<usize> = Vec::new();
    let mut list: Vec<usize> = iter::repeat(0).take(x * 3).collect::<Vec<usize>>();
    for i in 1..x {
        for j in 2..x * 3 / i {
            list[i * j] += i;
        }
    }
    println!("foo");
    for i in 0..x {
        if list[list[i]] == i && i != list[i] {
            amicables.push(i)
        }
    }
    amicables
}
