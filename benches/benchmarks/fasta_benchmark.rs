use std::fs::File;

use criterion::{criterion_group, Criterion};
use nuc::fasta::FastaReader;

pub fn parsing_chromosome1_benchmark(c: &mut Criterion) {
    c.bench_function("parsing a random 3MB FASTA file", |b| {
        b.iter(|| {
            let reader = FastaReader::new(File::open("benches/random.fa").unwrap());
            assert_eq!(reader.into_iter().count(), 13);
        })
    });
}

criterion_group!(fasta_benches, parsing_chromosome1_benchmark);
