use std::ops::Add;
use std::ops::Mul;
use std::fmt;

// TODO: Implement real comparison
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Bignum {
    pub sign: bool,
    pub data: Vec<usize>
}

// 10^9, this way the product of the sum of two parts still fits inside a u64
const CHUNK: usize = 1000000000;

impl Bignum {
    pub fn new(value: isize) -> Self {
        if value == 0 {
            return Bignum { sign: false, data: Vec::new() }
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

        Bignum {
            sign: sign,
            data: data
        }
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
            for chunk in self.data.iter() {
                let mut cur = *chunk;
                while cur > 0 {
                    result.push((cur % 10) as isize);
                    cur /= 10;
                }
            }
            result.into_iter().rev().collect()
        }
    }
}

fn vector_add(a: &Vec<usize>, b: &Vec<usize>) -> Vec<usize> {
    let max_len = a.len().max(b.len());
    let mut result = Vec::new();
    let mut carry: usize = 0;
    for i in 0..max_len {
        let a = *a.get(i).unwrap_or(&0);
        let b = *b.get(i).unwrap_or(&0);

        let res = a + b + carry;
        result.push(res % CHUNK);
        carry = res / CHUNK;
    }
    if carry > 0 {
        result.push(carry);
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
            write!(f, "{}", iter.nth(0).unwrap());
            for datum in iter {
                write!(f, "{:09}", datum)?;
            }
        }
        write!(f, "")
    }
}
