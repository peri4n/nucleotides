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
            nucleotides: vec![0; ((ascii.len() as f32) / NUCS_PER_BLOCK as f32).ceil() as usize],
        };

        for (i, c) in ascii.char_indices() {
            match c {
                'C' | 'c' => dna.set_nucleotide(i, 1),
                'G' | 'g' => dna.set_nucleotide(i, 2),
                'T' | 't' => dna.set_nucleotide(i, 3),
                _ => dna.set_nucleotide(i, 0),
            }
        }

        dna
    }

    #[inline(always)]
    fn address(&self, index: usize) -> (usize, u8) {
        let block = index / NUCS_PER_BLOCK;
        let bit = ((NUCS_PER_BLOCK - 1 - (index % NUCS_PER_BLOCK)) * BITS_PER_NUCLEOTIDE) as u8;
        (block, bit)
    }

    fn set_nucleotide(&mut self, index: usize, nucleotide: u8) {
        let (block, bit) = self.address(index);
        self.nucleotides[block] |= nucleotide << bit;
    }

    pub fn get(&self, index: usize) -> u8 {
        let (block, bit) = self.address(index);
        (self.nucleotides[block] >> bit) & MASK
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

impl PartialOrd for Dna{
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
    fn can_be_converted_to_a_string() {
        let str = "ATACCGTA";
        assert_eq!(Dna::from_ascii(str).to_string(), str);
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

        assert_eq!(sequences, vec![
            Dna::from_ascii("ATAA"),
            Dna::from_ascii("ATAC"),
            Dna::from_ascii("ATGCCGTA"),
            Dna::from_ascii("CTAACGAA"),
            Dna::from_ascii("GTAGGG")
        ]);
    }
}
