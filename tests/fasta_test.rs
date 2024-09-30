use std::fs::File;

use nuc::{
    dna::Dna,
    fasta::{FastaDna, FastaReader},
};

#[test]
fn can_read_an_example_fasta_file() {
    let reader = FastaReader::new(
        ">Some Identifier\n\
                         ATGCCGTA\n\
                         >Another Identifier\n\
                         CTAACGAA\n\
                         >Yet Another Identifier\n\
                         ATAC\n\
                         ATAAGTAGGG"
            .as_bytes()
    );

    assert_eq!(
        reader.into_iter().collect::<Vec<_>>(),
        vec![
            FastaDna {
                id: "Some Identifier".to_string(),
                sequence: Dna::from_ascii("ATGCCGTA")
            },
            FastaDna {
                id: "Another Identifier".to_string(),
                sequence: Dna::from_ascii("CTAACGAA")
            },
            FastaDna {
                id: "Yet Another Identifier".to_string(),
                sequence: Dna::from_ascii("ATACATAAGTAGGG")
            },
        ]
    );
}

#[test]
fn can_read_an_empty_fasta_file() {
    let records = FastaReader::new("".as_bytes()).into_iter().collect::<Vec<_>>();
    assert_eq!(records, vec![]);
}

#[test]
fn read_chromosome1() {
    let reader = FastaReader::new(File::open("/home/fbull/test.fa").unwrap());

    assert_eq!(reader.into_iter().count(), 1);
}
