use std::fmt::{Display, Formatter};

fn main() {
    let a = U4::from_u8(3);
    let b = U4::from_u8(3);

    println!("{:?}", a);
    println!("{:?}", b);

    println!("{}", a + b);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct U4 {
    bits: [bool; 4],
}

impl Display for U4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u8())
    }
}

impl std::ops::Add for U4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut bits = [false; 4];
        let mut carry = false;
        for i in (0..4).rev() {
            let a = self.bits[i];
            let b = rhs.bits[i];

            if a && b {
                if carry {
                    carry = true;
                    bits[i] = true;
                } else {
                    carry = true;
                    bits[i] = false;
                }
            } else if !a && !b {
                if carry {
                    carry = false;
                    bits[i] = true;
                } else {
                    carry = false;
                    bits[i] = false;
                }
            } else {
                if carry {
                    carry = true;
                    bits[i] = false;
                } else {
                    carry = false;
                    bits[i] = true;
                }
            }
        }

        Self { bits }
    }
}

impl U4 {
    fn to_u8(&self) -> u8 {
        let mut sum = 0;
        for i in 0..4 {
            let bit = self.bits[4 - i - 1];

            if bit {
                sum += 2_u8.pow(i as u32);
            }
        }
        sum
    }
    fn from_u8(u: u8) -> Self {
        let mut bits = [false; 4];
        for i in 0..4 {
            bits[i] = (u & (1 << i)) != 0;
        }
        bits.reverse();
        U4 { bits }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_to_u8() {
        let u = U4::from_u8(2);
        assert_eq!(u.to_u8(), 2);
    }

    #[test]
    fn add() {
        let a = U4::from_u8(3);
        let b = U4::from_u8(3);

        assert_eq!(a + b, U4::from_u8(6));
    }
}
