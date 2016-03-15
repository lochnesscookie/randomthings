
use std::iter;
use factor::abundantnums;

// Returns the sum of all the numbers that cannot be expressed as the sum of two abundant numbers
pub fn nonabundantsums() -> usize {
    const UPPERLIMIT: usize = 28123;
    let abundantnums = abundantnums(UPPERLIMIT);
    let mut listnums = iter::repeat(true).take(UPPERLIMIT + 1).collect::<Vec<bool>>();
    let mut sum = 0;
    for i in &abundantnums {
        for j in &abundantnums {
            if *i + *j <= UPPERLIMIT {
                listnums[*i + *j] = false;
            } else {
                break;
            }
        }
    }
    for i in 0..UPPERLIMIT{
        if listnums[i] {
            sum += i;
        }
    }
    sum
}
