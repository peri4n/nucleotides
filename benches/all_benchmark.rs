use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::fasta_benchmark::fasta_benches,
    benchmarks::dna_benchmark::dna_benches,
}
