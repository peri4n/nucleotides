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

// -- LUT generation ----------------------------------------------------------

const NUC4_BYTE_TO_BITS: [u8; 256] = {
    let mut lut = [0xFFu8; 256];
    lut[b'A' as usize] = 0;
    lut[b'a' as usize] = 0;
    lut[b'C' as usize] = 1;
    lut[b'c' as usize] = 1;
    lut[b'G' as usize] = 2;
    lut[b'g' as usize] = 2;
    lut[b'T' as usize] = 3;
    lut[b't' as usize] = 3;
    lut
};

const NUC5_BYTE_TO_BITS: [u8; 256] = {
    let mut lut = [0xFFu8; 256];
    lut[b'A' as usize] = 0;
    lut[b'a' as usize] = 0;
    lut[b'C' as usize] = 1;
    lut[b'c' as usize] = 1;
    lut[b'G' as usize] = 2;
    lut[b'g' as usize] = 2;
    lut[b'T' as usize] = 3;
    lut[b't' as usize] = 3;
    lut[b'N' as usize] = 4;
    lut[b'n' as usize] = 4;
    lut
};

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
    const BYTE_TO_BITS: [u8; 256] = NUC4_BYTE_TO_BITS;

    fn from_byte(b: u8) -> Nucleotide {
        let bits = Self::BYTE_TO_BITS[b as usize];
        debug_assert!(bits != 0xFF, "invalid Nuc4 byte");
        // SAFETY: ELEMENTS has 4 entries, valid bits are 0..3
        unsafe { *Self::ELEMENTS.get_unchecked(bits as usize) }
    }

    fn to_byte(e: Nucleotide) -> u8 {
        const LUT: [u8; 5] = [b'A', b'C', b'G', b'T', 0];
        debug_assert!((e as u8) < 4, "N is not a valid Nuc4 symbol");
        LUT[e as usize]
    }
}

impl Alphabet for Nuc5 {
    type Elements = Nucleotide;
    const SIZE: u8 = 5;
    const BITS: u8 = 3;
    const ELEMENTS: &'static [Nucleotide] = NUC5_ELEMENTS;
    const BYTE_TO_BITS: [u8; 256] = NUC5_BYTE_TO_BITS;

    fn from_byte(b: u8) -> Nucleotide {
        let bits = Self::BYTE_TO_BITS[b as usize];
        debug_assert!(bits != 0xFF, "invalid Nuc5 byte");
        // SAFETY: ELEMENTS has 5 entries, valid bits are 0..4
        unsafe { *Self::ELEMENTS.get_unchecked(bits as usize) }
    }

    fn to_byte(e: Nucleotide) -> u8 {
        const LUT: [u8; 5] = [b'A', b'C', b'G', b'T', b'N'];
        LUT[e as usize]
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
