
use criterion::*;
use std::time::Duration;
use triple_accel::hamming::hamming as intrinsic_hd;
use hamming::*;

pub fn scalar_hd_lg(c: &mut Criterion) {
    c.bench_function("scalar", |b| b.iter(|| scalar_hamming(L1, L2)));
}

pub fn intrinsic_hd_lg(c: &mut Criterion) {
    c.bench_function("intrinsic", |b| b.iter(|| intrinsic_hd(L1, L2)));
}

pub fn simd_chunk_ne_hd16_lg(c: &mut Criterion) {
    c.bench_function("chunk16", |b| b.iter(|| simd_chunk_ne_hd::<16>(L1, L2)));
}

pub fn simd_chunk_ne_hd32_lg(c: &mut Criterion) {
    c.bench_function("chunk32", |b| b.iter(|| simd_chunk_ne_hd::<32>(L1, L2)));
}

pub fn simd_aligned_ne_hd16_lg(c: &mut Criterion) {
    let a1 = AlignedVec(L1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(L2.to_vec());
    let a2 = a2.get_slice();
    c.bench_function("aligned16", |b| b.iter(|| simd_aligned_ne_hd::<16>(a1, a2)));
}

pub fn simd_aligned_ne_hd32_lg(c: &mut Criterion) {
    let a1 = AlignedVec(L1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(L2.to_vec());
    let a2 = a2.get_slice();

    c.bench_function("aligned32",|b| b.iter(|| simd_aligned_ne_hd::<32>(a1, a2)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(Duration::new(1,0));
    targets = scalar_hd_lg, intrinsic_hd_lg, simd_chunk_ne_hd16_lg, simd_chunk_ne_hd32_lg, simd_aligned_ne_hd16_lg, simd_aligned_ne_hd32_lg
);
criterion_main!(benches);