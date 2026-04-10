use nuc::alphabet::*;

// -- Nuc4: from_byte / to_byte roundtrip for every valid byte ----------------

#[test]
fn nuc4_from_to_byte_roundtrip() {
    for &b in b"ACGTacgt" {
        let elem = Nuc4::from_byte(b);
        let out = Nuc4::to_byte(elem);
        assert_eq!(out, b.to_ascii_uppercase());
    }
}

#[test]
fn nuc5_from_to_byte_roundtrip() {
    for &b in b"ACGTNacgtn" {
        let elem = Nuc5::from_byte(b);
        let out = Nuc5::to_byte(elem);
        assert_eq!(out, b.to_ascii_uppercase());
    }
}

#[test]
fn aa20_from_to_byte_roundtrip() {
    for &b in b"ACDEFGHIKLMNPQRSTVWYacdefghiklmnpqrstvwy" {
        let elem = AA20::from_byte(b);
        let out = AA20::to_byte(elem);
        assert_eq!(out, b.to_ascii_uppercase());
    }
}

// -- Discriminant matches bit encoding ---------------------------------------

#[test]
fn nuc4_discriminant_matches_bits() {
    for (i, &elem) in Nuc4::ELEMENTS.iter().enumerate() {
        let bits: u8 = elem.into();
        assert_eq!(bits as usize, i);
    }
}

#[test]
fn nuc5_discriminant_matches_bits() {
    for (i, &elem) in Nuc5::ELEMENTS.iter().enumerate() {
        let bits: u8 = elem.into();
        assert_eq!(bits as usize, i);
    }
}

#[test]
fn aa20_discriminant_matches_bits() {
    for (i, &elem) in AA20::ELEMENTS.iter().enumerate() {
        let bits: u8 = elem.into();
        assert_eq!(bits as usize, i);
    }
}

// -- ELEMENTS length matches SIZE --------------------------------------------

#[test]
fn elements_len_matches_size() {
    assert_eq!(Nuc4::ELEMENTS.len(), Nuc4::SIZE as usize);
    assert_eq!(Nuc5::ELEMENTS.len(), Nuc5::SIZE as usize);
    assert_eq!(AA20::ELEMENTS.len(), AA20::SIZE as usize);
}

// -- BITS is wide enough for SIZE --------------------------------------------

#[test]
fn bits_wide_enough() {
    assert!(1u16 << Nuc4::BITS >= Nuc4::SIZE as u16);
    assert!(1u16 << Nuc5::BITS >= Nuc5::SIZE as u16);
    assert!(1u16 << AA20::BITS >= AA20::SIZE as u16);
}

// -- LUT: invalid bytes map to 0xFF -----------------------------------------

#[test]
fn nuc4_lut_rejects_invalid() {
    for b in 0u8..=255 {
        let v = Nuc4::BYTE_TO_BITS[b as usize];
        if b"ACGTacgt".contains(&b) {
            assert_ne!(v, 0xFF, "valid byte {b} mapped to 0xFF");
        } else {
            assert_eq!(v, 0xFF, "invalid byte {b} not 0xFF, got {v}");
        }
    }
}

#[test]
fn nuc5_lut_rejects_invalid() {
    for b in 0u8..=255 {
        let v = Nuc5::BYTE_TO_BITS[b as usize];
        if b"ACGTNacgtn".contains(&b) {
            assert_ne!(v, 0xFF, "valid byte {b} mapped to 0xFF");
        } else {
            assert_eq!(v, 0xFF, "invalid byte {b} not 0xFF, got {v}");
        }
    }
}

#[test]
fn aa20_lut_rejects_invalid() {
    let valid = b"ACDEFGHIKLMNPQRSTVWYacdefghiklmnpqrstvwy";
    for b in 0u8..=255 {
        let v = AA20::BYTE_TO_BITS[b as usize];
        if valid.contains(&b) {
            assert_ne!(v, 0xFF, "valid byte {b} mapped to 0xFF");
        } else {
            assert_eq!(v, 0xFF, "invalid byte {b} not 0xFF, got {v}");
        }
    }
}

