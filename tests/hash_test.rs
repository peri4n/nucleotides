use nuc::hash::{hash_chars_be, hash_chars_le};

#[test]
fn test_hash_chars_big_endian() {
    assert_eq!(hash_chars_be(b"AAAA"), 0);
    assert_eq!(hash_chars_be(b"AAAC"), 1);
    assert_eq!(hash_chars_be(b"AAAG"), 2);
    assert_eq!(hash_chars_be(b"AAAT"), 3);
    assert_eq!(hash_chars_be(b"AACA"), 4);
    assert_eq!(hash_chars_be(b"AACC"), 5);
    assert_eq!(hash_chars_be(b"AACG"), 6);
    assert_eq!(hash_chars_be(b"AACT"), 7);
}

#[test]
fn test_hash_chars_litte_endian() {
    assert_eq!(hash_chars_le(b"AAAA"), 0);
    assert_eq!(hash_chars_le(b"CAAA"), 1);
    assert_eq!(hash_chars_le(b"GAAA"), 2);
    assert_eq!(hash_chars_le(b"TAAA"), 3);
    assert_eq!(hash_chars_le(b"ACAA"), 4);
    assert_eq!(hash_chars_le(b"CCAA"), 5);
    assert_eq!(hash_chars_le(b"GCAA"), 6);
    assert_eq!(hash_chars_le(b"TCAA"), 7);
}
