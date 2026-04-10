//! `nuc` is a library for working with biological sequences.
//!
//! It's goal is to provide the fastest and easiest way to work with DNA, RNA, and amino acid sequences.
//!

/// Defines core biological alphabets and their properties.
pub mod alphabet;

/// Core functionality for working with biological sequences.
pub mod seq;

/// Handles IO with FastA files.
pub mod io;

/// Handles hashing of DNA sequences.
pub mod hash;
