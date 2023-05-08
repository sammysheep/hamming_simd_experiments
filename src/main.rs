#![feature(portable_simd)]
//use std::simd::*;
//use std::cmp::min;
use hamming::*;

fn main() {
    let n = S1.len();
    let d = scalar_hamming(S1, S2);
    println!("Scalar: {d} of {n}");

    let d = scalar_hamming1b(S1, S2);
    println!("Scalar 1b: {d} of {n}");

    let d = simd_chunk_ne_hd::<32>(S1, S2);
    println!("simd_chunk_ne_hd::<32>: {d} of {n}");

    let d = simd_chunk_ne_hd::<16>(S1, S2);
    println!("simd_chunk_ne_hd::<16>: {d} of {n}");

    let d = simd_reduce_ne_hd::<16>(S1, S2);
    println!("simd_reduce_ne_hd::<16>: {d} of {n}");

    let d = simd_chunk_eq_hd::<32>(S1, S2);
    println!("simd_chunk_eq_hd::<32>: {d} of {n}");

    let d = simd_chunk_eq_hd::<16>(S1, S2);
    println!("simd_chunk_eq_hd::<16>: {d} of {n}");

    let d = simd_aligned_ne_hd::<16>(S1, S2);
    println!("simd_aligned_ne_hd::<16> {d} of {n}");

    let d = simd_aligned_eq_hd::<16>(S1, S2);
    println!("simd_aligned_eq_hd::<16> {d} of {n}");

    let n = L1.len();
    let d = scalar_hamming(L1, L2);
    println!("\nScalar: {d} of {n}");

    let d = scalar_hamming1b(L1, L2);
    println!("Scalar 1b: {d} of {n}");

    let d = simd_chunk_ne_hd::<32>(L1, L2);
    println!("simd_chunk_ne_hd::<32>: {d} of {n}");

    let d = simd_chunk_ne_hd::<16>(L1, L2);
    println!("simd_chunk_ne_hd::<16>: {d} of {n}");

    let d = simd_fold_ne_hd::<16>(L1, L2);
    println!("simd_fold_ne_hd::<16>: {d} of {n}");

    let d = simd_reduce_ne_hd::<32>(L1, L2);
    println!("simd_reduce_ne_hd::<32>: {d} of {n}");

    let d = simd_chunk_eq_hd::<32>(L1, L2);
    println!("simd_chunk_eq_hd::<32>: {d} of {n}");

    let d = simd_chunk_eq_hd::<16>(L1, L2);
    println!("simd_chunk_eq_hd::<16>: {d} of {n}");

    let d = simd_chunk_xor_hd::<16>(L1, L2);
    println!("simd_chunk_xor_hd::<16>: {d} of {n}");

    let d = simd_chunk_xor_hd::<32>(L1, L2);
    println!("simd_chunk_xor_hd::<32>: {d} of {n}");

    let d = simd_chunk_select_hd::<32>(L1, L2);
    println!("simd_chunk_select_hd::<32>: {d} of {n}");

    let d = simd_for_ne_hd::<32>(L1, L2);
    println!("simd_for_ne_hd::<32>: {d} of {n}");

    let d = simd_while_ne_hd::<32>(L1, L2);
    println!("simd_while_ne_hd::<32>: {d} of {n}");

    let d = simd_aligned_ne_hd::<32>(L1, L2);
    println!("simd_aligned_ne_hd::<32> {d} of {n}");

    let d = simd_aligned_eq_hd::<32>(L1, L2);
    println!("simd_aligned_eq_hd::<32> {d} of {n}");
}
