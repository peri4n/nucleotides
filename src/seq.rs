use std::fmt;

use crate::alphabet::{Alphabet, Promote};

#[derive(Debug, PartialEq)]
pub enum SeqError {
    InvalidSymbol,
}

/// Represents a biological sequence.
///
/// Symbols are bit-packed into bytes, using `A::BITS` bits per symbol.
/// Symbols are stored big-endian within each byte (first symbol in the
/// highest bits).
#[derive(Debug, Eq)]
pub struct Seq<A>
where
    A: Alphabet,
{
    pub(crate) length: usize,
    pub(crate) data: Vec<u8>,
    _marker: std::marker::PhantomData<A>,
}

impl<A: Alphabet> Seq<A> {
    /// Symbols that fit in a single byte.
    const SYMBOLS_PER_BYTE: usize = 8 / A::BITS as usize;

    /// Bitmask for a single symbol.
    const MASK: u8 = (1 << A::BITS) - 1;

    /// Creates a new sequence with the given length.
    ///
    /// Every symbol is initialized to the first element of the alphabet.
    pub fn new(length: usize) -> Self {
        Self {
            length,
            data: vec![0; Self::bytes_to_store(length)],
            _marker: std::marker::PhantomData,
        }
    }

    /// Access the length of the sequence.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Creates a sequence from raw packed bytes.
    ///
    /// The caller is responsible for ensuring the bytes are correctly packed.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            length: bytes.len() * Self::SYMBOLS_PER_BYTE,
            data: bytes.to_vec(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Return the internal byte representation of the sequence.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Draws a random sequence with the given length.
    pub fn random(length: usize) -> Self {
        let mut seq = Self::new(length);
        for i in 0..length {
            seq.init_with(i, rand::random::<u8>() & Self::MASK);
        }
        seq
    }

    /// Computes the byte and bit offset for the symbol at `index`.
    #[inline(always)]
    fn address(&self, index: usize) -> (usize, u8) {
        let block = index / Self::SYMBOLS_PER_BYTE;
        let bit = ((Self::SYMBOLS_PER_BYTE - 1 - (index % Self::SYMBOLS_PER_BYTE))
            * A::BITS as usize) as u8;
        (block, bit)
    }

    /// Returns the internal bit sequence of the sequence.
    pub fn bit_string(&self) -> String {
        let mut bit_string = String::with_capacity(self.len() * 2);
        for i in 0..self.data.len() {
            bit_string.push_str(&format!("{:08b} ", self.data[i]));
        }
        bit_string
    }

    /// Initially sets the symbol at the given index (0-based).
    ///
    /// Note: If the index already contains set bits, bit patterns may cause bugs.
    #[inline(always)]
    pub(crate) fn init_with(&mut self, index: usize, bits: u8) {
        let (block, bit) = self.address(index);
        self.data[block] |= bits << bit;
    }

    /// Returns the raw bit value at the given index.
    #[inline(always)]
    pub fn get_bits(&self, index: usize) -> u8 {
        let (block, bit) = self.address(index);
        (self.data[block] >> bit) & Self::MASK
    }

    /// Returns the decoded element at the given index.
    pub fn get(&self, index: usize) -> A::Elements {
        A::ELEMENTS[self.get_bits(index) as usize]
    }

    /// Concatenates two sequences, returning a new sequence in the promoted alphabet.
    pub fn concat<B>(&self, other: &Seq<B>) -> Seq<<A as Promote<B>>::Output>
    where
        B: Alphabet<Elements = A::Elements>,
        A: Promote<B>,
    {
        let total = self.length + other.length;
        let mut result = Seq::<<A as Promote<B>>::Output>::new(total);

        for i in 0..self.length {
            result.init_with(i, self.get(i).into());
        }
        for i in 0..other.length {
            result.init_with(self.length + i, other.get(i).into());
        }

        result
    }

    /// Consumes self and appends another sequence, returning a new sequence in the promoted alphabet.
    pub fn append<B>(self, other: &Seq<B>) -> Seq<<A as Promote<B>>::Output>
    where
        B: Alphabet<Elements = A::Elements>,
        A: Promote<B>,
    {
        self.concat(other)
    }

    /// Trims the sequence to the given size.
    pub fn trim(&mut self, size: usize) {
        self.length = size;
        self.data.truncate(Self::bytes_to_store(size));
    }

    /// Computes the number of bytes needed to store `length` symbols.
    pub fn bytes_to_store(length: usize) -> usize {
        length.div_ceil(Self::SYMBOLS_PER_BYTE)
    }
}

impl<A: Alphabet> fmt::Display for Seq<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.length {
            let b = A::to_byte(self.get(i));
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

impl<A: Alphabet> TryFrom<&str> for Seq<A> {
    type Error = SeqError;

    fn try_from(ascii: &str) -> Result<Self, Self::Error> {
        let mut seq = Self::new(ascii.len());

        for (i, &b) in ascii.as_bytes().iter().enumerate() {
            let bits: u8 = A::from_byte(b).into();
            seq.init_with(i, bits);
        }

        Ok(seq)
    }
}

impl<A: Alphabet> PartialEq for Seq<A> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.length == other.length
    }
}

impl<A: Alphabet> PartialOrd for Seq<A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<A: Alphabet> Ord for Seq<A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

pub const fn is_valid_dna4(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'A' | b'C' | b'G' | b'T' => {}
            _ => return false,
        }
        i += 1;
    }

    true
}

pub const fn is_valid_dna5(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'A' | b'C' | b'G' | b'T' | b'N' => {}
            _ => return false,
        }
        i += 1;
    }

    true
}

#[macro_export]
macro_rules! dna4 {
    ($s:literal) => {{
        const VALID: bool = $crate::seq::is_valid_dna4($s);
        if !VALID {
            panic!("Invalid DNA sequence literal");
        }
        $crate::seq::Seq::<$crate::alphabet::Nuc4>::from_ascii($s)
    }};
}

#[macro_export]
macro_rules! dna5 {
    ($s:literal) => {{
        const VALID: bool = $crate::seq::is_valid_dna5($s);
        if !VALID {
            panic!("Invalid DNA sequence literal");
        }
        $crate::seq::Seq::<$crate::alphabet::Nuc5>::from_ascii($s)
    }};
}
