use std::fmt;
use std::mem::size_of;

const BITS_PER_NUCLEOTIDE: usize = 2;

const NUCS_PER_BLOCK: usize = size_of::<u8>() * 4;

const MASK: u8 = 3;

#[derive(Debug, Eq)]
pub struct Dna {
    length: usize,
    nucleotides: Vec<u8>,
}

impl Dna {
    pub fn new(length: usize) -> Dna {
        Dna {
            length,
            nucleotides: vec![0; length],
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Dna {
        Dna {
            length: bytes.len(),
            nucleotides: bytes.to_vec(),
        }
    }

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

    #[inline(always)]
    pub fn address(&self, index: usize) -> (usize, u8) {
        let block = index / NUCS_PER_BLOCK;
        let bit = ((NUCS_PER_BLOCK - 1 - (index % NUCS_PER_BLOCK)) * BITS_PER_NUCLEOTIDE) as u8;
        (block, bit)
    }

    /// Returns the internal bit sequence of the DNA sequence.
    pub fn bit_string(&self) -> String {
        let mut bit_string = String::new();
        for i in 0..self.nucleotides.len() {
            bit_string.push_str(&format!("{:08b} ", self.nucleotides[i]));
        }
        bit_string
    }

    /// Initially sets the base at the given index (0-based).
    ///
    /// Note: If the index already contains set bits, bit patterns may cause bugs.
    fn init_with(&mut self, index: usize, nucleotide: u8) {
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

    fn bytes_to_store(length: usize) -> usize {
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

#[cfg(test)]
mod test {
    use super::Dna;

    #[test]
    fn can_be_created_from_a_string() {
        let dna = Dna::try_from("ATGCCGTA").unwrap();

        assert_eq!(dna.get(0), 0);
        assert_eq!(dna.get(1), 3);
        assert_eq!(dna.get(2), 2);
        assert_eq!(dna.get(3), 1);
        assert_eq!(dna.get(4), 1);
        assert_eq!(dna.get(5), 2);
        assert_eq!(dna.get(6), 3);
        assert_eq!(dna.get(7), 0);
        assert_eq!(dna.length, 8);
    }

    #[test]
    fn can_be_sorted() {
        let mut sequences = vec![
            Dna::from_ascii("ATGCCGTA"),
            Dna::from_ascii("CTAACGAA"),
            Dna::from_ascii("ATAC"),
            Dna::from_ascii("ATAA"),
            Dna::from_ascii("GTAGGG"),
        ];
        sequences.sort();

        assert_eq!(
            sequences,
            vec![
                Dna::from_ascii("ATAA"),
                Dna::from_ascii("ATAC"),
                Dna::from_ascii("ATGCCGTA"),
                Dna::from_ascii("CTAACGAA"),
                Dna::from_ascii("GTAGGG")
            ]
        );
    }

    #[test]
    fn can_be_appended() {
        let mut dna = Dna::from_ascii("ATGCCGTA");
        dna.append(&Dna::from_ascii("AAA"));
        dna.append(&Dna::from_ascii("CCC"));
        dna.append(&Dna::from_ascii("GGG"));
        dna.append(&Dna::from_ascii("TTT"));
        dna.append(&Dna::from_ascii(""));

        assert_eq!(dna.to_string(), "ATGCCGTAAAACCCGGGTTT");
    }
}
