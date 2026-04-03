use crate::pcs::ipa::ipa_pc;
use crate::pcs::{CommitterKey, PcsParams, RawVerifierKey, VerifierKey, PCS};
use crate::Poly;
use ark_ec::CurveGroup;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::Rng;
use ark_std::vec::Vec;
use crate::pcs::kzg::commitment::WrappedAffine;

#[derive(Clone, Debug, Eq, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct IPA<C: CurveGroup> {
    g: Vec<C::Affine>,
    u: C::Affine,
}

impl<C: CurveGroup> CommitterKey for IPA<C> {
    fn max_degree(&self) -> usize {
        self.g.len() - 1
    }
}

impl<C: CurveGroup> VerifierKey for IPA<C> {

}

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
    type C = WrappedAffine<C::Affine>;
    type Proof = ipa_pc::Proof<C::Affine>;
    type CK = Self;
    type VK = Self;
    type Params = Self;

    fn setup<R: Rng>(max_degree: usize, rng: &mut R) -> Self::Params {
        todo!()
    }

    fn commit(ck: &Self::CK, p: &Poly<C::ScalarField>) -> Self::C {
        todo!()
    }

    fn open(ck: &Self::CK, p: &Poly<C::ScalarField>, x: C::ScalarField) -> Self::Proof {
        todo!()
    }

    fn verify(vk: &Self::VK, c: Self::C, x: C::ScalarField, z: C::ScalarField, proof: Self::Proof) -> bool {
        todo!()
    }
}
