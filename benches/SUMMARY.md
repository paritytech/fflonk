# Benchmark Results

Measured with `cargo bench -- --quick`.

Curve: BLS12-381 (kzg, fflonk, shplonk, pipeline), BW6-761 (primitives, multiexps).

Machine: AMD Ryzen Threadripper 3970X (64 logical cores), 62 GiB RAM, Arch Linux 6.18.9, rustc 1.93.0.

## KZG (`benches/kzg.rs`)

| Benchmark | Time |
|---|---|
| setup 2^8 | 16.09 ms |
| setup 2^10 | 38.54 ms |
| setup 2^12 | 104.74 ms |
| commit 2^8 | 8.67 ms |
| commit 2^10 | 25.88 ms |
| commit 2^12 | 82.64 ms |
| open 2^8 | 8.81 ms |
| open 2^10 | 23.91 ms |
| open 2^12 | 84.11 ms |
| verify 2^8 | 1.72 ms |
| verify 2^10 | 1.70 ms |
| batch-verify k=2 | 2.00 ms |
| batch-verify k=4 | 2.67 ms |
| batch-verify k=8 | 3.80 ms |

## FFLonk (`benches/fflonk.rs`)

| Benchmark | Time |
|---|---|
| combine t=4, d=63 | 315.75 ns |
| combine t=4, d=255 | 1.72 us |
| combine t=4, d=1023 | 6.69 us |
| combine t=8, d=63 | 618.91 ns |
| combine t=8, d=255 | 3.53 us |
| roots t=2 | 918.16 ns |
| roots t=4 | 918.60 ns |
| roots t=8 | 977.64 ns |
| roots t=16 | 1.10 us |
| opening_as_points t=4 | 1.71 us |
| opening_as_points t=8 | 3.98 us |
| multiopening t=4, m=2 | 4.66 us |
| multiopening t=4, m=4 | 9.72 us |
| multiopening t=8, m=2 | 6.80 us |

## SHPlonk (`benches/shplonk.rs`)

| Benchmark | Time |
|---|---|
| open t=4, d=255, m=2 | 18.02 ms |
| open t=4, d=1023, m=2 | 59.28 ms |
| open t=8, d=255, m=2 | 20.58 ms |
| open t=4, d=255, m=4 | 21.40 ms |
| verify t=4, d=255, m=2 | 2.42 ms |
| verify t=4, d=1023, m=2 | 2.08 ms |
| verify t=8, d=255, m=2 | 2.81 ms |

## FflonkyKzg Pipeline (`benches/pipeline.rs`)

| Benchmark | Time |
|---|---|
| open t=4, d=63, m=1 | 15.94 ms |
| open t=4, d=63, m=2 | 15.96 ms |
| open t=4, d=255, m=1 | 54.39 ms |
| open t=8, d=63, m=1 | 37.37 ms |
| verify t=4, d=63, m=1 | 2.07 ms |
| verify t=4, d=63, m=2 | 1.90 ms |
| verify t=4, d=255, m=1 | 1.69 ms |
| verify t=8, d=63, m=1 | 2.01 ms |
| open+verify t=4, d=255, m=2 | 57.14 ms |

## EC Primitives (`benches/primitives.rs`, BW6-761)

| Benchmark | Time |
|---|---|
| scalar-mul projective | 1.13 ms |
| scalar-mul affine | 950.04 us |
| into affine | 37.01 us |
| into projective | 9.10 ns |
| addition projective | 3.45 us |
| addition affine | 2.32 us |
| addition mixed | 2.60 us |
| doubling | 1.68 us |

## Multi-Exponentiation (`benches/multiexps.rs`, BW6-761)

| Benchmark | Time |
|---|---|
| small-multiexp-affine full n=10 | 5.86 ms |
| naive-multiexp-affine full n=10 | 10.35 ms |
| small-multiexp-affine 128-bit n=10 | 1.95 ms |
| naive-multiexp-affine 128-bit n=10 | 3.97 ms |
| small-multiexp-proj in_affine n=10 | 1.48 ms |
| small-multiexp-proj in_proj n=10 | 1.87 ms |
| small-multiexp full n=10 vs msm | 4.32 ms vs 4.25 ms |
| small-multiexp full n=20 vs msm | 9.94 ms vs 6.96 ms |
| small-multiexp 128-bit n=10 vs msm | 1.45 ms vs 1.50 ms |
| small-multiexp 128-bit n=20 vs msm | 2.82 ms vs 2.52 ms |
