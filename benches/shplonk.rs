use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
use ark_ff::UniformRand;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_std::collections::BTreeSet;
use ark_std::iter::FromIterator;
use ark_std::test_rng;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use w3f_pcs::pcs::kzg::KZG;
use w3f_pcs::pcs::{PcsParams, PCS};
use w3f_pcs::shplonk::Shplonk;
use w3f_pcs::Poly;

type F = <Bls12_381 as Pairing>::ScalarField;
type CS = KZG<Bls12_381>;

struct TestData {
    fs: Vec<Poly<F>>,
    fcs: Vec<<CS as PCS<F>>::C>,
    xss_btree: Vec<BTreeSet<F>>,
    xss_vec: Vec<Vec<F>>,
    yss: Vec<Vec<F>>,
}

fn generate_test_data(
    d: usize,
    t: usize,
    m: usize,
) -> (TestData, <CS as PCS<F>>::CK, <CS as PCS<F>>::VK) {
    let rng = &mut test_rng();
    let params = CS::setup(d, rng);
    let ck = params.ck();
    let vk = params.vk();

    let fs: Vec<Poly<F>> = (0..t).map(|_| Poly::rand(d, rng)).collect();
    let fcs: Vec<_> = fs.iter().map(|f| CS::commit(&ck, f).unwrap()).collect();
    let xss_vec: Vec<Vec<F>> = (0..t)
        .map(|_| (0..m).map(|_| F::rand(rng)).collect())
        .collect();
    let xss_btree: Vec<BTreeSet<F>> = xss_vec
        .iter()
        .map(|xs| BTreeSet::from_iter(xs.iter().cloned()))
        .collect();
    let yss: Vec<Vec<F>> = fs
        .iter()
        .zip(xss_vec.iter())
        .map(|(f, xs)| xs.iter().map(|x| f.evaluate(x)).collect())
        .collect();

    let data = TestData {
        fs,
        fcs,
        xss_btree,
        xss_vec,
        yss,
    };
    (data, ck, vk)
}

fn new_transcript() -> merlin::Transcript {
    merlin::Transcript::new(b"bench-shplonk")
}

fn shplonk_open(c: &mut Criterion) {
    let mut group = c.benchmark_group("shplonk-open");

    for (t, d, m) in [(4, 255, 2), (4, 1023, 2), (8, 255, 2), (4, 255, 4)] {
        let (data, ck, _) = generate_test_data(d, t, m);
        let label = format!("t={},d={},m={}", t, d, m);

        group.bench_with_input(BenchmarkId::new("open", &label), &label, |b, _| {
            b.iter_with_setup(new_transcript, |mut transcript| {
                Shplonk::<F, CS>::open_many(&ck, &data.fs, &data.xss_btree, &mut transcript)
            })
        });
    }

    group.finish();
}

fn shplonk_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("shplonk-verify");

    for (t, d, m) in [(4, 255, 2), (4, 1023, 2), (8, 255, 2)] {
        let (data, ck, vk) = generate_test_data(d, t, m);
        let label = format!("t={},d={},m={}", t, d, m);

        let mut transcript = new_transcript();
        let proof = Shplonk::<F, CS>::open_many(&ck, &data.fs, &data.xss_btree, &mut transcript);

        group.bench_with_input(BenchmarkId::new("verify", &label), &label, |b, _| {
            b.iter_with_setup(
                || (new_transcript(), proof.clone()),
                |(mut transcript, proof)| {
                    Shplonk::<F, CS>::verify_many(
                        &vk,
                        &data.fcs,
                        proof,
                        &data.xss_vec,
                        &data.yss,
                        &mut transcript,
                    )
                },
            )
        });
    }

    group.finish();
}

criterion_group!(benches, shplonk_open, shplonk_verify,);
criterion_main!(benches);
