use std::io::{BufReader, BufRead, Read};

pub struct FastaReader<R: Read> {
    reader: BufReader<R>,
    next_id: Option<String>,

}

impl<R: Read> FastaReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader: BufReader::new(reader), next_id: None }
    }
}

impl<R: Read> Iterator for FastaReader<R> {
    type Item = FastaDna;

    fn next(&mut self) -> Option<Self::Item> {
        let mut id = self.next_id.take().unwrap_or_default();
        let mut sequence = String::with_capacity(1000);
        //if let Ok(buffer) = self.reader.fill_buf() {
        //
        //} else {
        //
        //}

        loop {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                // buffer is completely read
                Ok(0) => {
                    if id.is_empty() {
                        return None;
                    } else {
                        return Some(FastaDna { id, sequence: Dna::from_ascii(&sequence) });
                    }
                }
                // buffer is not empty yet
                Ok(_) => {
                    if line.starts_with('>') {
                        self.next_id = Some(line[1..].trim().to_string());
                        if !id.is_empty() {
                            return Some(FastaDna { id, sequence: Dna::from_ascii(&sequence) });
                        }
                        id = line[1..].trim().to_string();
                        sequence.clear();
                    } else {
                        sequence.push_str(&line.trim());
                    }
                }
                Err(_) => {
                    return None;
                }
            }
        }
    }
}

use crate::dna::Dna;

#[derive(Debug, PartialEq)]
pub struct FastaDna {
    pub id: String,
    pub sequence: Dna,
}
