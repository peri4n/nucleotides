#![feature(portable_simd)]
//! `nuc` is a library for working with nucleotide sequences.
//!
//! It's goal is to provide the fastest and easiest way to work with DNA and RNA sequences.

/// Core functionality for working with nucleotide sequences.
pub mod dna;

/// Handles IO with FastA files.
pub mod fasta;

/// Handles hashing of DNA sequences.
pub mod hash;
