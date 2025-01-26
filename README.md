# Some example benchmarks.

## arm64

Anecdotal benchmark on an M4 Max running MacOS 15.2 and using rustc 1.86.0-nightly (48a426eca 2025-01-12).

```
running 27 tests
test intrinsic2_hd_lg           ... bench:         177.77 ns/iter (+/- 12.65)
test intrinsic_hd_lg            ... bench:         519.88 ns/iter (+/- 4.71)
test scalar_1b_hd_lg            ... bench:         521.44 ns/iter (+/- 76.62)
test scalar_hd_lg               ... bench:       1,049.39 ns/iter (+/- 23.20)
test simd_aligned_eq_hd16_lg    ... bench:         201.66 ns/iter (+/- 3.59)
test simd_aligned_eq_hd32_lg    ... bench:         124.62 ns/iter (+/- 4.36)
test simd_aligned_eq_hd64_lg    ... bench:         105.32 ns/iter (+/- 6.36)
test simd_aligned_ne_hd16_lg    ... bench:         206.80 ns/iter (+/- 3.78)
test simd_aligned_ne_hd32_lg    ... bench:         131.18 ns/iter (+/- 10.65)
test simd_chunk_bitmask_hd16_lg ... bench:         317.46 ns/iter (+/- 18.80)
test simd_chunk_bitmask_hd32_lg ... bench:         260.78 ns/iter (+/- 15.62)
test simd_chunk_eq_hd16_lg      ... bench:         202.00 ns/iter (+/- 7.92)
test simd_chunk_eq_hd32_lg      ... bench:         124.85 ns/iter (+/- 4.10)
test simd_chunk_ne_hd16_lg      ... bench:         204.53 ns/iter (+/- 2.83)
test simd_chunk_ne_hd32_lg      ... bench:         131.93 ns/iter (+/- 8.36)
test simd_chunk_select_hd16_lg  ... bench:         206.18 ns/iter (+/- 3.35)
test simd_chunk_select_hd32_lg  ... bench:         132.33 ns/iter (+/- 8.73)
test simd_chunk_xor_hd16_lg     ... bench:         205.12 ns/iter (+/- 3.01)
test simd_chunk_xor_hd32_lg     ... bench:         130.54 ns/iter (+/- 9.21)
test simd_fold_ne_hd16_lg       ... bench:         196.27 ns/iter (+/- 2.84)
test simd_fold_ne_hd32_lg       ... bench:         133.03 ns/iter (+/- 7.87)
test simd_for_ne_hd16_lg        ... bench:         301.71 ns/iter (+/- 12.50)
test simd_for_ne_hd32_lg        ... bench:         186.86 ns/iter (+/- 9.82)
test simd_reduce_ne_hd16_lg     ... bench:         310.61 ns/iter (+/- 12.30)
test simd_reduce_ne_hd32_lg     ... bench:         258.99 ns/iter (+/- 11.95)
test simd_while_ne_hd16_lg      ... bench:         211.52 ns/iter (+/- 4.09)
test simd_while_ne_hd32_lg      ... bench:         154.79 ns/iter (+/- 10.68)
```

## x86-64

Anecdotal benchmark on an Intel Core i7 (I7-4870HQ) running MacOS 11.6.8; we used rustc 1.65.0-nightly (060e47f74 2022-08-23) with no target specified.

