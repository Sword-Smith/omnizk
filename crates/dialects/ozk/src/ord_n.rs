use std::fmt::Display;
use Ord16::*;

/// `Ord16` represents numbers that are exactly 0--15.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Ord16 {
    #[default]
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,
    ST8,
    ST9,
    ST10,
    ST11,
    ST12,
    ST13,
    ST14,
    ST15,
}

impl Display for Ord16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n: usize = (*self).into();
        write!(f, "{n}")
    }
}

impl From<Ord16> for u32 {
    fn from(n: Ord16) -> Self {
        match n {
            ST0 => 0,
            ST1 => 1,
            ST2 => 2,
            ST3 => 3,
            ST4 => 4,
            ST5 => 5,
            ST6 => 6,
            ST7 => 7,
            ST8 => 8,
            ST9 => 9,
            ST10 => 10,
            ST11 => 11,
            ST12 => 12,
            ST13 => 13,
            ST14 => 14,
            ST15 => 15,
        }
    }
}

impl From<Ord16> for u64 {
    fn from(n: Ord16) -> Self {
        let v: u32 = n.into();
        v.into()
    }
}

impl TryFrom<u32> for Ord16 {
    type Error = String;

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(ST0),
            1 => Ok(ST1),
            2 => Ok(ST2),
            3 => Ok(ST3),
            4 => Ok(ST4),
            5 => Ok(ST5),
            6 => Ok(ST6),
            7 => Ok(ST7),
            8 => Ok(ST8),
            9 => Ok(ST9),
            10 => Ok(ST10),
            11 => Ok(ST11),
            12 => Ok(ST12),
            13 => Ok(ST13),
            14 => Ok(ST14),
            15 => Ok(ST15),
            _ => Err(format!("{n} is out of range for Ord16")),
        }
    }
}

impl TryFrom<u64> for Ord16 {
    type Error = String;

    fn try_from(n: u64) -> Result<Self, Self::Error> {
        let n: u32 = n.try_into().map_err(|_| format!("{n} is out of range for Ord16"))?;
        n.try_into()
    }
}

impl From<&Ord16> for u32 {
    fn from(n: &Ord16) -> Self {
        (*n).into()
    }
}

impl From<Ord16> for usize {
    fn from(n: Ord16) -> Self {
        let n: u32 = n.into();
        n as usize
    }
}

impl From<&Ord16> for usize {
    fn from(n: &Ord16) -> Self {
        (*n).into()
    }
}

impl TryFrom<usize> for Ord16 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ST0),
            1 => Ok(ST1),
            2 => Ok(ST2),
            3 => Ok(ST3),
            4 => Ok(ST4),
            5 => Ok(ST5),
            6 => Ok(ST6),
            7 => Ok(ST7),
            8 => Ok(ST8),
            9 => Ok(ST9),
            10 => Ok(ST10),
            11 => Ok(ST11),
            12 => Ok(ST12),
            13 => Ok(ST13),
            14 => Ok(ST14),
            15 => Ok(ST15),
            _ => Err("usize out of range for Ord16"),
        }
    }
}