// -- LUT agrees with from_byte for all valid bytes ---------------------------

#[test]
fn nuc4_lut_consistent_with_from_byte() {
    for &b in b"ACGTacgt" {
        let elem = Nuc4::from_byte(b);
        let bits_from_lut = Nuc4::BYTE_TO_BITS[b as usize];
        let bits_from_elem: u8 = elem.into();
        assert_eq!(bits_from_lut, bits_from_elem);
    }
}

#[test]
fn nuc5_lut_consistent_with_from_byte() {
    for &b in b"ACGTNacgtn" {
        let elem = Nuc5::from_byte(b);
        let bits_from_lut = Nuc5::BYTE_TO_BITS[b as usize];
        let bits_from_elem: u8 = elem.into();
        assert_eq!(bits_from_lut, bits_from_elem);
    }
}

#[test]
fn aa20_lut_consistent_with_from_byte() {
    for &b in b"ACDEFGHIKLMNPQRSTVWYacdefghiklmnpqrstvwy" {
        let elem = AA20::from_byte(b);
        let bits_from_lut = AA20::BYTE_TO_BITS[b as usize];
        let bits_from_elem: u8 = elem.into();
        assert_eq!(bits_from_lut, bits_from_elem);
    }
}

// -- Case insensitivity: upper and lower give same element -------------------

#[test]
fn nuc4_case_insensitive() {
    for (&u, &l) in b"ACGT".iter().zip(b"acgt".iter()) {
        assert_eq!(Nuc4::from_byte(u), Nuc4::from_byte(l));
    }
}

#[test]
fn nuc5_case_insensitive() {
    for (&u, &l) in b"ACGTN".iter().zip(b"acgtn".iter()) {
        assert_eq!(Nuc5::from_byte(u), Nuc5::from_byte(l));
    }
}

#[test]
fn aa20_case_insensitive() {
    for (&u, &l) in b"ACDEFGHIKLMNPQRSTVWY"
        .iter()
        .zip(b"acdefghiklmnpqrstvwy".iter())
    {
        assert_eq!(AA20::from_byte(u), AA20::from_byte(l));
    }
}

// -- to_byte always returns uppercase ASCII ----------------------------------

#[test]
fn nuc4_to_byte_uppercase() {
    for &elem in Nuc4::ELEMENTS {
        let b = Nuc4::to_byte(elem);
        assert!(
            b.is_ascii_uppercase(),
            "to_byte returned non-uppercase: {b}"
        );
    }
}

#[test]
fn nuc5_to_byte_uppercase() {
    for &elem in Nuc5::ELEMENTS {
        let b = Nuc5::to_byte(elem);
        assert!(
            b.is_ascii_uppercase(),
            "to_byte returned non-uppercase: {b}"
        );
    }
}

#[test]
fn aa20_to_byte_uppercase() {
    for &elem in AA20::ELEMENTS {
        let b = AA20::to_byte(elem);
        assert!(
            b.is_ascii_uppercase(),
            "to_byte returned non-uppercase: {b}"
        );
    }
}

// -- ELEMENTS has no duplicates ----------------------------------------------

#[test]
fn nuc4_elements_unique() {
    for i in 0..Nuc4::ELEMENTS.len() {
        for j in (i + 1)..Nuc4::ELEMENTS.len() {
            assert_ne!(Nuc4::ELEMENTS[i], Nuc4::ELEMENTS[j]);
        }
    }
}

#[test]
fn nuc5_elements_unique() {
    for i in 0..Nuc5::ELEMENTS.len() {
        for j in (i + 1)..Nuc5::ELEMENTS.len() {
            assert_ne!(Nuc5::ELEMENTS[i], Nuc5::ELEMENTS[j]);
        }
    }
}

#[test]
fn aa20_elements_unique() {
    for i in 0..AA20::ELEMENTS.len() {
        for j in (i + 1)..AA20::ELEMENTS.len() {
            assert_ne!(AA20::ELEMENTS[i], AA20::ELEMENTS[j]);
        }
    }
}

// -- Nuc4 is a subset of Nuc5 ------------------------------------------------

