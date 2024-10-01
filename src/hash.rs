use std::sync::LazyLock;

pub static CHAR_TO_TWO_BIT: LazyLock<[u8; 256]> = LazyLock::new(|| {
    let mut cache = [0; 256];
    cache[99] = 1; // lowercase c
    cache[67] = 1; // uppercase C
    cache[103] = 2; // lowercase g
    cache[71] = 2; // uppercase G
    cache[116] = 3; // lowercase t
    cache[84] = 3; // uppercase T
    cache
});

/// Hashes a byte slice into a 8-bit integer.
pub fn hash_chars_be(bytes: &[u8]) -> u8 {
    match bytes.len() {
        1 => CHAR_TO_TWO_BIT[bytes[0] as usize] << 6,
        2 => CHAR_TO_TWO_BIT[bytes[0] as usize] << 6 | (CHAR_TO_TWO_BIT[bytes[1] as usize] << 4),
        3 => CHAR_TO_TWO_BIT[bytes[0] as usize] << 6 | (CHAR_TO_TWO_BIT[bytes[1] as usize] << 4) | (CHAR_TO_TWO_BIT[bytes[2] as usize] << 2),
        _ => CHAR_TO_TWO_BIT[bytes[0] as usize] << 6 | (CHAR_TO_TWO_BIT[bytes[1] as usize] << 4) | (CHAR_TO_TWO_BIT[bytes[2] as usize] << 2) | (CHAR_TO_TWO_BIT[bytes[3] as usize]),
    }
}

/// Hashes a byte slice into a 8-bit integer.
pub fn hash_chars_le(bytes: &[u8]) -> u8 {
    CHAR_TO_TWO_BIT[bytes[0] as usize]
        | (CHAR_TO_TWO_BIT[bytes[1] as usize] << 2)
        | (CHAR_TO_TWO_BIT[bytes[2] as usize] << 4)
        | (CHAR_TO_TWO_BIT[bytes[3] as usize] << 6)
}
