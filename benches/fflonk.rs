use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
use ark_ff::UniformRand;
use ark_poly::DenseUVPolynomial;
use ark_std::test_rng;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use w3f_pcs::fflonk::Fflonk;
use w3f_pcs::Poly;

type F = <Bls12_381 as Pairing>::ScalarField;
type P = Poly<F>;
type FflonkBls = Fflonk<F, P>;

fn fflonk_combine(c: &mut Criterion) {
    let mut group = c.benchmark_group("fflonk-combine");
    let rng = &mut test_rng();

    // Vary t (number of polynomials) and d (degree)
    for (t, d) in [(4, 63), (4, 255), (4, 1023), (8, 63), (8, 255)] {
        let fs: Vec<P> = (0..t).map(|_| P::rand(d, rng)).collect();
        let label = format!("t={},d={}", t, d);

        group.bench_with_input(BenchmarkId::new("combine", &label), &label, |b, _| {
            b.iter(|| FflonkBls::combine(t, &fs))
        });
    }

    group.finish();
}

fn fflonk_roots(c: &mut Criterion) {
    let mut group = c.benchmark_group("fflonk-roots");
    let rng = &mut test_rng();

    for t in [2, 4, 8, 16] {
        let root = F::rand(rng);

        group.bench_with_input(BenchmarkId::new("roots", t), &t, |b, &t| {
            b.iter(|| FflonkBls::roots(t, root))
        });
    }

    group.finish();
}

fn fflonk_opening_as_points(c: &mut Criterion) {
    let mut group = c.benchmark_group("fflonk-opening-as-points");
    let rng = &mut test_rng();

    for t in [4, 8] {
        let root = F::rand(rng);
        let evals: Vec<F> = (0..t).map(|_| F::rand(rng)).collect();

        group.bench_with_input(BenchmarkId::new("as-points", t), &t, |b, _| {
            b.iter(|| FflonkBls::opening_as_points(t, root, &evals))
        });
    }

    group.finish();
}

fn fflonk_multiopening(c: &mut Criterion) {
    let mut group = c.benchmark_group("fflonk-multiopening");
    let rng = &mut test_rng();

    for (t, m) in [(4, 2), (4, 4), (8, 2)] {
        let roots: Vec<F> = (0..m).map(|_| F::rand(rng)).collect();
        let evals: Vec<Vec<F>> = (0..m)
            .map(|_| (0..t).map(|_| F::rand(rng)).collect())
            .collect();
        let label = format!("t={},m={}", t, m);

        group.bench_with_input(BenchmarkId::new("multiopening", &label), &label, |b, _| {
            b.iter(|| FflonkBls::multiopening(t, &roots, &evals))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    fflonk_combine,
    fflonk_roots,
    fflonk_opening_as_points,
    fflonk_multiopening,
);
criterion_main!(benches);
