use super::Alphabet;

/// Typed nucleotide symbols (shared by Nuc4 and Nuc5)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Nucleotide {
    A = 0,
    C = 1,
    G = 2,
    T = 3,
    N = 4,
}

impl From<Nucleotide> for u8 {
    fn from(n: Nucleotide) -> u8 {
        n as u8
    }
}

// -- Alphabet structs --------------------------------------------------------

/// 4-symbol DNA alphabet (A, C, G, T)
#[derive(Debug, PartialEq, Eq)]
pub struct Nuc4;

/// 5-symbol DNA alphabet (A, C, G, T, N)
#[derive(Debug, PartialEq, Eq)]
pub struct Nuc5;

// -- Alphabet impls ----------------------------------------------------------

const NUC4_ELEMENTS: &[Nucleotide] = &[Nucleotide::A, Nucleotide::C, Nucleotide::G, Nucleotide::T];

const NUC5_ELEMENTS: &[Nucleotide] = &[
    Nucleotide::A,
    Nucleotide::C,
    Nucleotide::G,
    Nucleotide::T,
    Nucleotide::N,
];

impl Alphabet for Nuc4 {
    type Elements = Nucleotide;
    const SIZE: u8 = 4;
    const BITS: u8 = 2;
    const ELEMENTS: &'static [Nucleotide] = NUC4_ELEMENTS;

    fn from_byte(b: u8) -> Nucleotide {
        match b {
            b'A' | b'a' => Nucleotide::A,
            b'C' | b'c' => Nucleotide::C,
            b'G' | b'g' => Nucleotide::G,
            b'T' | b't' => Nucleotide::T,
            _ => panic!("invalid Nuc4 byte"),
        }
    }

    fn to_byte(e: Nucleotide) -> u8 {
        match e {
            Nucleotide::A => b'A',
            Nucleotide::C => b'C',
            Nucleotide::G => b'G',
            Nucleotide::T => b'T',
            Nucleotide::N => panic!("N is not a valid Nuc4 symbol"),
        }
    }
}

impl Alphabet for Nuc5 {
    type Elements = Nucleotide;
    const SIZE: u8 = 5;
    const BITS: u8 = 3;
    const ELEMENTS: &'static [Nucleotide] = NUC5_ELEMENTS;

    fn from_byte(b: u8) -> Nucleotide {
        match b {
            b'A' | b'a' => Nucleotide::A,
            b'C' | b'c' => Nucleotide::C,
            b'G' | b'g' => Nucleotide::G,
            b'T' | b't' => Nucleotide::T,
            b'N' | b'n' => Nucleotide::N,
            _ => panic!("invalid Nuc5 byte"),
        }
    }

    fn to_byte(e: Nucleotide) -> u8 {
        match e {
            Nucleotide::A => b'A',
            Nucleotide::C => b'C',
            Nucleotide::G => b'G',
            Nucleotide::T => b'T',
            Nucleotide::N => b'N',
        }
    }
}

// -- Promote impls -----------------------------------------------------------

use super::Promote;

impl Promote<Nuc4> for Nuc4 {
    type Output = Nuc4;
}
impl Promote<Nuc5> for Nuc4 {
    type Output = Nuc5;
}
impl Promote<Nuc4> for Nuc5 {
    type Output = Nuc5;
}
impl Promote<Nuc5> for Nuc5 {
    type Output = Nuc5;
}
