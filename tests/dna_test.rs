use nuc::dna::{Dna, Nucleotide};

#[test]
fn can_be_created_from_a_string_with_perfect_alignment() {
    let dna = Dna::try_from("ATGCCGTA").unwrap();

    assert_eq!(*dna.as_bytes(), vec![0b00111001, 0b01101100]);

    assert_eq!(dna[0], Nucleotide::A);
    assert_eq!(dna[1], Nucleotide::T);
    assert_eq!(dna[2], Nucleotide::G);
    assert_eq!(dna[3], Nucleotide::C);
    assert_eq!(dna[4], Nucleotide::C);
    assert_eq!(dna[5], Nucleotide::G);
    assert_eq!(dna[6], Nucleotide::T);
    assert_eq!(dna[7], Nucleotide::A);
    assert_eq!(dna.len(), 8);
}

#[test]
fn can_be_created_from_a_string_three_empty_nucleotides() {
    let dna = Dna::try_from("TGCAT").unwrap();

    assert_eq!(*dna.as_bytes(), vec![0b11100100, 0b11000000]);

    assert_eq!(dna[0], Nucleotide::T);
    assert_eq!(dna[1], Nucleotide::G);
    assert_eq!(dna[2], Nucleotide::C);
    assert_eq!(dna[3], Nucleotide::A);
    assert_eq!(dna[4], Nucleotide::T);
    assert_eq!(dna.len(), 5);
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

    println!("{:?}", sequences);

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

#[test]
fn kmer_cache_contains_correct_value() {
    let cache = &nuc::dna::NUC_COUNT_CACHE;
    assert_eq!(cache[0], [4, 0, 0, 0]);
    assert_eq!(cache[1], [3, 1, 0, 0]);
    assert_eq!(cache[2], [3, 0, 1, 0]);
    assert_eq!(cache[3], [3, 0, 0, 1]);
    assert_eq!(cache[4], [3, 1, 0, 0]);
    assert_eq!(cache[5], [2, 2, 0, 0]);

    cache.iter().for_each(|&kmer| {
        assert_eq!(kmer[0] + kmer[1] + kmer[2] + kmer[3], 4);
    });
}