#[test]
fn nuc4_subset_of_nuc5() {
    for &elem in Nuc4::ELEMENTS {
        let byte = Nuc4::to_byte(elem);
        let nuc5_elem = Nuc5::from_byte(byte);
        assert_eq!(elem, nuc5_elem);
    }
}

// -- Proptest: from_byte(to_byte(e)) == e for random elements ----------------

proptest::proptest! {
    #[test]
    fn nuc4_element_roundtrip(idx in 0usize..4) {
        let elem = Nuc4::ELEMENTS[idx];
        let byte = Nuc4::to_byte(elem);
        let back = Nuc4::from_byte(byte);
        assert_eq!(elem, back);
    }

    #[test]
    fn nuc5_element_roundtrip(idx in 0usize..5) {
        let elem = Nuc5::ELEMENTS[idx];
        let byte = Nuc5::to_byte(elem);
        let back = Nuc5::from_byte(byte);
        assert_eq!(elem, back);
    }

    #[test]
    fn aa20_element_roundtrip(idx in 0usize..20) {
        let elem = AA20::ELEMENTS[idx];
        let byte = AA20::to_byte(elem);
        let back = AA20::from_byte(byte);
        assert_eq!(elem, back);
    }

    // -- Proptest: random valid ASCII always roundtrips -----------------------

    #[test]
    fn nuc4_random_valid_byte(idx in 0usize..8) {
        let valid = b"ACGTacgt";
        let b = valid[idx];
        let elem = Nuc4::from_byte(b);
        let out = Nuc4::to_byte(elem);
        assert_eq!(out, b.to_ascii_uppercase());
    }

    #[test]
    fn nuc5_random_valid_byte(idx in 0usize..10) {
        let valid = b"ACGTNacgtn";
        let b = valid[idx];
        let elem = Nuc5::from_byte(b);
        let out = Nuc5::to_byte(elem);
        assert_eq!(out, b.to_ascii_uppercase());
    }

    #[test]
    fn aa20_random_valid_byte(idx in 0usize..40) {
        let valid = b"ACDEFGHIKLMNPQRSTVWYacdefghiklmnpqrstvwy";
        let b = valid[idx];
        let elem = AA20::from_byte(b);
        let out = AA20::to_byte(elem);
        assert_eq!(out, b.to_ascii_uppercase());
    }

    // -- Proptest: invalid bytes give 0xFF in LUT ----------------------------

    #[test]
    fn nuc4_invalid_byte_rejected(b in 0u8..=255u8) {
        if !b"ACGTacgt".contains(&b) {
            assert_eq!(Nuc4::BYTE_TO_BITS[b as usize], 0xFF);
        }
    }

    #[test]
    fn nuc5_invalid_byte_rejected(b in 0u8..=255u8) {
        if !b"ACGTNacgtn".contains(&b) {
            assert_eq!(Nuc5::BYTE_TO_BITS[b as usize], 0xFF);
        }
    }

    #[test]
    fn aa20_invalid_byte_rejected(b in 0u8..=255u8) {
        if !b"ACDEFGHIKLMNPQRSTVWYacdefghiklmnpqrstvwy".contains(&b) {
            assert_eq!(AA20::BYTE_TO_BITS[b as usize], 0xFF);
        }
    }

    // -- Proptest: BYTE_TO_BITS[b] < SIZE for all valid b --------------------

    #[test]
    fn nuc4_lut_values_in_range(b in 0u8..=255u8) {
        let v = Nuc4::BYTE_TO_BITS[b as usize];
        assert!(v == 0xFF || v < Nuc4::SIZE);
    }

    #[test]
    fn nuc5_lut_values_in_range(b in 0u8..=255u8) {
        let v = Nuc5::BYTE_TO_BITS[b as usize];
        assert!(v == 0xFF || v < Nuc5::SIZE);
    }

    #[test]
    fn aa20_lut_values_in_range(b in 0u8..=255u8) {
        let v = AA20::BYTE_TO_BITS[b as usize];
        assert!(v == 0xFF || v < AA20::SIZE);
    }
}
