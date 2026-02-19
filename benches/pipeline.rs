use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
use ark_ff::{Field, UniformRand};
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_std::test_rng;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use w3f_pcs::fflonk::Fflonk;
use w3f_pcs::pcs::kzg::KZG;
use w3f_pcs::pcs::{PcsParams, PCS};
use w3f_pcs::FflonkyKzg;
use w3f_pcs::Poly;

type F = <Bls12_381 as Pairing>::ScalarField;
type CS = KZG<Bls12_381>;

struct PipelineData {
    fs: Vec<Poly<F>>,
    roots: Vec<F>,
    vss: Vec<Vec<F>>,
    gc: <CS as PCS<F>>::C,
}

fn setup_pipeline(
    d: usize,
    t: usize,
    m: usize,
) -> (PipelineData, <CS as PCS<F>>::CK, <CS as PCS<F>>::VK) {
    let rng = &mut test_rng();
    let params = CS::setup(t * (d + 1) - 1, rng);
    let ck = params.ck();
    let vk = params.vk();

    let fs: Vec<Poly<F>> = (0..t).map(|_| Poly::rand(d, rng)).collect();
    let roots: Vec<F> = (0..m).map(|_| F::rand(rng)).collect();
    let xs: Vec<F> = roots.iter().map(|root| root.pow([t as u64])).collect();
    let vss: Vec<Vec<F>> = xs
        .iter()
        .map(|x| fs.iter().map(|f| f.evaluate(x)).collect())
        .collect();
    let g = Fflonk::combine(t, &fs);
    let gc = CS::commit(&ck, &g).unwrap();

    let data = PipelineData {
        fs,
        roots,
        vss,
        gc,
    };
    (data, ck, vk)
}

fn new_transcript() -> merlin::Transcript {
    merlin::Transcript::new(b"bench-fflonky-kzg")
}

fn fflonky_kzg_open(c: &mut Criterion) {
    let mut group = c.benchmark_group("fflonky-kzg-open");

    for (t, d, m) in [(4, 63, 1), (4, 63, 2), (4, 255, 1), (8, 63, 1)] {
        let (data, ck, _) = setup_pipeline(d, t, m);
        let label = format!("t={},d={},m={}", t, d, m);

        group.bench_with_input(BenchmarkId::new("open", &label), &label, |b, _| {
            b.iter_with_setup(new_transcript, |mut transcript| {
                FflonkyKzg::<F, CS>::open_single(&ck, &data.fs, t, &data.roots, &mut transcript)
            })
        });
    }

    group.finish();
}

fn fflonky_kzg_verify(c: &mut Criterion) {
    let mut group = c.benchmark_group("fflonky-kzg-verify");

    for (t, d, m) in [(4, 63, 1), (4, 63, 2), (4, 255, 1), (8, 63, 1)] {
        let (data, ck, vk) = setup_pipeline(d, t, m);
        let label = format!("t={},d={},m={}", t, d, m);

        let mut transcript = new_transcript();
        let proof =
            FflonkyKzg::<F, CS>::open_single(&ck, &data.fs, t, &data.roots, &mut transcript);

        group.bench_with_input(BenchmarkId::new("verify", &label), &label, |b, _| {
            b.iter_with_setup(
                || (new_transcript(), proof.clone()),
                |(mut transcript, proof)| {
                    FflonkyKzg::<F, CS>::verify_single(
                        &vk,
                        &data.gc,
                        t,
                        proof,
                        &data.roots,
                        &data.vss,
                        &mut transcript,
                    )
                },
            )
        });
    }

    group.finish();
}

fn fflonky_kzg_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("fflonky-kzg-roundtrip");

    let (t, d, m) = (4, 255, 2);
    let (data, ck, vk) = setup_pipeline(d, t, m);

    group.bench_function("open+verify/t=4,d=255,m=2", |b| {
        b.iter_with_setup(new_transcript, |mut transcript| {
            let proof = FflonkyKzg::<F, CS>::open_single(
                &ck,
                &data.fs,
                t,
                &data.roots,
                &mut transcript,
            );
            FflonkyKzg::<F, CS>::verify_single(
                &vk,
                &data.gc,
                t,
                proof,
                &data.roots,
                &data.vss,
                &mut transcript,
            )
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    fflonky_kzg_open,
    fflonky_kzg_verify,
    fflonky_kzg_roundtrip,
);
criterion_main!(benches);
