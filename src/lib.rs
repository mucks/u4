use std::{
    fmt::{Display, Formatter},
    num::ParseIntError,
};

use hex::{decode_hex, encode_hex};

mod hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct U4 {
    bits: [bool; 4],
}

impl Display for U4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u8())
    }
}

impl std::ops::BitXor for U4 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut bits = [false; 4];
        for i in 0..4 {
            bits[i] = self.bits[i] ^ rhs.bits[i];
        }
        Self { bits }
    }
}

impl std::ops::BitOr for U4 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut bits = [false; 4];
        for i in 0..4 {
            bits[i] = self.bits[i] || rhs.bits[i];
        }
        Self { bits }
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

impl std::ops::Sub for U4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut bits = [false; 4];
        let mut carry = false;
        for i in (0..4).rev() {
            let a = self.bits[i];
            let b = rhs.bits[i];

            // 1 - 1
            if a && b {
                if carry {
                    carry = false;
                    bits[i] = true;
                } else {
                    carry = false;
                    bits[i] = false;
                }
            // 0 - 0
            } else if !a && !b {
                if carry {
                    carry = true;
                    bits[i] = true;
                } else {
                    carry = false;
                    bits[i] = false;
                }
            // 0 - 1
            } else if !a && b {
                if carry {
                    carry = true;
                    bits[i] = false;
                } else {
                    carry = true;
                    bits[i] = true;
                }
            // 1 - 0
            } else {
                if carry {
                    carry = false;
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
    pub const MIN: U4 = U4 { bits: [false; 4] };
    pub const MAX: U4 = U4 { bits: [true; 4] };
    pub const BITS: usize = 4;

    fn n(n: u8) -> Self {
        Self::from_u8(n)
    }

    fn to_u8(&self) -> u8 {
        let mut sum = 0;
        for i in 0..Self::BITS {
            let bit = self.bits[Self::BITS - i - 1];

            if bit {
                sum += 2_u8.pow(i as u32);
            }
        }
        sum
    }

    fn from_bytes(a: &[u8]) -> Self {
        let mut bits = [false; Self::BITS];
        for i in 0..U4::BITS {
            bits[i] = (a[0] & (1 << i)) != 0;
        }
        bits.reverse();
        U4 { bits }
    }

    // From Hex String
    fn from_hex_str(s: &str) -> Result<Self, ParseIntError> {
        let s = decode_hex(s)?;
        Ok(Self::from_bytes(&s))
    }

    fn to_hex_str(&self) -> String {
        encode_hex(&self.to_u8().to_le_bytes())
    }

    fn from_u8(u: u8) -> Self {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = (u & (1 << i)) != 0;
        }
        bits.reverse();
        U4 { bits }
    }

    pub fn rotate_left(self, n: u32) -> Self {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = self.bits[(i + n as usize) % Self::BITS];
        }
        Self { bits }
    }

    pub fn rotate_right(self, n: u32) -> Self {
        let mut bits = [false; Self::BITS];
        for i in 0..Self::BITS {
            bits[i] = self.bits[(i + Self::BITS - n as usize) % Self::BITS];
        }
        Self { bits }
    }

    pub fn wrapping_add(self, rhs: Self) -> Self {
        self + rhs
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self - rhs
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
    fn rotate_right() {
        let u = U4::from_u8(2);
        let u = u.rotate_right(1);
        assert_eq!(u.to_u8(), 1);
    }

    #[test]
    fn rotate_left() {
        let u = U4::from_u8(2);
        let u = u.rotate_left(1);
        assert_eq!(u.to_u8(), 4);
    }

    #[test]
    fn rotate_left_wrap() {
        let u = U4::from_u8(2);
        let u = u.rotate_left(3);
        assert_eq!(u.to_u8(), 1);
    }

    #[test]
    fn from_bytes() {
        let a = 11_u8.to_le_bytes();
        let b = 1_u8.to_le_bytes();

        let u = U4::from_bytes(&a);
        assert_eq!(u, U4::n(11));
        let u = U4::from_bytes(&b);
        assert_eq!(u, U4::n(1));
    }

    #[test]
    fn from_hex() {
        let u = U4::from_hex_str("0b").unwrap();
        assert_eq!(u, U4::n(11));
    }

    #[test]
    fn add() {
        let a = U4::from_u8(3);
        let b = U4::from_u8(3);

        assert_eq!(a + b, U4::from_u8(6));
    }

    #[test]
    fn sub() {
        let a = U4::from_u8(3);
        let b = U4::from_u8(2);

        assert_eq!(a - b, U4::from_u8(1));
    }

    #[test]
    fn wrapping_add() {
        let a = U4::from_u8(12);
        let b = U4::from_u8(12);

        assert_eq!(a + b, U4::from_u8(8));
    }

    #[test]
    fn wrapping_sub() {
        let a = U4::from_u8(1);
        let b = U4::from_u8(2);

        assert_eq!(a - b, U4::from_u8(15));
    }

    #[test]
    fn or() {
        let a = U4::from_u8(3);
        let b = U4::from_u8(4);

        assert_eq!(a | b, U4::from_u8(7));
    }
    #[test]
    fn xor() {
        let a = U4::from_u8(3);
        let b = U4::from_u8(5);

        assert_eq!(a ^ b, U4::from_u8(6));
    }
}
