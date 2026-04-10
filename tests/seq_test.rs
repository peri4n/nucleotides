use nuc::{
    alphabet::{Alphabet, Nuc4, Nuc5},
    seq::*,
};
use proptest::prelude::prop;

// -- empty sequences --

#[test]
fn empty_nuc4() {
    let seq = Seq::<Nuc4>::try_from("").unwrap();
    assert!(seq.is_empty());
    assert_eq!(seq.len(), 0);
    assert_eq!(seq.to_string(), "");
}

#[test]
fn empty_nuc5() {
    let seq = Seq::<Nuc5>::try_from("").unwrap();
    assert!(seq.is_empty());
    assert_eq!(seq.len(), 0);
    assert_eq!(seq.to_string(), "");
}

proptest::proptest! {

    // -- Construction roundtrips --

    #[test]
    fn nuc4_roundtrip(s in "[ATGC]{0,100}") {
        let seq = Seq::<Nuc4>::try_from(s.as_str()).unwrap();
        assert_eq!(seq.to_string(), s);
        assert_eq!(seq.len(), s.len());
    }

    #[test]
    fn nuc5_roundtrip(s in "[ATGCN]{0,100}") {
        let seq = Seq::<Nuc5>::try_from(s.as_str()).unwrap();
        assert_eq!(seq.to_string(), s);
        assert_eq!(seq.len(), s.len());
    }

    #[test]
    fn nuc4_case_insensitive(s in "[ATGCatgc]{0,100}") {
        let seq = Seq::<Nuc4>::try_from(s.as_str()).unwrap();
        assert_eq!(seq.to_string(), s.to_uppercase());
        assert_eq!(seq.len(), s.len());
    }

    #[test]
    fn nuc5_case_insensitive(s in "[ATGCNatgcn]{0,100}") {
        let seq = Seq::<Nuc5>::try_from(s.as_str()).unwrap();
        assert_eq!(seq.to_string(), s.to_uppercase());
        assert_eq!(seq.len(), s.len());
    }

    #[test]
    fn from_bytes_roundtrip(bytes in prop::collection::vec(0u8..=255, 0..100)) {
        let seq = Seq::<Nuc4>::from_bytes(&bytes);
        assert_eq!(seq.as_bytes(), &bytes);
    }

    // -- get() returns correct element --

    #[test]
    fn nuc4_get_matches_string(s in "[ATGC]{1,100}") {
        let seq = Seq::<Nuc4>::try_from(s.as_str()).unwrap();
        let bytes = s.as_bytes();
        for i in 0..s.len() {
            assert_eq!(Nuc4::to_byte(seq.get(i)), bytes[i]);
        }
    }

    #[test]
    fn nuc5_get_matches_string(s in "[ATGCN]{1,100}") {
        let seq = Seq::<Nuc5>::try_from(s.as_str()).unwrap();
        let bytes = s.as_bytes();
        for i in 0..s.len() {
            assert_eq!(Nuc5::to_byte(seq.get(i)), bytes[i]);
        }
    }

    // -- concat / append --

    #[test]
    fn concat_nuc4_nuc4(a in "[ATGC]{0,50}", b in "[ATGC]{0,50}") {
        let sa = Seq::<Nuc4>::try_from(a.as_str()).unwrap();
        let sb = Seq::<Nuc4>::try_from(b.as_str()).unwrap();
        let result = sa.concat(&sb);
        assert_eq!(result.to_string(), format!("{}{}", a, b));
        assert_eq!(result.len(), a.len() + b.len());
    }

    #[test]
    fn concat_nuc4_nuc5(a in "[ATGC]{0,50}", b in "[ATGCN]{0,50}") {
        let sa = Seq::<Nuc4>::try_from(a.as_str()).unwrap();
        let sb = Seq::<Nuc5>::try_from(b.as_str()).unwrap();
        let result = sa.concat(&sb);
        assert_eq!(result.to_string(), format!("{}{}", a, b));
        assert_eq!(result.len(), a.len() + b.len());
    }

    #[test]
    fn concat_nuc5_nuc4(a in "[ATGCN]{0,50}", b in "[ATGC]{0,50}") {
        let sa = Seq::<Nuc5>::try_from(a.as_str()).unwrap();
        let sb = Seq::<Nuc4>::try_from(b.as_str()).unwrap();
        let result = sa.concat(&sb);
        assert_eq!(result.to_string(), format!("{}{}", a, b));
        assert_eq!(result.len(), a.len() + b.len());
    }

    #[test]
    fn concat_nuc5_nuc5(a in "[ATGCN]{0,50}", b in "[ATGCN]{0,50}") {
        let sa = Seq::<Nuc5>::try_from(a.as_str()).unwrap();
        let sb = Seq::<Nuc5>::try_from(b.as_str()).unwrap();
        let result = sa.concat(&sb);
        assert_eq!(result.to_string(), format!("{}{}", a, b));
        assert_eq!(result.len(), a.len() + b.len());
    }

    #[test]
    fn append_consumes_self(a in "[ATGC]{0,50}", b in "[ATGCN]{0,50}") {
        let sa = Seq::<Nuc4>::try_from(a.as_str()).unwrap();
        let sb = Seq::<Nuc5>::try_from(b.as_str()).unwrap();
        let result = sa.append(&sb);
        assert_eq!(result.to_string(), format!("{}{}", a, b));
    }

    // -- trim --

    #[test]
    fn trim_preserves_prefix(s in "[ATGC]{1,100}", trim_to in 0usize..100) {
        let trim_to = trim_to.min(s.len());
        let mut seq = Seq::<Nuc4>::try_from(s.as_str()).unwrap();
        seq.trim(trim_to);
        assert_eq!(seq.len(), trim_to);
        assert_eq!(seq.to_string(), &s[..trim_to]);
    }

    // -- ordering --

    #[test]
    fn ord_consistent_with_string(a in "[ATGC]{4}", b in "[ATGC]{4}") {
        let sa = Seq::<Nuc4>::try_from(a.as_str()).unwrap();
        let sb = Seq::<Nuc4>::try_from(b.as_str()).unwrap();
        assert_eq!(sa.cmp(&sb), a.cmp(&b));
    }

    // -- equality --

    #[test]
    fn eq_reflexive(s in "[ATGC]{0,100}") {
        let sa = Seq::<Nuc4>::try_from(s.as_str()).unwrap();
        let sb = Seq::<Nuc4>::try_from(s.as_str()).unwrap();
        assert_eq!(sa, sb);
    }

    #[test]
    fn neq_different(a in "[ATGC]{1,50}", b in "[ATGC]{1,50}") {
        let sa = Seq::<Nuc4>::try_from(a.as_str()).unwrap();
        let sb = Seq::<Nuc4>::try_from(b.as_str()).unwrap();
        assert_eq!(sa == sb, a == b);
    }

}
