use crate::pcs::ipa::ipa_pc;
use crate::pcs::commitment::WrappedAffine;
use crate::pcs::{CommitterKey, PcsParams, RawVerifierKey, VerifierKey, PCS};
use crate::Poly;
use ark_ec::CurveGroup;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::Rng;
use ark_std::vec::Vec;
use ark_std::UniformRand;

#[derive(Clone, Debug, Eq, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct IPA<C: CurveGroup> {
    log_n: usize,
    n: usize,
    pub g: Vec<C::Affine>,
    h: C::Affine,
}

impl<C: CurveGroup> CommitterKey for IPA<C> {
    fn max_degree(&self) -> usize {
        self.g.len() - 1
    }
}

impl<C: CurveGroup> VerifierKey for IPA<C> {}

impl<C: CurveGroup> RawVerifierKey for IPA<C> {
    type VK = Self;

    fn prepare(&self) -> Self::VK {
        self.clone()
    }
}

impl<C: CurveGroup> PcsParams for IPA<C> {
    type CK = Self;
    type VK = Self;
    type RVK = Self;

    fn ck(&self) -> Self::CK {
        self.clone()
    }

    fn vk(&self) -> Self::VK {
        self.clone()
    }

    fn raw_vk(&self) -> Self::RVK {
        self.clone()
    }
}

impl<C: CurveGroup> PCS<C::ScalarField> for IPA<C> {
    type C = WrappedAffine<C>;
    type Proof = ipa_pc::Proof<C::Affine>;
    type CK = Self;
    type VK = Self;
    type Params = Self;

    fn setup<R: Rng>(max_degree: usize, rng: &mut R) -> Self::Params {
        let log_n = ark_std::log2(max_degree + 1);
        let n = 2usize.pow(log_n);
        assert!(max_degree + 1 <= n);
        let g = (0..n).map(|_| C::Affine::rand(rng)).collect::<Vec<_>>(); //TODO: proj + batch affine
        let h = C::Affine::rand(rng);
        Self { log_n: log_n as usize, n, g, h }
    }

    fn commit(ck: &Self, p: &Poly<C::ScalarField>) -> Result<Self::C, ()> {
        if ck.max_evals() < p.coeffs.len() {
            return Err(());
        }
        let p_comm: C::Affine = C::msm(&ck.g[..p.coeffs.len()], &p.coeffs).unwrap().into_affine();
        Ok(WrappedAffine(p_comm))
    }

    fn open(ck: &Self, p: &Poly<C::ScalarField>, x: C::ScalarField) -> Result<Self::Proof, ()> {
        let n_coeffs = p.coeffs.len();
        let log_n = ark_std::log2(n_coeffs) as usize;
        let n = 1 << log_n;
        assert!(n_coeffs <= n);
        let x_powers: Vec<C::ScalarField> = crate::utils::powers(x).take(n).collect();
        let proof = ipa_pc::open(log_n, ck.g[..n].to_vec(), ck.h, p.coeffs.clone(), x_powers);
        Ok(proof)
    }

    fn verify(
        vk: &Self,
        c: Self::C,
        x: C::ScalarField,
        z: C::ScalarField,
        proof: Self::Proof,
    ) -> Result<(), ()> {
        let log_n = proof.log_n;
        let n = 1 << log_n;
        ipa_pc::check(vk.g[..n].to_vec(), vk.h, c.0, x, z, proof)
            .then(|| ())
            .ok_or(())
    }
}
