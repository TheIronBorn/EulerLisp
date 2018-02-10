use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt;

use std::cmp::{PartialOrd, Ordering};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Bignum {
    pub sign: bool,
    pub data: Vec<usize>
}

impl PartialOrd for Bignum {
    fn partial_cmp(&self, other: &Bignum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bignum {
    fn cmp(&self, other: &Bignum) -> Ordering {
        match (self.sign, other.sign) {
            (true, true) => {
                let len = self.data.len();
                let len_other = other.data.len();

                if len > len_other {
                    Ordering::Less
                } else if len < len_other {
                    Ordering::Greater
                } else {
                    for j in 0..len {
                        let res = other.data[len - j - 1].cmp(&self.data[len - j - 1]);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    Ordering::Equal
                }
            },
            (true, false) => {
                Ordering::Less
            },
            (false, true) => {
                Ordering::Greater
            },
            (false, false) => {
                let len = self.data.len();
                let len_other = other.data.len();
                
                if len > len_other {
                    Ordering::Greater
                } else if len < len_other {
                    Ordering::Less
                } else {
                    for j in 0..len {
                        let res = self.data[len - j - 1].cmp(&other.data[len - j - 1]);
                        if res != Ordering::Equal {
                            return res;
                        }
                    }
                    Ordering::Equal
                }
            }
        }
    }
}

// 10^9, this way the product of the sum of two parts still fits inside a u64
pub const CHUNK: usize = 1000000000;
pub const DIGITS: usize = 9;

impl Bignum {
    pub fn new(value: isize) -> Self {
        if value == 0 {
            return Self { sign: false, data: Vec::new() }
        }

        let sign;
        let mut uvalue; 
        if value < 0 {
            sign = true;
            uvalue = (-value) as usize;
        } else {
            sign = false;
            uvalue = value as usize;
        }

        let mut data = Vec::new();
        while uvalue > 0 {
            data.push(uvalue % CHUNK);
            uvalue /= CHUNK;
        }

        Self {
            sign: sign,
            data: data
        }
    }

    pub fn from_chunks(chunks: Vec<usize>) -> Self {
        return Self { sign: false, data: chunks }
    }

    pub fn to_isize(&self) -> isize {
        if self.data.len() > 1 {
            panic!("Bignum is to big for isize");
        } else {
            if self.sign {
                -(*self.data.get(0).unwrap_or(&0) as isize)
            } else {
                (*self.data.get(0).unwrap_or(&0) as isize)
            }
        }
    }

    pub fn num_digits(&self) -> isize {
        let len = self.data.len();

        if len == 0 {
            1
        } else {
            let last = *self.data.get(len - 1).unwrap() as f64;
            (((len - 1) * 9) as isize) + (last.log(10.0) as isize) + 1
        }
    }

    pub fn digits(&self) -> Vec<isize> {
        if self.data.len() == 0 {
            vec![0]
        } else {
            let mut result = Vec::new();
            for (i, chunk) in self.data.iter().enumerate() {
                let mut remaining = DIGITS;
                let mut cur = *chunk;
                while cur > 0{
                    result.push((cur % 10) as isize);
                    cur /= 10;
                    remaining -= 1;
                }

                if i < (self.data.len() - 1) {
                    while remaining > 0 {
                        result.push(0);
                        remaining -= 1;
                    }
                }
            }
            result
        }
    }

    pub fn chunks(&self) -> Vec<isize> {
        self.data.iter().map(|x| *x as isize).collect()
    }
}

// Addition for positive numbers
fn vector_add(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    // Based on Algorithm A, Section 4.3.1, TAoCP Vol. 2
    let n = a.len().max(b.len());
    let mut result = Vec::new();
    let mut carry = 0;

    for j in 0..n {
        let u = *a.get(j).unwrap_or(&0);
        let v = *b.get(j).unwrap_or(&0);

        let res = u + v + carry;
        result.push(res % CHUNK);
        carry = res / CHUNK;
    }

    if carry > 0 {
        result.push(carry);
    }

    result
}

// Subtraction for positive numbers,
// a - b, assuming a >= b
//
// Based on Algorithm S, Section 4.3.1, TAoCP Vol. 2
// TODO: The handling of negative results is strange,
// is there a better way to do this?
fn vector_sub(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let mut result = Vec::new();
    let mut carry = 0_isize;

    for j in 0..n {
        let u = *a.get(j).unwrap();
        let v = *b.get(j).unwrap_or(&0);

        let res : isize = (u as isize) - (v as isize) - carry;

        if res < 0 {
            carry = 1;
        } else {
            carry = 0
        }

        let res_u = if res < 0 {
           (res + (CHUNK as isize)) as usize
        } else {
            res as usize
        };

        result.push(res_u % CHUNK)
    }

    // If this is the case, the assumption a >= b was wrong
    if carry == 1 {
        panic!("Invalid bignum subtraction, carry was -1 at the end");
    }

    // Remove leading 0s
    for j in 0..n {
        if result[n - j - 1] == 0 {
            result.pop();
        } else {
            break
        }
    }

    result
}

// Naive multiplication
// TODO: Implement Karatsuba (if possible) or FFT multiplication
fn vector_mul(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();

    for (i, item) in a.iter().enumerate() {
        let mut chunk_result = Vec::new();
        let mut chunk_carry = 0;
        for chunk in b {
            let res = (chunk * item) + chunk_carry;
            chunk_result.push(res % CHUNK);
            chunk_carry = res / CHUNK;
        }

        // TODO: Is this loop necessary?
        while chunk_carry > 0 {
            chunk_result.push(chunk_carry % CHUNK);
            chunk_carry /= CHUNK;
        }

        for _ in 0..i {
            chunk_result.insert(0, 0);
        }
        result = vector_add(&result, &chunk_result);
    }

    result
}

impl Add for Bignum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.sign || other.sign {
            panic!("Adding of Bignums < 0 is not implemented yet");
        }
        let new_data = vector_add(&self.data, &other.data);

        Self {
            sign: false,
            data: new_data,
        }
    }
}

impl Sub for Bignum {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.sign || other.sign {
            panic!("Subtraction of Bignums < 0 is not implemented yet");
        }
        let new_data = vector_sub(&self.data, &other.data);

        Self {
            sign: false,
            data: new_data,
        }
    }
}

impl Mul for Bignum {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let new_data = vector_mul(&self.data, &other.data);

        Self {
            sign: (self.sign ^ other.sign),
            data: new_data,
        }
    }
}


impl fmt::Display for Bignum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sign {
            write!(f, "-")?;
        }

        if self.data.len() == 0 {
            write!(f, "0")?;
        } else {
            let mut iter = self.data.iter().rev();
            write!(f, "{}", iter.nth(0).unwrap())?;
            for datum in iter {
                write!(f, "{:09}", datum)?;
            }
        }
        write!(f, "")
    }
}
