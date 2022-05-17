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

    let a1 = AlignedVec(S1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(S2.to_vec());
    let a2 = a2.get_slice();

    let d = simd_aligned_ne_hd::<16>(a1, a2);
    println!("simd_aligned_ne_hd::<16> {d} of {n}");

    let d = simd_aligned_eq_hd::<16>(a1, a2);
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

    let a1 = AlignedVec(L1.to_vec());
    let a1 = a1.get_slice();

    let a2 = AlignedVec(L2.to_vec());
    let a2 = a2.get_slice();

    let d = simd_aligned_ne_hd::<32>(a1, a2);
    println!("simd_aligned_ne_hd::<32> {d} of {n}");

    let d = simd_aligned_eq_hd::<32>(a1, a2);
    println!("simd_aligned_eq_hd::<32> {d} of {n}");
}
