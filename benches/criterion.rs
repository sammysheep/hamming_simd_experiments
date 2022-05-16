use criterion::{criterion_group, criterion_main, Criterion};

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

criterion_group!(benches, scalar_hd_lg, intrinsic_hd_lg, simd_chunk_ne_hd16_lg, simd_chunk_ne_hd32_lg);
criterion_main!(benches);