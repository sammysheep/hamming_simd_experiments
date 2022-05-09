
#![feature(test)]
use triple_accel::hamming::hamming as intrinsic_hd;

extern crate test;
use test::Bencher;
use hamming::*;

#[bench]
fn scalar_hd_lg(b: &mut Bencher) {
    b.iter(|| {
        scalar_hamming(L1, L2)
    });
}

#[bench]
fn intrinsic_hd_lg(b: &mut Bencher) {
    b.iter(|| {
        intrinsic_hd(L1, L2)
    });
}

#[bench]
fn simd_chunk_eq_hd16_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_chunk_eq_hd::<16>(L1, L2)
    });
}

#[bench]
fn simd_chunk_eq_hd32_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_chunk_eq_hd::<32>(L1, L2)
    });
}

#[bench]
fn simd_chunk_ne_hd16_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_chunk_ne_hd::<16>(L1, L2)
    });
}

#[bench]
fn simd_chunk_ne_hd32_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_chunk_ne_hd::<32>(L1, L2)
    });
}

#[bench]
fn simd_chunk_xor_hd16_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_chunk_xor_hd::<16>(L1, L2)
    });
}

#[bench]
fn simd_chunk_xor_hd32_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_chunk_xor_hd::<32>(L1, L2)
    });
}

#[bench]
fn simd_for_ne_hd16_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_for_ne_hd::<16>(L1, L2)
    });
}

#[bench]
fn simd_for_ne_hd32_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_for_ne_hd::<32>(L1, L2)
    });
}

#[bench]
fn simd_while_ne_hd16_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_for_ne_hd::<16>(L1, L2)
    });
}

#[bench]
fn simd_while_ne_hd32_lg(b: &mut Bencher) {
    b.iter(|| {
        simd_for_ne_hd::<32>(L1, L2)
    });
}

#[bench]
fn simd_aligned_ne_hd16_lg(b: &mut Bencher) {
    let a1 = AlignedVec(L1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(L2.to_vec());
    let a2 = a2.get_slice();
    b.iter(|| {
        simd_aligned_ne_hd::<16>(a1, a2)
    });
}

#[bench]
fn simd_aligned_ne_hd32_lg(b: &mut Bencher) {
    let a1 = AlignedVec(L1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(L2.to_vec());
    let a2 = a2.get_slice();

    b.iter(|| {
        simd_aligned_ne_hd::<32>(a1, a2)
    });
}

#[bench]
fn simd_aligned_eq_hd16_lg(b: &mut Bencher) {
    let a1 = AlignedVec(L1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(L2.to_vec());
    let a2 = a2.get_slice();
    b.iter(|| {
        simd_aligned_eq_hd::<16>(a1, a2)
    });
}

#[bench]
fn simd_aligned_eq_hd32_lg(b: &mut Bencher) {
    let a1 = AlignedVec(L1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(L2.to_vec());
    let a2 = a2.get_slice();

    b.iter(|| {
        simd_aligned_eq_hd::<32>(a1, a2)
    });
}