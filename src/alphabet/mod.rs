mod amino;
mod nucleotides;

pub use amino::*;
pub use nucleotides::*;

/// Marker trait for alphabets
pub trait Alphabet: 'static + Eq {
    type Elements: Copy + Into<u8> + PartialEq;

    /// Number of symbols in the alphabet
    const SIZE: u8;

    /// Bits per symbol
    const BITS: u8;

    const ELEMENTS: &'static [Self::Elements];

    /// Lookup table: ASCII byte → bit value. 0xFF = invalid.
    const BYTE_TO_BITS: [u8; 256];

    /// Convert an ASCII byte to a typed element
    fn from_byte(b: u8) -> Self::Elements;

    /// Convert a typed element to an ASCII byte
    fn to_byte(e: Self::Elements) -> u8;
}

/// Promotion trait for combining two alphabets.
///
/// Output is the smallest alphabet that can represent all symbols from both.
/// Rule: max width, case-insensitive if either input is case-insensitive.
pub trait Promote<Rhs: Alphabet>: Alphabet {
    type Output: Alphabet<Elements = Self::Elements>;
}
