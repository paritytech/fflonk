use ark_ec::CurveGroup;
use ark_serialize::*;
use ark_std::iter::Sum;
use ark_std::ops::{Add, Mul, Sub};
use ark_std::vec::Vec;

use crate::pcs::Commitment;
use crate::utils::ec::small_multiexp_affine;

/// KZG commitment to G1 represented in affine coordinates.
#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct WrappedAffine<C: CurveGroup>(pub C::Affine);

impl<C: CurveGroup> Mul<C::ScalarField> for WrappedAffine<C> {
    type Output = Self;

    fn mul(self, by: C::ScalarField) -> Self {
        (&self).mul(by)
    }
}

impl<C: CurveGroup> Commitment<C::ScalarField> for WrappedAffine<C> {
    fn mul(&self, by: C::ScalarField) -> WrappedAffine<C> {
        WrappedAffine(self.0.mul(by).into_affine())
    }

    fn combine(coeffs: &[C::ScalarField], commitments: &[Self]) -> Self {
        let bases = commitments.iter().map(|c| c.0).collect::<Vec<_>>();
        let prod = small_multiexp_affine(coeffs, &bases);
        WrappedAffine(prod.into_affine())
    }
}

impl<C: CurveGroup> Add<Self> for WrappedAffine<C> {
    type Output = WrappedAffine<C>;

    fn add(self, other: WrappedAffine<C>) -> WrappedAffine<C> {
        WrappedAffine((self.0 + other.0).into_affine())
    }
}

impl<C: CurveGroup> Sub<Self> for WrappedAffine<C> {
    type Output = WrappedAffine<C>;

    fn sub(self, other: WrappedAffine<C>) -> WrappedAffine<C> {
        WrappedAffine((self.0 - other.0).into_affine())
    }
}

impl<C: CurveGroup> Sum<Self> for WrappedAffine<C> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> WrappedAffine<C> {
        let sum: C = iter.map(|c| c.0).sum();
        WrappedAffine(sum.into_affine())
    }
}
