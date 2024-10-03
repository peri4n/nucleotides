use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::dna_benchmark::dna_benches,
    benchmarks::fasta_benchmark::fasta_benches,
    benchmarks::hash_benchmark::hash_benches,
}
