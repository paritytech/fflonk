use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
use ark_ff::UniformRand;
use ark_poly::DenseUVPolynomial;
use ark_std::test_rng;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use w3f_pcs::pcs::kzg::KZG;
use w3f_pcs::pcs::{PcsParams, PCS};
use w3f_pcs::Poly;

fn kzg_setup<E: Pairing>(c: &mut Criterion) {
    let curve = w3f_pcs::utils::curve_name::<E>();
    let mut group = c.benchmark_group(format!("{}/kzg-setup", curve));

    for log_n in [8, 10, 12] {
        let max_degree = (1 << log_n) - 1;
        group.bench_with_input(BenchmarkId::new("setup", format!("2^{}", log_n)), &max_degree, |b, &d| {
            b.iter_with_setup(
                || test_rng(),
                |mut rng| KZG::<E>::setup(d, &mut rng),
            )
        });
    }

    group.finish();
}

fn kzg_commit<E: Pairing>(c: &mut Criterion) {
    let curve = w3f_pcs::utils::curve_name::<E>();
    let mut group = c.benchmark_group(format!("{}/kzg-commit", curve));
    let rng = &mut test_rng();

    for log_n in [8, 10, 12] {
        let max_degree = (1 << log_n) - 1;
        let urs = KZG::<E>::setup(max_degree, rng);
        let ck = urs.ck();
        let poly = Poly::<E::ScalarField>::rand(max_degree, rng);

        group.bench_with_input(BenchmarkId::new("commit", format!("2^{}", log_n)), &log_n, |b, _| {
            b.iter(|| KZG::<E>::commit(&ck, &poly).unwrap())
        });
    }

    group.finish();
}

fn kzg_open<E: Pairing>(c: &mut Criterion) {
    let curve = w3f_pcs::utils::curve_name::<E>();
    let mut group = c.benchmark_group(format!("{}/kzg-open", curve));
    let rng = &mut test_rng();

    for log_n in [8, 10, 12] {
        let max_degree = (1 << log_n) - 1;
        let urs = KZG::<E>::setup(max_degree, rng);
        let ck = urs.ck();
        let poly = Poly::<E::ScalarField>::rand(max_degree, rng);
        let x = E::ScalarField::rand(rng);

        group.bench_with_input(BenchmarkId::new("open", format!("2^{}", log_n)), &log_n, |b, _| {
            b.iter(|| KZG::<E>::open(&ck, &poly, x).unwrap())
        });
    }

    group.finish();
}

fn kzg_verify<E: Pairing>(c: &mut Criterion) {
    let curve = w3f_pcs::utils::curve_name::<E>();
    let mut group = c.benchmark_group(format!("{}/kzg-verify", curve));
    let rng = &mut test_rng();

    for log_n in [8, 10] {
        let max_degree = (1 << log_n) - 1;
        let urs = KZG::<E>::setup(max_degree, rng);
        let ck = urs.ck();
        let vk = urs.vk();
        let poly = Poly::<E::ScalarField>::rand(max_degree, rng);
        let x = E::ScalarField::rand(rng);
        let y = ark_poly::Polynomial::evaluate(&poly, &x);
        let proof = KZG::<E>::open(&ck, &poly, x).unwrap();
        let commitment = KZG::<E>::commit(&ck, &poly).unwrap();

        group.bench_with_input(BenchmarkId::new("verify", format!("2^{}", log_n)), &log_n, |b, _| {
            b.iter(|| KZG::<E>::verify(&vk, commitment.clone(), x, y, proof.clone()).unwrap())
        });
    }

    group.finish();
}

fn kzg_batch_verify<E: Pairing>(c: &mut Criterion) {
    let curve = w3f_pcs::utils::curve_name::<E>();
    let mut group = c.benchmark_group(format!("{}/kzg-batch-verify", curve));
    let rng = &mut test_rng();

    let log_n = 10;
    let max_degree = (1 << log_n) - 1;
    let urs = KZG::<E>::setup(max_degree, rng);
    let ck = urs.ck();
    let vk = urs.vk();

    for k in [2, 4, 8] {
        let polys: Vec<_> = (0..k).map(|_| Poly::<E::ScalarField>::rand(max_degree, rng)).collect();
        let xs: Vec<_> = (0..k).map(|_| E::ScalarField::rand(rng)).collect();
        let ys: Vec<_> = polys.iter().zip(xs.iter()).map(|(p, x)| ark_poly::Polynomial::evaluate(p, x)).collect();
        let commitments: Vec<_> = polys.iter().map(|p| KZG::<E>::commit(&ck, p).unwrap()).collect();
        let proofs: Vec<_> = polys.iter().zip(xs.iter()).map(|(p, &x)| KZG::<E>::open(&ck, p, x).unwrap()).collect();

        group.bench_with_input(BenchmarkId::new("batch-verify", k), &k, |b, _| {
            b.iter_with_setup(
                || test_rng(),
                |mut rng| {
                    KZG::<E>::batch_verify(
                        &vk,
                        commitments.clone(),
                        xs.clone(),
                        ys.clone(),
                        proofs.clone(),
                        &mut rng,
                    )
                    .unwrap()
                },
            )
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    kzg_setup::<Bls12_381>,
    kzg_commit::<Bls12_381>,
    kzg_open::<Bls12_381>,
    kzg_verify::<Bls12_381>,
    kzg_batch_verify::<Bls12_381>,
);
criterion_main!(benches);
