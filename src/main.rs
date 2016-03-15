
fn main(){
    let max = 0;
    let primes = make_primes(1000).iter().rev();
    for p in primes {
        if max > p {
            break;
        } else {
            for i in 
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
        let signa = i * 2 - 1;
        for b in 0..2 {
            let signb = i * 2 - 1;
            for j in 0..1000 {
                if is_prime(j * j + j * i *
