use criterion::{criterion_group, Criterion};
use nuc::dna::Dna;

pub fn counting_nucleotides_benchmark(c: &mut Criterion) {
    let dna = Dna::random(1_000_000);
    c.bench_function("count nucleotides", |b| b.iter(|| dna.counts()));
}

pub fn reverse_complement_benchmark(c: &mut Criterion) {
    let dna = Dna::random(1_000_000);
    c.bench_function("create the reverse complement", |b| b.iter(|| dna.rc()));
}

criterion_group!(dna_benches, counting_nucleotides_benchmark, reverse_complement_benchmark);
