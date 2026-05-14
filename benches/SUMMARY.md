# Benchmark Results

Measured with `cargo bench -- --quick`.

Curve: BLS12-381 (kzg, fflonk, shplonk, pipeline), BW6-761 (primitives, multiexps).

Machine: AMD Ryzen Threadripper 3970X (64 logical cores), 62 GiB RAM, Arch Linux 7.0.5, rustc 1.95.0.

## KZG (`benches/kzg.rs`)

| Benchmark | Time |
|---|---|
| setup 2^8 | 18.06 ms |
| setup 2^10 | 35.40 ms |
| setup 2^12 | 113.69 ms |
| commit 2^8 | 7.16 ms |
| commit 2^10 | 24.82 ms |
| commit 2^12 | 63.75 ms |
| open 2^8 | 7.19 ms |
| open 2^10 | 19.77 ms |
| open 2^12 | 64.22 ms |
| verify 2^8 | 1.51 ms |
| verify 2^10 | 1.49 ms |
| batch-verify k=2 | 1.84 ms |
| batch-verify k=4 | 2.32 ms |
| batch-verify k=8 | 3.30 ms |

## FFLonk (`benches/fflonk.rs`)

| Benchmark | Time |
|---|---|
| combine t=4, d=63 | 367.11 ns |
| combine t=4, d=255 | 1.69 us |
| combine t=4, d=1023 | 7.88 us |
| combine t=8, d=63 | 708.61 ns |
| combine t=8, d=255 | 4.15 us |
| roots t=2 | 832.91 ns |
| roots t=4 | 849.34 ns |
| roots t=8 | 906.33 ns |
| roots t=16 | 1.01 us |
| opening_as_points t=4 | 1.58 us |
| opening_as_points t=8 | 3.70 us |
| multiopening t=4, m=2 | 4.31 us |
| multiopening t=4, m=4 | 8.59 us |
| multiopening t=8, m=2 | 6.24 us |

## SHPlonk (`benches/shplonk.rs`)

| Benchmark | Time |
|---|---|
| open t=4, d=255, m=2 | 13.49 ms |
| open t=4, d=1023, m=2 | 40.25 ms |
| open t=8, d=255, m=2 | 13.77 ms |
| open t=4, d=255, m=4 | 13.67 ms |
| verify t=4, d=255, m=2 | 1.76 ms |
| verify t=4, d=1023, m=2 | 1.97 ms |
| verify t=8, d=255, m=2 | 2.06 ms |

## FflonkyKzg Pipeline (`benches/pipeline.rs`)

| Benchmark | Time |
|---|---|
| open t=4, d=63, m=1 | 13.42 ms |
| open t=4, d=63, m=2 | 14.38 ms |
| open t=4, d=255, m=1 | 39.89 ms |
| open t=8, d=63, m=1 | 23.50 ms |
| verify t=4, d=63, m=1 | 1.52 ms |
| verify t=4, d=63, m=2 | 1.53 ms |
| verify t=4, d=255, m=1 | 1.52 ms |
| verify t=8, d=63, m=1 | 1.52 ms |
| open+verify t=4, d=255, m=2 | 42.60 ms |

## EC Primitives (`benches/primitives.rs`, BW6-761)

| Benchmark | Time |
|---|---|
| scalar-mul projective | 983.89 us |
| scalar-mul affine | 913.59 us |
| into affine | 38.94 us |
| into projective | 7.73 ns |
| addition projective | 3.11 us |
| addition affine | 2.36 us |
| addition mixed | 1.97 us |
| doubling | 1.57 us |

## Multi-Exponentiation (`benches/multiexps.rs`, BW6-761)

| Benchmark | Time |
|---|---|
| small-multiexp-affine full n=10 | 4.02 ms |
| naive-multiexp-affine full n=10 | 8.40 ms |
| small-multiexp-affine 128-bit n=10 | 1.36 ms |
| naive-multiexp-affine 128-bit n=10 | 2.83 ms |
| small-multiexp-proj in_affine n=10 | 1.38 ms |
| small-multiexp-proj in_proj n=10 | 1.83 ms |
| small-multiexp full n=10 vs msm | 4.20 ms vs 3.63 ms |
| small-multiexp full n=20 vs msm | 8.38 ms vs 5.63 ms |
| small-multiexp 128-bit n=10 vs msm | 1.36 ms vs 1.27 ms |
| small-multiexp 128-bit n=20 vs msm | 2.93 ms vs 2.37 ms |
