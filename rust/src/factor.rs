
use std::iter;

// Generates a list of primes up to a number x
pub fn makeprimes(x: usize) -> Vec<usize> {
    let mut list = (0..x + 1).collect::<Vec<usize>>();
    list[1] = 0;
    let mut primelist = Vec::new();
    for i in 0..x + 1 {
        match list[i] {
            0 => {},
            _ => {
                for j in 2..x / list[i] + 1 {
                    list[i * j] = 0;
                }
                primelist.push(list[i]);
            }
        }
    }
    primelist
}

// Factors a number and puts the resulting factorization into a Vec<(usize, usize)>
pub fn factorization(x: usize, primelist: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut num = x;
    let mut factorization: Vec<(usize, usize)> = Vec::new();
    for p in 0.. {
        if num != 1 {
            factorization.push((primelist[p], 0));
            loop {
                match num % primelist[p] {
                    0 => {
                        factorization[p].1 += 1;
                        num /= primelist[p];
                    },
                    _ => break
                }
            }
        } else {
            break;
        }
    }
    factorization
}

// Finds all the divisors of a number and puts them in a Vec<usize>
pub fn divisors (x: usize, primelist: &Vec<usize>)-> Vec<usize> {
    let factorization = factorization(x, &primelist);
    let mut divisorlist: Vec<usize> = vec![1];
    for f in factorization {
        match f.1 {
            0 => continue,
            _ => {
                let olddivisors = divisorlist.clone();
                for i in 1..f.1 + 1 {
                    let mut newdivisors = olddivisors.clone().into_iter()
                        .map(|x| x * f.0.pow(i as u32)).collect::<Vec<usize>>();
                    divisorlist.append(&mut newdivisors);
                }
            }
        }
    }
    divisorlist
}

// Returns a Vec<usize> of all the abundant numbers up to x
pub fn abundantnums(x: usize) -> Vec<usize> {
    let mut divisorsums = iter::repeat(0).take(x + 1).collect::<Vec<usize>>();
    let mut abundantnums = Vec::new();
    for i in 1..x / 2 + 1 {
        for j in 2..x / i + 1 {
            divisorsums[i * j] += i;
        }
    }
    for i in 0..x + 1 {
        if divisorsums[i] > i {
            abundantnums.push(i);
        }
    }
    abundantnums
}

// Note: won't work for really big numbers (doesn't use BigNum)
// Finds the factorial (!) of a number
pub fn factorial(x: usize) -> usize {
    let mut product = 1;
    for i in 1..x + 1 {
        product *= i;
    }
    product
}
