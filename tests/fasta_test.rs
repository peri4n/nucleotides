
use nuc::{
    dna::Dna,
    fasta::{self, FastaDna},
};

#[test]
fn can_read_an_example_fasta_file() {
    let fasta_content = ">Some Identifier\n\
                         ATGCCGTA\n\
                         >Another Identifier\n\
                         CTAACGAA\n\
                         >Yet Another Identifier\n\
                         ATAC\n\
                         ATAAGTAGGG";
    let records = fasta::parse_records(fasta_content).unwrap().1;
    assert_eq!(
        records,
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
    let records = fasta::parse_records("").unwrap().1;
    assert_eq!(records, vec![]);
}
