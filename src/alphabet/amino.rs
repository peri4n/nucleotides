use super::Alphabet;

/// Typed amino acid symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AminoAcid {
    A = 0,
    C = 1,
    D = 2,
    E = 3,
    F = 4,
    G = 5,
    H = 6,
    I = 7,
    K = 8,
    L = 9,
    M = 10,
    N = 11,
    P = 12,
    Q = 13,
    R = 14,
    S = 15,
    T = 16,
    V = 17,
    W = 18,
    Y = 19,
}

impl From<AminoAcid> for u8 {
    fn from(a: AminoAcid) -> u8 {
        a as u8
    }
}

const AA20_BYTE_TO_BITS: [u8; 256] = {
    let mut lut = [0xFFu8; 256];
    lut[b'A' as usize] = 0;
    lut[b'a' as usize] = 0;
    lut[b'C' as usize] = 1;
    lut[b'c' as usize] = 1;
    lut[b'D' as usize] = 2;
    lut[b'd' as usize] = 2;
    lut[b'E' as usize] = 3;
    lut[b'e' as usize] = 3;
    lut[b'F' as usize] = 4;
    lut[b'f' as usize] = 4;
    lut[b'G' as usize] = 5;
    lut[b'g' as usize] = 5;
    lut[b'H' as usize] = 6;
    lut[b'h' as usize] = 6;
    lut[b'I' as usize] = 7;
    lut[b'i' as usize] = 7;
    lut[b'K' as usize] = 8;
    lut[b'k' as usize] = 8;
    lut[b'L' as usize] = 9;
    lut[b'l' as usize] = 9;
    lut[b'M' as usize] = 10;
    lut[b'm' as usize] = 10;
    lut[b'N' as usize] = 11;
    lut[b'n' as usize] = 11;
    lut[b'P' as usize] = 12;
    lut[b'p' as usize] = 12;
    lut[b'Q' as usize] = 13;
    lut[b'q' as usize] = 13;
    lut[b'R' as usize] = 14;
    lut[b'r' as usize] = 14;
    lut[b'S' as usize] = 15;
    lut[b's' as usize] = 15;
    lut[b'T' as usize] = 16;
    lut[b't' as usize] = 16;
    lut[b'V' as usize] = 17;
    lut[b'v' as usize] = 17;
    lut[b'W' as usize] = 18;
    lut[b'w' as usize] = 18;
    lut[b'Y' as usize] = 19;
    lut[b'y' as usize] = 19;
    lut
};

const AA20_ELEMENTS: &[AminoAcid] = &[
    AminoAcid::A,
    AminoAcid::C,
    AminoAcid::D,
    AminoAcid::E,
    AminoAcid::F,
    AminoAcid::G,
    AminoAcid::H,
    AminoAcid::I,
    AminoAcid::K,
    AminoAcid::L,
    AminoAcid::M,
    AminoAcid::N,
    AminoAcid::P,
    AminoAcid::Q,
    AminoAcid::R,
    AminoAcid::S,
    AminoAcid::T,
    AminoAcid::V,
    AminoAcid::W,
    AminoAcid::Y,
];

const AA20_TO_BYTE: [u8; 20] = [
    b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S',
    b'T', b'V', b'W', b'Y',
];

#[derive(Debug, PartialEq, Eq)]
pub struct AA20;

impl Alphabet for AA20 {
    type Elements = AminoAcid;

    const SIZE: u8 = 20;
    const BITS: u8 = 5;

    const ELEMENTS: &'static [AminoAcid] = AA20_ELEMENTS;
    const BYTE_TO_BITS: [u8; 256] = AA20_BYTE_TO_BITS;

    fn from_byte(b: u8) -> AminoAcid {
        let bits = Self::BYTE_TO_BITS[b as usize];
        debug_assert!(bits != 0xFF, "invalid amino acid byte");
        // SAFETY: ELEMENTS has 20 entries, valid bits are 0..19
        unsafe { *Self::ELEMENTS.get_unchecked(bits as usize) }
    }

    fn to_byte(e: AminoAcid) -> u8 {
        AA20_TO_BYTE[e as usize]
    }
}
