use nuc::dna::Dna;

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
    assert_eq!(dna.len(), 8);
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
