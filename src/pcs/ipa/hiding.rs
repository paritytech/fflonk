use crate::pcs::commitment::WrappedAffine;
use crate::pcs::ipa::ipa_pc;
use crate::pcs::PCS;
use crate::pcs::{ipa, CommitterKey, PcsParams, RawVerifierKey, VerifierKey};
use crate::Poly;
use ark_ec::CurveGroup;
use ark_ff::{PrimeField, Zero};
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::Rng;
use ark_std::vec::Vec;
use ark_std::UniformRand;

// To open a hiding commitment `Cp = Commit(p, t1) = (p0.G0 + ... + pn.Gn) + t1.H` at `z`,
// the prover:
// 1. Computes a hiding commitment to a random `q` such that `deg(q) = deg(p)` and `q(z) = 0`
//   `Cq = Commit(q, t2) = (q0.G0 + ... + qn.Gn) + t2.H`.
// 2. Computes the blinded polynomial `p' = p + a.q`, p'(z) = p(z) + a.q(z) = p(z)`.
// 3. Opens the non-hiding commitment `Cp' = Commit(p', 0)` to the blinded polynomial `p'`.
// 4. Reveals `Cq` and `t = t1 + a.t2` to the verifier.
// The verifier
// 1. Computes the non-hiding commitment `Cp'` as `Cp + a.Cq - t'H = (Cp - t1.H) + a.(Cq - t2.H)`.
// 2. Verifies the opening against `Cp'`.
#[derive(Clone, Debug, Eq, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct HidingIpa<C: CurveGroup> {
    pub ipa_pcs: ipa::IPA<C>,
    pub h: C::Affine,
}

/// `Cp = Commit(p, t1) = (p0.G0 + ... + pn.Gn) + t1.H`.
#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct HidingProof<C: CurveGroup> {
    /// A hiding commitment to the blinding polynomial `q`, `deg(q) = deg(p), q(z) = 0`.
    /// `Commit(q, t2) = (q0.G0 + ... + qn.Gn) + t2.H`.
    q: C::Affine,
    /// The blinding factor `t` in the hiding commitment `Cp' = Cp + a.Cq` to the blinded polynomial `p' = p + a.q`.
    /// `t = t1 + a.t2`, `Cp' = Commit(p', t) = Commit(p', 0) + (t1 + a.t2).H`
    t: C::ScalarField,
    /// Opening proof for the non-hiding commitment `Commit(p', 0)` to `p'`.
    ipa_pcs_proof: ipa_pc::Proof<C::Affine>,
    // TODO: remove
    a: C::ScalarField,
}

impl<F: PrimeField, C: CurveGroup<ScalarField = F>> HidingIpa<C> {
    pub fn commit_hiding(&self, p: &Poly<F>, bf: F) -> Result<WrappedAffine<C>, ()> {
        let c = ipa::IPA::commit(&self.ipa_pcs, p)?;
        self.reblind(c.0, F::zero(), bf)
    }

    pub fn reblind(&self, c: C::Affine, bf_old: F, bf_new: F) -> Result<WrappedAffine<C>, ()> {
        let c = c + self.h * (bf_new - bf_old);
        let c = c.into_affine();
        Ok(WrappedAffine(c))
    }
}

impl<C: CurveGroup> CommitterKey for HidingIpa<C> {
    fn max_degree(&self) -> usize {
        self.ipa_pcs.max_degree()
    }
}

impl<C: CurveGroup> VerifierKey for HidingIpa<C> {}

impl<C: CurveGroup> RawVerifierKey for HidingIpa<C> {
    type VK = Self;

    fn prepare(&self) -> Self::VK {
        self.clone()
    }
}

impl<C: CurveGroup> PcsParams for HidingIpa<C> {
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

impl<C: CurveGroup> PCS<C::ScalarField> for HidingIpa<C> {
    type C = WrappedAffine<C>;
    type Proof = HidingProof<C>;
    type CK = Self;
    type VK = Self;
    type Params = Self;

    fn setup<R: Rng>(max_degree: usize, rng: &mut R) -> Self::Params {
        let ipa_pcs = ipa::IPA::setup(max_degree, rng);
        let h = C::Affine::rand(rng);
        Self { ipa_pcs, h }
    }

    fn commit(ck: &Self::CK, p: &Poly<C::ScalarField>) -> Result<Self::C, ()> {
        Self::commit_hiding(ck, p, C::ScalarField::zero())
    }

    fn open(
        _ck: &Self::CK,
        _p: &Poly<C::ScalarField>,
        _x: C::ScalarField,
    ) -> Result<Self::Proof, ()> {
        todo!()
    }

    fn open_hiding<R: Rng>(
        ck: &Self::CK,
        p: &Poly<C::ScalarField>,
        bf: C::ScalarField,
        x: C::ScalarField,
        rng: &mut R,
    ) -> Result<Self::Proof, ()> {
        let t1 = bf;
        let mut q = Poly::rand(p.degree(), rng);
        let q_at_z = q.evaluate(&x);
        q[0] -= q_at_z;
        debug_assert!(q.evaluate(&x).is_zero());
        let t2 = C::ScalarField::rand(rng);
        let cq = ck.commit_hiding(&q, t2)?.0;
        let a = C::ScalarField::rand(rng); // TODO: that's a FS point
        let p = p + q * a;
        let t = t1 + t2 * a;
        let ipa_pcs_proof = ipa::IPA::open(&ck.ipa_pcs, &p, x)?;
        Ok(HidingProof {
            q: cq,
            t,
            ipa_pcs_proof,
            a,
        })
    }

    fn verify(
        vk: &Self::VK,
        c: Self::C,
        x: C::ScalarField,
        z: C::ScalarField,
        proof: Self::Proof,
    ) -> Result<(), ()> {
        let c = c.0 + proof.q * proof.a - vk.h * proof.t;
        ipa::IPA::verify(
            &vk.ipa_pcs,
            WrappedAffine(c.into_affine()),
            x,
            z,
            proof.ipa_pcs_proof,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Fr, G1Projective};
    use ark_std::{test_rng, UniformRand};

    #[test]
    fn test_hiding_ipa_opening() {
        let rng = &mut test_rng();

        let max_coeffs = 2usize.pow(6);
        let max_degree = max_coeffs - 1;

        let hiding_pcs = HidingIpa::<G1Projective>::setup(max_degree, rng);

        let p = Poly::rand(max_degree, rng);
        let bf = Fr::rand(rng);

        let c = hiding_pcs.commit_hiding(&p, bf).unwrap();

        let x = Fr::rand(rng);
        let pi = HidingIpa::<G1Projective>::open_hiding(&hiding_pcs, &p, bf, x, rng).unwrap();
        let v = p.evaluate(&x);

        assert!(HidingIpa::<G1Projective>::verify(&hiding_pcs, c, x, v, pi).is_ok());
    }
}
