
use std::ops::{Add, Mul, Index, IndexMut};
use std::iter;
use std::clone::Clone;
use std::fmt;
use std;

// A BigNum is a representation of an arbitrarily large number. BigNums can be thought of as a
// wrapper over a vector of digits
#[derive(Clone, Debug)]
pub struct BigNum {
    numvec: Vec<usize>
}

impl BigNum {
    // Returns the numbers of digits in the BigNum
    pub fn len(&self) -> usize {
        self.numvec.len()
    }
    // Allows one to push a digit onto a BigNum, as if it were a vector
    fn push(&mut self, elem: usize){
        self.numvec.push(elem);
    }
    // Takes care of 'carrying' during addition, multiplication, etc.
    fn fix(&mut self){
        for i in 0..(self.len() - 1){
            self[i + 1] += self[i] / 10;
            self[i] %= 10;
        }
        loop{
            if self[self.len() - 1] >= 10 {
                let mut n = self[self.len() - 1] / 10;
                self.push(n);
                n = self.len() - 2;
                self[n] %= 10;
            } else {
                break;
            }
        }
    }
    // Splits a BigNum into two parts, as if it were a string
    fn split(&self, index: usize) -> (BigNum, BigNum) {
        let (numvec1, numvec2) = self.numvec.split_at(index);
        ( BigNum { numvec: numvec1.to_owned() }, BigNum { numvec: numvec2.to_owned()} )
    }
    // Creates a BigNum with a usize
    pub fn new(num: usize) -> BigNum {
        BigNum {
            numvec: num.to_string().chars().rev()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        }
    }
    // Returns a vector of digits in a correct numerical format
    pub fn todigits(self) -> Vec<usize>{
        let mut digits: Vec<usize> = Vec::new();
        for d in self.numvec.into_iter().rev() {
            digits.push(d);
        }
        digits
    }
}

// Allows digits of BigNums to be accessed immutably with array indices (foo[x])
impl Index<usize> for BigNum {
    type Output = usize;

    fn index(&self, index: usize) -> &usize {
        &self.numvec[index]
    }
}

// Allows digits of BigNums to be accessed mutably with array indices (foo[x])
impl IndexMut<usize> for BigNum {
    fn index_mut(&mut self, index: usize) -> &mut usize {
        &mut self.numvec[index]
    }
}

// Implements adding for BigNums
impl Add for BigNum {
    type Output = BigNum;

    fn add(self, other: BigNum) -> BigNum {
        let sumlen = std::cmp::max(self.len(), other.len());
        let mut sum = BigNum {
            numvec: iter::repeat(0).take(sumlen).collect::<Vec<usize>>()
        };
        for i in 0.. {
            if self.len() <= i && other.len() <= i {
                break;
            } else if self.len() <= i {
                sum[i] = other[i];
            } else if other.len() <= i {
                sum[i] = self[i];
            } else {
                sum[i] = self[i] + other[i];
            }
        }
        sum.fix();
        sum
    }
}

// Implements multiplication for BigNums using the Karatsuba Algorithm
impl Mul for BigNum {
    type Output = BigNum;

    fn mul(self, other: BigNum) -> BigNum {
        let productlen = self.len() + other.len() - 1;
        let mut product = BigNum {
            numvec: iter::repeat(0).take(productlen).collect::<Vec<usize>>()
        };
        if self.len() == 1 {
            for i in 0..other.len(){
                product[i] = other[i] * self[0];
            }
        } else if other.len() == 1 {
            for i in 0..self.len(){
                product[i] = self[i] * other[0];
            }
        } else {
            let (split1, split2) = (self.len() / 2, other.len() / 2);
            let (sub2, sub1) = self.split(split1);
            let (sub4, sub3) = other.split(split2);
            let mut product1 = iter::repeat(0).take(split1 + split2).collect::<Vec<usize>>();
            let mut product2 = iter::repeat(0).take(split2).collect::<Vec<usize>>();
            let mut product3 = iter::repeat(0).take(split1).collect::<Vec<usize>>();
            product1.extend( (sub1.clone() * sub3.clone()).numvec );
            product2.extend( (sub2.clone() * sub3).numvec );
            product3.extend( (sub1 * sub4.clone()).numvec );
            let product4 = (sub2 * sub4).numvec;
            product = BigNum{ numvec: product1 } + 
                BigNum{ numvec: product2 } + 
                BigNum{ numvec: product3 } + 
                BigNum{ numvec: product4 };
        }
        product.fix();
        product
    }
}


impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in self.numvec.iter().rev() {
            try!(write!(f, "{}", n));
        }
        write!(f, "")
    }
}
