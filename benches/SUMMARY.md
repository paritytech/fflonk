# Benchmark Results

Measured with `cargo bench -- --quick`.

Curve: BLS12-381 (kzg, fflonk, shplonk, pipeline), BW6-761 (primitives, multiexps).

Machine: AMD Ryzen Threadripper 3970X (64 logical cores), 62 GiB RAM, Arch Linux 6.18.9, rustc 1.93.0.

## KZG (`benches/kzg.rs`)

| Benchmark | Time |
|---|---|
| setup 2^8 | 16.12 ms |
| setup 2^10 | 37.57 ms |
| setup 2^12 | 103.24 ms |
| commit 2^8 | 8.63 ms |
| commit 2^10 | 25.46 ms |
| commit 2^12 | 82.17 ms |
| open 2^8 | 8.63 ms |
| open 2^10 | 24.26 ms |
| open 2^12 | 77.29 ms |
| verify 2^8 | 1.66 ms |
| verify 2^10 | 1.98 ms |
| batch-verify k=2 | 1.95 ms |
| batch-verify k=4 | 2.48 ms |
| batch-verify k=8 | 3.53 ms |

## FFLonk (`benches/fflonk.rs`)

| Benchmark | Time |
|---|---|
| combine t=4, d=63 | 311.09 ns |
| combine t=4, d=255 | 1.53 us |
| combine t=4, d=1023 | 5.71 us |
| combine t=8, d=63 | 629.39 ns |
| combine t=8, d=255 | 3.07 us |
| roots t=2 | 916.75 ns |
| roots t=4 | 925.19 ns |
| roots t=8 | 964.00 ns |
| roots t=16 | 1.09 us |
| opening_as_points t=4 | 1.71 us |
| opening_as_points t=8 | 4.03 us |
| multiopening t=4, m=2 | 4.69 us |
| multiopening t=4, m=4 | 9.21 us |
| multiopening t=8, m=2 | 6.70 us |

## SHPlonk (`benches/shplonk.rs`)

| Benchmark | Time |
|---|---|
| open t=4, d=255, m=2 | 17.58 ms |
| open t=4, d=1023, m=2 | 51.89 ms |
| open t=8, d=255, m=2 | 16.46 ms |
| open t=4, d=255, m=4 | 16.30 ms |
| verify t=4, d=255, m=2 | 1.89 ms |
| verify t=4, d=1023, m=2 | 2.01 ms |
| verify t=8, d=255, m=2 | 2.38 ms |

## FflonkyKzg Pipeline (`benches/pipeline.rs`)

| Benchmark | Time |
|---|---|
| open t=4, d=63, m=1 | 17.35 ms |
| open t=4, d=63, m=2 | 17.52 ms |
| open t=4, d=255, m=1 | 51.53 ms |
| open t=8, d=63, m=1 | 30.13 ms |
| verify t=4, d=63, m=1 | 1.75 ms |
| verify t=4, d=63, m=2 | 1.75 ms |
| verify t=4, d=255, m=1 | 1.75 ms |
| verify t=8, d=63, m=1 | 1.74 ms |
| open+verify t=4, d=255, m=2 | 53.19 ms |

## EC Primitives (`benches/primitives.rs`, BW6-761)

| Benchmark | Time |
|---|---|
| scalar-mul projective | 1.09 ms |
| scalar-mul affine | 938.20 us |
| into affine | 39.55 us |
| into projective | 7.91 ns |
| addition projective | 2.97 us |
| addition affine | 2.06 us |
| addition mixed | 2.07 us |
| doubling | 1.31 us |

## Multi-Exponentiation (`benches/multiexps.rs`, BW6-761)

| Benchmark | Time |
|---|---|
| small-multiexp-affine full n=10 | 4.46 ms |
| naive-multiexp-affine full n=10 | 9.32 ms |
| small-multiexp-affine 128-bit n=10 | 1.49 ms |
| naive-multiexp-affine 128-bit n=10 | 3.13 ms |
| small-multiexp-proj in_affine n=10 | 1.51 ms |
| small-multiexp-proj in_proj n=10 | 1.96 ms |
| small-multiexp full n=10 vs msm | 4.42 ms vs 4.30 ms |
| small-multiexp full n=20 vs msm | 8.58 ms vs 6.77 ms |
| small-multiexp 128-bit n=10 vs msm | 1.49 ms vs 1.50 ms |
| small-multiexp 128-bit n=20 vs msm | 2.89 ms vs 2.33 ms |
