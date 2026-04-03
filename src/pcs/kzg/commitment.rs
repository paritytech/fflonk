use ark_ec::{AffineRepr, CurveGroup};
use ark_serialize::*;
use ark_std::iter::Sum;
use ark_std::ops::{Add, Sub};
use ark_std::vec::Vec;

use crate::pcs::Commitment;
use crate::utils::ec::small_multiexp_affine;

/// KZG commitment to G1 represented in affine coordinates.
#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct WrappedAffine<C: AffineRepr>(pub C);

impl<C: AffineRepr> Commitment<C::ScalarField> for WrappedAffine<C> {
    fn mul(&self, by: C::ScalarField) -> WrappedAffine<C> {
        WrappedAffine(self.0.mul(by).into())
    }

    fn combine(coeffs: &[C::ScalarField], commitments: &[Self]) -> Self {
        let bases = commitments.iter().map(|c| c.0).collect::<Vec<_>>();
        let prod = small_multiexp_affine(coeffs, &bases);
        WrappedAffine(prod.into())
    }
}

impl<C: AffineRepr> Add<Self> for WrappedAffine<C> {
    type Output = WrappedAffine<C>;

    fn add(self, other: WrappedAffine<C>) -> WrappedAffine<C> {
        WrappedAffine((self.0 + other.0).into_affine())
    }
}

impl<C: AffineRepr> Sub<Self> for WrappedAffine<C> {
    type Output = WrappedAffine<C>;

    fn sub(self, other: WrappedAffine<C>) -> WrappedAffine<C> {
        WrappedAffine((self.0 + -other.0.into_group()).into_affine())
    }
}

impl<C: AffineRepr> Sum<Self> for WrappedAffine<C> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> WrappedAffine<C> {
        WrappedAffine(iter.map(|c| c.0.into_group()).sum::<C::Group>().into_affine())
    }
}
