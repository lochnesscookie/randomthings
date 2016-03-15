
// Note: `numletcount` means the number of letters written when numbers are written out in words

// Finds the numletcount when you write out all the numbers from one to x
pub fn numletcount(x: usize) -> usize {
    let mut sum = 0;
    for i in 1..x + 1 {
        sum += numletcountone(i % 10);
        if i % 100 >= 20 {
            sum += numletcountten(i % 100 / 10);
        } else if i % 100 >= 10 {
            sum += numletcountteen(i % 10);
        } else {
            sum += 0;
        }
        if i % 1000 >= 100 {
            sum += numletcountone(i % 1000 / 100);
            if i % 100 == 0 {
                sum += 7;
            } else {
                sum += 10;
            }
        } else {
            sum += 0
        }
        if i >= 1000 {
            sum += numletcountone(i / 1000);
            sum += 8;
        }
    }
    sum
}


// Find numletcount for a teen 10 + x (the returned value should be added to numletcountone(x))
fn numletcountteen(x: usize) -> usize {
    match x {
        0 | 1 | 2 | 3 | 5 | 8 => 3,
        _ => 4
    }
}

// Find numletcount for x * 10 (x != 0, 10)
fn numletcountten(x: usize) -> usize {
    match x {
        4 | 5 | 6 => 5,
        2 | 3 | 8 | 9 => 6,
        7 => 7,
        _ => 0
    }
}

// Find numletcount for a digit
fn numletcountone(x: usize) -> usize {
    match x {
        1 | 2 | 6 => 3,
        4 | 5 | 9 => 4,
        3 | 7 | 8 => 5,
        _ => 0
    }
}
