use criterion::{criterion_group, criterion_main, Criterion};
use nuc::dna::Dna;

pub fn hash_nucleotides_benchmark(c: &mut Criterion) {
    let dna = Dna::random(1_000_000);
    let bytes = dna.as_bytes();
    c.bench_function("Hashing random nucleotides", |b| {
        b.iter(|| {
            bytes
                .chunks(4)
                .map(|chunk| nuc::hash::hash_chars_be(chunk))
                .count();
        })
    });
}

criterion_group!(hash_benches, hash_nucleotides_benchmark);
criterion_main!(hash_benches);
