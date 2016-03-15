
fn main(){
    let max = 0;
    let primes = make_primes(1000).iter().rev();
    for p in primes {
        if max > p {
            break;
        } else {
            max_primes = max_quadratic_primes(*p, &primes);
            if max < max_primes {
                max = max_primes;
            }
        }
    }
    println!("{}",max);
}

fn make_primes(upper: usize) -> Vec<usize>{
    let mut list = (0..).take(upper).collect::<Vec<usize>>();
    let mut primes = Vec::new();
    list[1] = 0;
    for i in 0..upper {
        match list[i]{
            0 => continue,
            _ => {
                for j in 2..((upper - 1) / i) + 1 {
                    list[i * j] = 0;
                }
                primes.push(i);
            }
        }
    }
    primes
}
    
fn is_prime(num: usize, primelist: &Vec<usize>) -> bool {
    for p in primelist {
        if num == *p {
            return true;
        }
    }
    false
}

fn max_quadratic_primes(num: usize, primelist: &Vec<usize>) -> usize {
    let mut max = 0;
    for a in 0..2 {
        let sign = i * 2 - 1;
        for coef in 0..1000 {
            for i in 0.. {
                let result = i * i + sign * (coef * i) as isize + num as isize;
                if (result <= 1 || !is_prime(result, primelist)) && max < i {
                    max = i;
                }
            }
        }
    }
    max
}
