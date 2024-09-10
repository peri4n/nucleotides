use std::fmt;
use std::mem::size_of;
use std::ops::Index;

const NR_OF_NUCLEOTIDES: usize = 4;

const BITS_PER_NUCLEOTIDE: usize = NR_OF_NUCLEOTIDES.ilog2() as usize;

const NUCS_PER_BLOCK: usize = size_of::<u8>() * NR_OF_NUCLEOTIDES;

const MASK: u8 = NR_OF_NUCLEOTIDES as u8 - 1;

pub enum Nucleotide {
    A = 0,
    C = 1,
    G = 2,
    T = 3,
}

const NUCS: [Nucleotide; NR_OF_NUCLEOTIDES] = [
    Nucleotide::A,
    Nucleotide::C,
    Nucleotide::G,
    Nucleotide::T,
];

/// Represents a _case-insensitive_ DNA sequence.
///
/// The DNA sequence is stored in a compact way, using 2 bits per nucleotide.
/// This allows for a low memory footprint and fast bitwise operations.
/// 
/// The bitwise encoding is as follows:
/// - `00` for `A`
/// - `01` for `C`
/// - `10` for `G`
/// - `11` for `T`
#[derive(Debug, Eq)]
pub struct Dna {
    pub(crate) length: usize,
    pub(crate) nucleotides: Vec<u8>,
}

impl Dna {
    /// Creates a new DNA sequence with the given length.
    ///
    /// Every nucleotide is initialized to `A`.
    pub fn new(length: usize) -> Dna {
        Dna {
            length,
            nucleotides: vec![0; length],
        }
    }

