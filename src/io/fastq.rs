use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
pub struct FastqReader<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read + std::fmt::Debug> FastqReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }
}

impl<R: Read + std::fmt::Debug> Iterator for FastqReader<R> {
    type Item = FastqRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let mut id = String::new();
        let mut sequence = String::new();
        let mut qualities = String::new();

        for _ in 0..4 {
            let mut line = String::new();
            self.reader.read_line(&mut line).ok()?;
            match line.chars().next()? {
                '@' => id = line[1..].trim().to_string(),
                '+' => (),
                _ => {
                    if sequence.is_empty() {
                        sequence.push_str(&line.trim());
                    } else {
                        qualities.push_str(&line.trim());
                    }
                }
            }
        }

        Some(FastqRecord::new(
            id,
            Dna::from_ascii(&sequence),
            qualities.into_bytes(),
        ))
    }
}

use crate::dna::Dna;

#[derive(Debug, PartialEq)]
pub struct FastqRecord {
    pub id: String,
    pub sequence: Dna,
    pub qualities: Vec<u8>,
}

impl FastqRecord {
    pub fn new(id: String, sequence: Dna, qualities: Vec<u8>) -> Self {
        Self {
            id,
            sequence,
            qualities,
        }
    }
}
