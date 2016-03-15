
use std::iter;

// Returns the period of the reciprocal of x
pub fn repeatlength(x: usize) -> usize {
    let mut num = 9;
    let mut ninelength = 2;
    loop {
        let remainder = num % x;
        if remainder == 0 {
            return ninelength
        } else {
            num = remainder * 10 + 9;
            ninelength += 1;
        }
    }
}

// Finds the number up to x whose repeatlength is the largest
pub fn largestrepeatlength(x: usize) -> usize {
    let mut totry = iter::repeat(true).take(x).collect::<Vec<bool>>();
    let mut max = 0;
    let mut maxnum = 0;
    for i in 1..x {
        if totry[i] { let repeatlength = repeatlength(i);
            if repeatlength > max {
                max = repeatlength;
                maxnum = i;
            }
            let mut twos = 0;
            let mut fives = 0;
            loop {
                if i * (2f64.powi(twos) as usize) * (5f64.powi(fives) as usize) < x {
                    totry[i * 2f64.powi(twos) as usize * 5f64.powi(fives) as usize] = false;
                    twos += 1;
                } else if i * (5f64.powi(fives) as usize) < x {
                    twos = 0;
                    fives += 1;
                } else {
                    break;
                }
            }
        }
    }
    maxnum
}
