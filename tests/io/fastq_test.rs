use nuc::{
    dna::Dna,
    io::fastq::{FastqReader, FastqRecord},
};

#[test]
fn can_read_an_example_fasta_file() {
    let reader = FastqReader::new(
        r#"@SEQ_ID
GATTTGGGGTTCAAAGCAGTATCGATCAAATAGTAAATCCATTTGTTCAACTCACAGTTT
+
!''*((((***+))%%%++)(%%%%).1***-+*''))**55CCF>>>>>>CCCCCCC65"#
            .as_bytes(),
    );

    assert_eq!(
        reader.into_iter().collect::<Vec<_>>(),
        vec![FastqRecord {
            id: "SEQ_ID".to_string(),
            sequence: Dna::from_ascii(
                "GATTTGGGGTTCAAAGCAGTATCGATCAAATAGTAAATCCATTTGTTCAACTCACAGTTT"
            ),
            qualities: vec![
                33, 39, 39, 42, 40, 40, 40, 40, 42, 42, 42, 43, 41, 41, 37, 37, 37, 43, 43, 41, 40,
                37, 37, 37, 37, 41, 46, 49, 42, 42, 42, 45, 43, 42, 39, 39, 41, 41, 42, 42, 53, 53,
                67, 67, 70, 62, 62, 62, 62, 62, 62, 67, 67, 67, 67, 67, 67, 67, 54, 53
            ]
        },]
    );
}
