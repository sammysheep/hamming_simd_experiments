# Some example benchmarks.

On an Intel Core i7 (I7-4870HQ) with no target specified.

```
test intrinsic_hd_lg           ... bench:         157 ns/iter (+/- 14)
test scalar_1b_hd_lg           ... bench:       1,376 ns/iter (+/- 102)
test scalar_hd_lg              ... bench:       3,519 ns/iter (+/- 226)
test simd_aligned_eq_hd16_lg   ... bench:         182 ns/iter (+/- 14)
test simd_aligned_eq_hd32_lg   ... bench:         171 ns/iter (+/- 8)
test simd_aligned_ne_hd16_lg   ... bench:         211 ns/iter (+/- 6)
test simd_aligned_ne_hd32_lg   ... bench:         195 ns/iter (+/- 23)
test simd_chunk_eq_hd16_lg     ... bench:         253 ns/iter (+/- 20)
test simd_chunk_eq_hd32_lg     ... bench:       1,189 ns/iter (+/- 139)
test simd_chunk_ne_hd16_lg     ... bench:         277 ns/iter (+/- 31)
test simd_chunk_ne_hd32_lg     ... bench:       1,234 ns/iter (+/- 138)
test simd_chunk_select_hd16_lg ... bench:         277 ns/iter (+/- 33)
test simd_chunk_select_hd32_lg ... bench:       1,238 ns/iter (+/- 137)
test simd_chunk_xor_hd16_lg    ... bench:         279 ns/iter (+/- 31)
test simd_chunk_xor_hd32_lg    ... bench:       1,236 ns/iter (+/- 121)
test simd_fold_ne_hd16_lg      ... bench:         561 ns/iter (+/- 39)
test simd_fold_ne_hd32_lg      ... bench:       1,221 ns/iter (+/- 142)
test simd_for_ne_hd16_lg       ... bench:       2,315 ns/iter (+/- 240)
test simd_for_ne_hd32_lg       ... bench:       1,388 ns/iter (+/- 155)
test simd_reduce_ne_hd16_lg    ... bench:       1,321 ns/iter (+/- 84)
test simd_reduce_ne_hd32_lg    ... bench:       1,743 ns/iter (+/- 151)
test simd_while_ne_hd16_lg     ... bench:       2,156 ns/iter (+/- 213)
test simd_while_ne_hd32_lg     ... bench:       1,355 ns/iter (+/- 182)
```

Targeting "Haswell" architecture (similar results seen targeting "x86-64-v3" on more recent equipment):

```
test intrinsic_hd_lg           ... bench:         150 ns/iter (+/- 4)
test scalar_1b_hd_lg           ... bench:         678 ns/iter (+/- 67)
test scalar_hd_lg              ... bench:       1,321 ns/iter (+/- 157)
test simd_aligned_eq_hd16_lg   ... bench:         202 ns/iter (+/- 15)
test simd_aligned_eq_hd32_lg   ... bench:         112 ns/iter (+/- 6)
test simd_aligned_ne_hd16_lg   ... bench:         204 ns/iter (+/- 29)
test simd_aligned_ne_hd32_lg   ... bench:         112 ns/iter (+/- 6)
test simd_chunk_eq_hd16_lg     ... bench:         260 ns/iter (+/- 18)
test simd_chunk_eq_hd32_lg     ... bench:       1,201 ns/iter (+/- 26)
test simd_chunk_ne_hd16_lg     ... bench:         251 ns/iter (+/- 28)
test simd_chunk_ne_hd32_lg     ... bench:       1,222 ns/iter (+/- 152)
test simd_chunk_select_hd16_lg ... bench:         252 ns/iter (+/- 29)
test simd_chunk_select_hd32_lg ... bench:       1,228 ns/iter (+/- 135)
test simd_chunk_xor_hd16_lg    ... bench:         250 ns/iter (+/- 31)
test simd_chunk_xor_hd32_lg    ... bench:       1,231 ns/iter (+/- 99)
test simd_fold_ne_hd16_lg      ... bench:         386 ns/iter (+/- 45)
test simd_fold_ne_hd32_lg      ... bench:       1,219 ns/iter (+/- 109)
test simd_for_ne_hd16_lg       ... bench:       1,173 ns/iter (+/- 130)
test simd_for_ne_hd32_lg       ... bench:       1,377 ns/iter (+/- 77)
test simd_reduce_ne_hd16_lg    ... bench:         457 ns/iter (+/- 52)
test simd_reduce_ne_hd32_lg    ... bench:       1,311 ns/iter (+/- 130)
test simd_while_ne_hd16_lg     ... bench:       2,233 ns/iter (+/- 272)
test simd_while_ne_hd32_lg     ... bench:       1,356 ns/iter (+/- 27)
```