    /// Access the length of the DNA sequence.
    ///
    /// ```
    /// let dna = nuc::dna::Dna::from_ascii("ACGT");
    /// assert_eq!(dna.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if the DNA sequence is empty.
    ///
    /// ```
    /// let dna = nuc::dna::Dna::new(0);
    /// assert!(dna.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn from_bytes(bytes: &[u8]) -> Dna {
        Dna {
            length: bytes.len(),
            nucleotides: bytes.to_vec(),
        }
    }

    /// Creates a DNA sequence from an ASCII string.
    ///
    /// ```
    /// let dna = nuc::dna::Dna::from_ascii("ACGT");
    /// assert_eq!(dna.to_string(), "ACGT");
    /// ```
    pub fn from_ascii(ascii: &str) -> Dna {
        let mut dna = Dna {
            length: ascii.len(),
            nucleotides: vec![0; Dna::bytes_to_store(ascii.len())],
        };

        for (i, c) in ascii.char_indices() {
            match c {
                'C' | 'c' => dna.init_with(i, 1),
                'G' | 'g' => dna.init_with(i, 2),
                'T' | 't' => dna.init_with(i, 3),
                _ => dna.init_with(i, 0),
            }
        }

        dna
    }

    /// Draws a random DNA sequence with the given length.
    ///
    /// ```
    /// let dna = nuc::dna::Dna::random(8);
    /// assert_eq!(dna.len(), 8);
    /// ```
    pub fn random(length: usize) -> Dna {
        let mut dna = Dna::new(length);
        for i in 0..length {
            dna.init_with(i, rand::random::<u8>() & MASK);
        }
        dna
    }

    #[inline(always)]
    fn address(&self, index: usize) -> (usize, u8) {
        let block = index / NUCS_PER_BLOCK;
        let bit = ((NUCS_PER_BLOCK - 1 - (index % NUCS_PER_BLOCK)) * BITS_PER_NUCLEOTIDE) as u8;
        (block, bit)
    }

    /// Returns the internal bit sequence of the DNA sequence.
    pub fn bit_string(&self) -> String {
        let mut bit_string = String::with_capacity(self.len() * 2);
        for i in 0..self.nucleotides.len() {
            bit_string.push_str(&format!("{:08b} ", self.nucleotides[i]));
        }
        bit_string
    }

    /// Counts every nucleotide in the DNA sequence.
    ///
    /// ```
    /// use nuc::dna::Nucleotide;
    ///
    /// let dna = nuc::dna::Dna::from_ascii("ACATGCATGACAGTT");
    /// let counts = dna.counts();
    /// assert_eq!(counts[Nucleotide::A], 5);
    /// assert_eq!(counts[Nucleotide::C], 3);
    /// assert_eq!(counts[Nucleotide::G], 3);
    /// assert_eq!(counts[Nucleotide::T], 4);
    /// ```
    pub fn counts(&self) -> NucCount {
        let mut counts = NucCount { a: 0, c: 0, g: 0, t: 0 };
        for i in 0..self.length {
            match self.get(i) {
                1 => counts.c += 1,
                2 => counts.g += 1,
                3 => counts.t += 1,
                _ => counts.a += 1,
            }
        }
        counts
    }

    /// Initially sets the base at the given index (0-based).
    ///
    /// Note: If the index already contains set bits, bit patterns may cause bugs.
    pub(crate) fn init_with(&mut self, index: usize, nucleotide: u8) {
        let (block, bit) = self.address(index);
        self.nucleotides[block] |= nucleotide << bit;
    }

    /// Returns the base at the given index.
    ///
    /// ```
    /// let dna = nuc::dna::Dna::from_ascii("ACGT");
    ///
    /// assert_eq!(dna.get(0), 0);
    /// assert_eq!(dna.get(1), 1);
    /// assert_eq!(dna.get(2), 2);
    /// assert_eq!(dna.get(3), 3);
    /// ```
    pub fn get(&self, index: usize) -> u8 {
        let (block, bit) = self.address(index);
        (self.nucleotides[block] >> bit) & MASK
    }

    /// Computes the reverse complement of the DNA sequence.
    ///
    /// ```
    /// let dna = nuc::dna::Dna::from_ascii("ATGCCGTA").rc();
    /// assert_eq!(dna.to_string(), "TACGGCAT");
    /// ```
    pub fn rc(&self) -> Dna {
        let mut rc = Dna::new(self.length);
        for i in 0..self.length {
            rc.init_with(i, 3 - self.get(self.length - 1 - i));
        }
        rc
    }

    /// Appends a DNA sequence to the end of the current sequence.
    ///
    /// ```
    /// let mut dna = nuc::dna::Dna::from_ascii("ATGCCGTA");
    /// dna.append(&nuc::dna::Dna::from_ascii("TACCAT"));
    /// assert_eq!(dna.to_string(), "ATGCCGTATACCAT");
    /// ```
    pub fn append(&mut self, dna: &Dna) {
        self.nucleotides
            .resize(Dna::bytes_to_store(self.length + dna.length), 0);

        for i in 0..dna.length {
            self.init_with(self.length + i, dna.get(i));
        }
        self.length += dna.length;
    }

    /// Trims the DNA sequence to the given size.
    ///
    /// ```
    /// let mut dna = nuc::dna::Dna::from_ascii("ATGCCGTA");
    /// dna.trim(5);
    /// assert_eq!(dna.to_string(), "ATGCC");
    /// ```
    pub fn trim(&mut self, size: usize) {
        self.length = size;
        self.nucleotides.truncate(Dna::bytes_to_store(size));
    }

    /// Computes the number of bytes to store the given length.
    ///
    /// It assumes that each nucleotide is stored in 2 bits.
    pub fn bytes_to_store(length: usize) -> usize {
        ((length as f32) / NUCS_PER_BLOCK as f32).ceil() as usize
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.length {
            let c = match self.get(i) {
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => 'A',
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Dna {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Dna::from_ascii(value))
    }
}

impl PartialEq for Dna {
    fn eq(&self, other: &Self) -> bool {
        self.nucleotides == other.nucleotides && self.length == other.length
    }
}

impl PartialOrd for Dna {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.nucleotides.cmp(&other.nucleotides))
    }
}

impl Ord for Dna {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        PartialOrd::partial_cmp(self, other).unwrap()
    }
}

impl Index<usize> for Dna {
    type Output = Nucleotide;

    fn index(&self, index: usize) -> &Self::Output {
        match self.get(index) {
            1 => &NUCS[1],
            2 => &NUCS[2],
            3 => &NUCS[3],
            _ => &NUCS[0],
        }
    }
}

pub struct  NucCount {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}

impl Index<Nucleotide> for NucCount {
    type Output = usize;

    fn index(&self, index: Nucleotide) -> &Self::Output {
        match index {
            Nucleotide::A => &self.a,
            Nucleotide::C => &self.c,
            Nucleotide::G => &self.g,
            Nucleotide::T => &self.t,
        }
    }
}
