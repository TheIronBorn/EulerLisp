use std::ops::Add;
use std::fmt;

// TODO: Implement real comparison
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
pub struct Bignum {
    pub sign: bool,
    pub data: Vec<usize>
}

const CHUNK: usize = 10000000000;

impl Bignum {
    pub fn new(value: isize) -> Self {
        // TODO: This is broken for value > CHUNK
        if value > 0 {
            Bignum {
                sign: false,
                data: vec![value as usize]
            }
        } else if value == 0 {
            Bignum {
                sign: false,
                data: Vec::new()
            }
        } else {
            Bignum {
                sign: true,
                data: vec![(-value) as usize]
            }
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

    pub fn digits(&self) -> isize {
        let len = self.data.len();

        if len == 0 {
            1
        } else {
            let last = *self.data.get(len - 1).unwrap() as f64;
            (((len - 1) * 10) as isize) + (last.log(10.0) as isize) + 1
        }
    }
}

impl Add for Bignum {
    type Output = Bignum;

    fn add(self, other: Bignum) -> Bignum {
        if self.sign || other.sign {
            panic!("Adding of Bignums < 0 is not implemented yet");
        }

        let max_len = self.data.len().max(other.data.len());
        let mut new_data = Vec::new();

        let mut carry: usize = 0;
        for i in 0..max_len {
            let a = *self.data.get(i).unwrap_or(&0);
            let b = *other.data.get(i).unwrap_or(&0);

            let res = a + b + carry;
            new_data.push(res % CHUNK);
            carry = res / CHUNK;
        }
        if carry > 0 {
            new_data.push(carry);
        }

        Bignum {
            sign: false,
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
                write!(f, "{:010}", datum)?;
            }
        }
        write!(f, "")
    }
}