```
running 23 tests
test intrinsic_hd_lg           ... bench:         164 ns/iter (+/- 17)
test scalar_1b_hd_lg           ... bench:       1,441 ns/iter (+/- 220)
test scalar_hd_lg              ... bench:       3,543 ns/iter (+/- 230)
test simd_aligned_eq_hd16_lg   ... bench:         174 ns/iter (+/- 4)
test simd_aligned_eq_hd32_lg   ... bench:         173 ns/iter (+/- 9)
test simd_aligned_ne_hd16_lg   ... bench:         199 ns/iter (+/- 24)
test simd_aligned_ne_hd32_lg   ... bench:         194 ns/iter (+/- 8)
test simd_chunk_eq_hd16_lg     ... bench:         254 ns/iter (+/- 38)
test simd_chunk_eq_hd32_lg     ... bench:       1,218 ns/iter (+/- 267)
test simd_chunk_ne_hd16_lg     ... bench:         267 ns/iter (+/- 29)
test simd_chunk_ne_hd32_lg     ... bench:       1,244 ns/iter (+/- 145)
test simd_chunk_select_hd16_lg ... bench:         276 ns/iter (+/- 30)
test simd_chunk_select_hd32_lg ... bench:       1,244 ns/iter (+/- 136)
test simd_chunk_xor_hd16_lg    ... bench:         266 ns/iter (+/- 18)
test simd_chunk_xor_hd32_lg    ... bench:       1,248 ns/iter (+/- 168)
test simd_fold_ne_hd16_lg      ... bench:         416 ns/iter (+/- 47)
test simd_fold_ne_hd32_lg      ... bench:       1,235 ns/iter (+/- 124)
test simd_for_ne_hd16_lg       ... bench:       2,317 ns/iter (+/- 164)
test simd_for_ne_hd32_lg       ... bench:       1,416 ns/iter (+/- 116)
test simd_reduce_ne_hd16_lg    ... bench:       1,317 ns/iter (+/- 84)
test simd_reduce_ne_hd32_lg    ... bench:       1,810 ns/iter (+/- 187)
test simd_while_ne_hd16_lg     ... bench:       2,175 ns/iter (+/- 250)
test simd_while_ne_hd32_lg     ... bench:       1,285 ns/iter (+/- 145)
```

Targeting "Haswell" architecture (similar results seen targeting "x86-64-v3" on more recent equipment):
```
running 23 tests
test intrinsic_hd_lg           ... bench:         176 ns/iter (+/- 19)
test scalar_1b_hd_lg           ... bench:         692 ns/iter (+/- 103)
test scalar_hd_lg              ... bench:       1,263 ns/iter (+/- 86)
test simd_aligned_eq_hd16_lg   ... bench:         203 ns/iter (+/- 6)
test simd_aligned_eq_hd32_lg   ... bench:         111 ns/iter (+/- 2)
test simd_aligned_ne_hd16_lg   ... bench:         202 ns/iter (+/- 18)
test simd_aligned_ne_hd32_lg   ... bench:         112 ns/iter (+/- 3)
test simd_chunk_eq_hd16_lg     ... bench:         252 ns/iter (+/- 32)
test simd_chunk_eq_hd32_lg     ... bench:       1,263 ns/iter (+/- 30)
test simd_chunk_ne_hd16_lg     ... bench:         255 ns/iter (+/- 36)
test simd_chunk_ne_hd32_lg     ... bench:       1,255 ns/iter (+/- 182)
test simd_chunk_select_hd16_lg ... bench:         257 ns/iter (+/- 34)
test simd_chunk_select_hd32_lg ... bench:       1,334 ns/iter (+/- 149)
test simd_chunk_xor_hd16_lg    ... bench:         258 ns/iter (+/- 23)
test simd_chunk_xor_hd32_lg    ... bench:       1,233 ns/iter (+/- 101)
test simd_fold_ne_hd16_lg      ... bench:         389 ns/iter (+/- 48)
test simd_fold_ne_hd32_lg      ... bench:       1,231 ns/iter (+/- 71)
test simd_for_ne_hd16_lg       ... bench:       1,155 ns/iter (+/- 132)
test simd_for_ne_hd32_lg       ... bench:       1,376 ns/iter (+/- 85)
test simd_reduce_ne_hd16_lg    ... bench:         470 ns/iter (+/- 59)
test simd_reduce_ne_hd32_lg    ... bench:       1,370 ns/iter (+/- 110)
test simd_while_ne_hd16_lg     ... bench:       1,916 ns/iter (+/- 101)
test simd_while_ne_hd32_lg     ... bench:       1,450 ns/iter (+/- 163)
```
