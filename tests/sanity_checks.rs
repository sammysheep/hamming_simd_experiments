use hamming::*;

#[test]
fn scalar() {
    let n = scalar_hamming(S1, S2);
    let d = scalar_hamming1b(S1, S2) as usize;
    assert_eq!(n, d);

    let n = scalar_hamming(L1, L2);
    let d = scalar_hamming1b(L1, L2) as usize;
    assert_eq!(n, d);
}

#[test]
fn chunk() {
    let n = scalar_hamming(S1, S2);

    let d = simd_chunk_ne_hd::<32>(S1, S2);
    assert!(n == d);
    let d = simd_chunk_ne_hd::<16>(S1, S2);
    assert!(n == d);
    let d = simd_chunk_eq_hd::<32>(S1, S2);
    assert!(n == d);
    let d = simd_chunk_eq_hd::<16>(S1, S2);
    assert!(n == d);

    let n = scalar_hamming(L1, L2);

    let d = simd_chunk_ne_hd::<32>(L1, L2);
    assert!(n == d);
    let d = simd_chunk_ne_hd::<16>(L1, L2);
    assert!(n == d);
    let d = simd_chunk_eq_hd::<32>(L1, L2);
    assert!(n == d);
    let d = simd_chunk_eq_hd::<16>(L1, L2);
    assert!(n == d);
}

#[test]
fn aligned() {
    let n = scalar_hamming(S1, S2);

    let d = simd_aligned_ne_hd::<16>(S1, S2);
    assert!(n == d);
    let d = simd_aligned_eq_hd::<16>(S1, S2);
    assert!(n == d);

    let n = scalar_hamming(L1, L2);

    let d = simd_aligned_ne_hd::<32>(L1, L2);
    assert!(n == d);
    let d = simd_aligned_eq_hd::<32>(L1, L2);
    assert!(n == d);
}

#[test]
fn fold_reduce_select() {
    let n = scalar_hamming(S1, S2);

    let d = simd_reduce_ne_hd::<16>(S1, S2);
    assert!(n == d);

    let n = scalar_hamming(L1, L2);

    let d = simd_fold_ne_hd::<16>(L1, L2);
    assert!(n == d);
    let d = simd_reduce_ne_hd::<32>(L1, L2);
    assert!(n == d);
}

#[test]
fn xor() {
    let n = scalar_hamming(L1, L2);

    let d = simd_chunk_xor_hd::<16>(L1, L2);
    assert!(n == d);
    let d = simd_chunk_xor_hd::<32>(L1, L2);
    assert!(n == d);
    let d = simd_chunk_select_hd::<32>(L1, L2);
    assert!(n == d);
}

#[test]
fn for_while() {
    let n = scalar_hamming(L1, L2);

    let d = simd_for_ne_hd::<32>(L1, L2);
    assert!(n == d);
    let d = simd_while_ne_hd::<32>(L1, L2);
    assert!(n == d);
}

#[test]
fn bitmask() {
    let n = scalar_hamming(L1, L2);

    let d = simd_chunk_bitmask_hd::<16>(L1, L2);
    assert_eq!(n, d);
    let d = simd_chunk_bitmask_hd::<32>(L1, L2);
    assert_eq!(n, d);
}
