# Some example benchmarks.

Anecdotal benchmark on an Intel Core i7 (I7-4870HQ) running MacOS 11.6.8; we used rustc 1.65.0-nightly (060e47f74 2022-08-23) with no target specified. A subet of benches were done on Criterion and then all tests were done using Bencher.

```
     Running benches/criterion.rs (target/release/deps/criterion-f76d36f5f1c350fb)

Gnuplot not found, using plotters backend
scalar_hd_lg            time:   [3.5466 us 3.5573 us 3.5694 us]                          
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe

intrinsic_hd_lg         time:   [152.99 ns 153.53 ns 154.11 ns]                            
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

simd_chunk_ne_hd16_lg   time:   [269.34 ns 270.32 ns 271.41 ns]                                  
Found 18 outliers among 100 measurements (18.00%)
  7 (7.00%) high mild
  11 (11.00%) high severe

simd_chunk_ne_hd32_lg   time:   [1.2647 us 1.2864 us 1.3144 us]                                   
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

simd_aligned_ne_hd16_lg time:   [214.11 ns 215.13 ns 216.32 ns]                                    
Found 12 outliers among 100 measurements (12.00%)
  12 (12.00%) high mild

simd_aligned_ne_hd32_lg time:   [200.37 ns 202.22 ns 204.26 ns]                                    
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe

     Running benches/hd.rs (target/release/deps/hd-7f0151760682294c)

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
     Running benches/criterion.rs (target/release/deps/criterion-f76d36f5f1c350fb)

Gnuplot not found, using plotters backend
scalar_hd_lg            time:   [1.2909 us 1.3046 us 1.3191 us]                          
                        change: [-63.856% -63.555% -63.219%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild

intrinsic_hd_lg         time:   [151.89 ns 152.49 ns 153.22 ns]                            
                        change: [-0.1129% +1.0657% +2.2822%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe

simd_chunk_ne_hd16_lg   time:   [255.24 ns 258.42 ns 262.09 ns]                                  
                        change: [-5.7034% -4.7691% -3.7817%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

simd_chunk_ne_hd32_lg   time:   [1.2782 us 1.2927 us 1.3085 us]                                   
                        change: [+0.2555% +1.8604% +3.3418%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

simd_aligned_ne_hd16_lg time:   [206.11 ns 207.20 ns 208.43 ns]                                    
                        change: [-4.8481% -3.9852% -3.1024%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe

simd_aligned_ne_hd32_lg time:   [115.30 ns 116.43 ns 117.73 ns]                                    
                        change: [-42.603% -41.834% -41.076%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  12 (12.00%) high mild
  2 (2.00%) high severe

     Running benches/hd.rs (target/release/deps/hd-7f0151760682294c)

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
