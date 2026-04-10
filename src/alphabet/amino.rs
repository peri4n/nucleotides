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

#[derive(Debug, PartialEq, Eq)]
pub struct AA20;

impl Alphabet for AA20 {
    type Elements = AminoAcid;

    const SIZE: u8 = 20;
    const BITS: u8 = 5;

    const ELEMENTS: &'static [AminoAcid] = &[
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

    fn from_byte(b: u8) -> AminoAcid {
        match b {
            b'A' | b'a' => AminoAcid::A,
            b'C' | b'c' => AminoAcid::C,
            b'D' | b'd' => AminoAcid::D,
            b'E' | b'e' => AminoAcid::E,
            b'F' | b'f' => AminoAcid::F,
            b'G' | b'g' => AminoAcid::G,
            b'H' | b'h' => AminoAcid::H,
            b'I' | b'i' => AminoAcid::I,
            b'K' | b'k' => AminoAcid::K,
            b'L' | b'l' => AminoAcid::L,
            b'M' | b'm' => AminoAcid::M,
            b'N' | b'n' => AminoAcid::N,
            b'P' | b'p' => AminoAcid::P,
            b'Q' | b'q' => AminoAcid::Q,
            b'R' | b'r' => AminoAcid::R,
            b'S' | b's' => AminoAcid::S,
            b'T' | b't' => AminoAcid::T,
            b'V' | b'v' => AminoAcid::V,
            b'W' | b'w' => AminoAcid::W,
            b'Y' | b'y' => AminoAcid::Y,
            _ => panic!("invalid amino acid byte"),
        }
    }

    fn to_byte(e: AminoAcid) -> u8 {
        match e {
            AminoAcid::A => b'A',
            AminoAcid::C => b'C',
            AminoAcid::D => b'D',
            AminoAcid::E => b'E',
            AminoAcid::F => b'F',
            AminoAcid::G => b'G',
            AminoAcid::H => b'H',
            AminoAcid::I => b'I',
            AminoAcid::K => b'K',
            AminoAcid::L => b'L',
            AminoAcid::M => b'M',
            AminoAcid::N => b'N',
            AminoAcid::P => b'P',
            AminoAcid::Q => b'Q',
            AminoAcid::R => b'R',
            AminoAcid::S => b'S',
            AminoAcid::T => b'T',
            AminoAcid::V => b'V',
            AminoAcid::W => b'W',
            AminoAcid::Y => b'Y',
        }
    }
}
