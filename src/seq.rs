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
        // SAFETY: block is always in bounds when index < self.length
        // and self.data was allocated with bytes_to_store(self.length)
        unsafe { *self.data.get_unchecked_mut(block) |= bits << bit };
    }

    /// Returns the raw bit value at the given index.
    #[inline(always)]
    pub fn get_bits(&self, index: usize) -> u8 {
        let (block, bit) = self.address(index);
        // SAFETY: same as init_with — block in bounds when index < self.length
        unsafe { (self.data.get_unchecked(block) >> bit) & Self::MASK }
    }

    /// Returns the decoded element at the given index.
    #[inline(always)]
    pub fn get(&self, index: usize) -> A::Elements {
        let bits = self.get_bits(index) as usize;
        // SAFETY: bits is masked to BITS width, always < SIZE <= ELEMENTS.len()
        unsafe { *A::ELEMENTS.get_unchecked(bits) }
    }

    /// Concatenates two sequences, returning a new sequence in the promoted alphabet.
    pub fn concat<B>(&self, other: &Seq<B>) -> Seq<<A as Promote<B>>::Output>
    where
        B: Alphabet<Elements = A::Elements>,
        A: Promote<B>,
    {
        type_assert_eq::<A::Elements, B::Elements>();

        let total = self.length + other.length;
        let mut result = Seq::<<A as Promote<B>>::Output>::new(total);
        let out_bits = <A as Promote<B>>::Output::BITS;
        let out_spb = 8 / out_bits as usize;

        // Fast path: memcpy self.data when bit widths match
        if A::BITS == out_bits {
            result.data[..self.data.len()].copy_from_slice(&self.data);
        } else {
            for i in 0..self.length {
                result.init_with(i, self.get(i).into());
            }
        }

        // Fast path: memcpy other.data when bit widths match AND byte-aligned
        // NB: alignment + dst offset must use the *output* alphabet's packing
        if B::BITS == out_bits && self.length % out_spb == 0 {
            let dst_start = self.length / out_spb;
            result.data[dst_start..dst_start + other.data.len()].copy_from_slice(&other.data);
        } else {
            for i in 0..other.length {
                result.init_with(self.length + i, other.get(i).into());
            }
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

    /// Returns an iterator over the elements.
    pub fn iter(&self) -> SeqIter<'_, A> {
        SeqIter { seq: self, pos: 0 }
    }
}

/// Compile-time assert that two types are equal. Optimized away entirely.
fn type_assert_eq<T, U>()
where
    T: Into<u8>,
    U: Into<u8>,
{
}

// -- Batch-packing TryFrom ---------------------------------------------------

impl<A: Alphabet> TryFrom<&str> for Seq<A> {
    type Error = SeqError;

    fn try_from(ascii: &str) -> Result<Self, Self::Error> {
        let input = ascii.as_bytes();
        let len = input.len();
        let spb = Self::SYMBOLS_PER_BYTE;
        let bits = A::BITS as u32;
        let full_bytes = len / spb;
        let remainder = len % spb;
        let mut data = vec![0u8; Self::bytes_to_store(len)];
        let lut = &A::BYTE_TO_BITS;

        // Pack full bytes (SYMBOLS_PER_BYTE symbols → 1 byte)
        for byte_idx in 0..full_bytes {
            let base = byte_idx * spb;
            let mut packed = 0u8;
            for j in 0..spb {
                // SAFETY: base + j < full_bytes * spb <= len
                let b = unsafe { *input.get_unchecked(base + j) };
                let v = unsafe { *lut.get_unchecked(b as usize) };
                if v == 0xFF {
                    return Err(SeqError::InvalidSymbol);
                }
                packed |= v << ((spb - 1 - j) as u32 * bits);
            }
            // SAFETY: byte_idx < full_bytes = bytes_to_store(len) when remainder == 0,
            // or < bytes_to_store(len) - 1 when remainder > 0
            unsafe { *data.get_unchecked_mut(byte_idx) = packed };
        }

        // Pack remaining symbols into the last byte
        if remainder > 0 {
            let base = full_bytes * spb;
            let mut packed = 0u8;
            for j in 0..remainder {
                let b = unsafe { *input.get_unchecked(base + j) };
                let v = unsafe { *lut.get_unchecked(b as usize) };
                if v == 0xFF {
                    return Err(SeqError::InvalidSymbol);
                }
                packed |= v << ((spb - 1 - j) as u32 * bits);
            }
            unsafe { *data.get_unchecked_mut(full_bytes) = packed };
        }

        Ok(Self {
            length: len,
            data,
            _marker: std::marker::PhantomData,
        })
    }
}

// -- Iterator ----------------------------------------------------------------

pub struct SeqIter<'a, A: Alphabet> {
    seq: &'a Seq<A>,
    pos: usize,
}

impl<'a, A: Alphabet> Iterator for SeqIter<'a, A> {
    type Item = A::Elements;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.seq.length {
            let elem = self.seq.get(self.pos);
            self.pos += 1;
            Some(elem)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.seq.length - self.pos;
        (remaining, Some(remaining))
    }
}

impl<'a, A: Alphabet> ExactSizeIterator for SeqIter<'a, A> {}

impl<'a, A: Alphabet> IntoIterator for &'a Seq<A> {
    type Item = A::Elements;
    type IntoIter = SeqIter<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// -- Display -----------------------------------------------------------------

impl<A: Alphabet> fmt::Display for Seq<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Batch: decode a full byte's worth of symbols at a time
        let spb = Self::SYMBOLS_PER_BYTE;
        let bits = A::BITS;
        let full_bytes = self.length / spb;
        let remainder = self.length % spb;

        let mut buf = [0u8; 8]; // max 8 symbols per byte (1-bit encoding)

        for byte_idx in 0..full_bytes {
            let packed = unsafe { *self.data.get_unchecked(byte_idx) };
            for j in 0..spb {
                let shift = (spb - 1 - j) * bits as usize;
                let elem_bits = (packed >> shift) & Self::MASK;
                let elem = unsafe { *A::ELEMENTS.get_unchecked(elem_bits as usize) };
                buf[j] = A::to_byte(elem);
            }
            // SAFETY: buf[..spb] contains valid ASCII bytes
            let s = unsafe { std::str::from_utf8_unchecked(&buf[..spb]) };
            f.write_str(s)?;
        }

        // Remainder
        if remainder > 0 {
            let packed = unsafe { *self.data.get_unchecked(full_bytes) };
            for j in 0..remainder {
                let shift = (spb - 1 - j) * bits as usize;
                let elem_bits = (packed >> shift) & Self::MASK;
                let elem = unsafe { *A::ELEMENTS.get_unchecked(elem_bits as usize) };
                buf[j] = A::to_byte(elem);
            }
            let s = unsafe { std::str::from_utf8_unchecked(&buf[..remainder]) };
            f.write_str(s)?;
        }

        Ok(())
    }
}

// -- Trait impls -------------------------------------------------------------

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

// -- Validation + macros -----------------------------------------------------

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
        // SAFETY: validation above guarantees all bytes are valid Nuc4 symbols
        $crate::seq::Seq::<$crate::alphabet::Nuc4>::try_from($s).unwrap()
    }};
}

#[macro_export]
macro_rules! dna5 {
    ($s:literal) => {{
        const VALID: bool = $crate::seq::is_valid_dna5($s);
        if !VALID {
            panic!("Invalid DNA sequence literal");
        }
        // SAFETY: validation above guarantees all bytes are valid Nuc5 symbols
        $crate::seq::Seq::<$crate::alphabet::Nuc5>::try_from($s).unwrap()
    }};
}
