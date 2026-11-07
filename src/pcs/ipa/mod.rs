pub mod hiding;

use crate::pcs::commitment::WrappedAffine;
use crate::pcs::{CommitterKey, PcsParams, RawVerifierKey, VerifierKey, PCS};
use crate::Poly;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{batch_inversion, Field, Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::Rng;
use ark_std::test_rng;
use ark_std::vec;
use ark_std::vec::Vec;
use ark_std::UniformRand;
#[cfg(feature = "parallel")]
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

/// Parameters of a non-hiding Inner Product Argument (IPA) Polynomial Commitment Scheme (PCS).
#[derive(Clone, Debug, Eq, PartialEq, CanonicalSerialize, CanonicalDeserialize)]
pub struct IpaPcs<C: CurveGroup> {
    /// The number of bases `n = 2^k = g.len()`, a power of `2`.
    n: usize,
    /// `k = log2(n)`, the number of folding rounds.
    log_n: usize,
    /// The bases `G_0, ..., G_{n-1}` committing to the coefficients of a polynomial.
    gs: Vec<C::Affine>,
    /// The base `U` committing to the inner product in Bulletproofs.
    // TODO 3: re-randomize this, or sample new
    u: C::Affine,
}

#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct Proof<C: AffineRepr> {
    l: Vec<C>,
    r: Vec<C>,
    g_final: C, // aka U
    f_final: C::ScalarField,
    xs: Vec<C::ScalarField>, //TODO:
}

impl<C: CurveGroup> IpaPcs<C> {
    pub fn ipa_open(&self, f: Vec<C::ScalarField>, z: Vec<C::ScalarField>) -> Proof<C::Affine> {
        let mut f = f;
        f.resize(self.n, C::ScalarField::zero());
        assert_eq!(f.len(), self.n);
        assert_eq!(z.len(), self.n);

        let mut g_folded = self.gs.clone();
        let mut f_folded = f;
        let mut z_folded = z;

        let mut l = Vec::<C::Affine>::with_capacity(self.log_n);
        let mut r = Vec::<C::Affine>::with_capacity(self.log_n);

        // TODO 4: sample from transcript
        let xs: Vec<_> = (0..self.log_n)
            .map(|_| C::ScalarField::from(test_rng().gen::<u128>()))
            .collect();

        let mut n1 = self.n;
        for x in xs.iter() {
            n1 /= 2;

            let gl = &g_folded[..n1];
            let gr = &g_folded[n1..];
            let fl = &f_folded[..n1];
            let fr = &f_folded[n1..];
            let zl = &z_folded[..n1];
            let zr = &z_folded[n1..];

            let cl = scalar_prod(fl, zr);
            let cr = scalar_prod(fr, zl);

            let points = [gr, &[self.u]].concat();
            let scalars = [fl, &[cl]].concat();
            l.push(C::msm(&points, &scalars).unwrap().into());

            let points = [gl, &[self.u]].concat();
            let scalars = [fr, &[cr]].concat();
            r.push(C::msm(&points, &scalars).unwrap().into());

            // TODO: fold with x and x^2, see halo-inf
            let x_inv = x.inverse().unwrap();

            g_folded = fold_points(gl, gr, &x_inv);
            f_folded = fold_scalars(fl, fr, &x);
            z_folded = fold_scalars(zl, zr, &x_inv);
        }

        let g_final = g_folded[0];
        let f_final = f_folded[0];

        Proof {
            l,
            r,
            g_final,
            f_final,
            xs,
        }
    }

    // TODO 6: 1 msm
    pub fn ipa_check(
        &self,
        c: C::Affine,
        z: C::ScalarField,
        v: C::ScalarField,
        proof: Proof<C::Affine>,
    ) -> bool {
        // TODO 5: sample from transcript
        let xs = proof.xs.clone();
        let mut xs_inv = xs.clone();
        batch_inversion(xs_inv.as_mut_slice());

        let final_exps = final_folding_exponents(&xs_inv);
        let g_final = C::msm(&self.gs, &final_exps).unwrap().into_affine();
        self.check_assuming_g_final(g_final, c, z, v, proof)
    }

    /// `g_final` -- folded vector of Pedersen bases `G1,...,Gn`
    /// `u` -- the extra base used to commit to the scalar products in Bulletproofs
    /// `cf` -- Pedersen commitment to the polynomial, `Cf = f0.G1 + ...+ fd.Gn`, where `n = d+1`
    /// `f(z) = v`
    pub fn check_assuming_g_final(
        &self,
        g_final: C::Affine,
        c: C::Affine,
        z: C::ScalarField,
        v: C::ScalarField,
        proof: Proof<C::Affine>,
    ) -> bool {
        // TODO: duplicate code
        let xs = proof.xs;
        let mut xs_inv = xs.clone();
        batch_inversion(xs_inv.as_mut_slice());

        // the main Bulletproof commitment is `P = Cf + vU`
        // The folded one is then `P' = P + (1/x_i).Li + x_i.R_i`
        let bases = [proof.l, proof.r, vec![self.u]].concat();
        let exps = [xs_inv.as_slice(), xs.as_slice(), &[v]].concat();
        let cp_final = c + C::msm(&bases, &exps).unwrap();

        // now we check that it's consistent with the other values
        let z_final = evaluate_final_poly(&xs_inv, &z);
        let sp_final = proof.f_final * z_final; // scalar product of size 1
        let rhs = g_final * proof.f_final + self.u * sp_final;
        cp_final == rhs //TODO: merge into single msm
    }
}

impl<C: CurveGroup> CommitterKey for IpaPcs<C> {
    fn max_degree(&self) -> usize {
        self.gs.len() - 1
    }
}

impl<C: CurveGroup> VerifierKey for IpaPcs<C> {}

impl<C: CurveGroup> RawVerifierKey for IpaPcs<C> {
    type VK = Self;

    fn prepare(&self) -> Self::VK {
        self.clone()
    }
}

impl<C: CurveGroup> PcsParams for IpaPcs<C> {
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

impl<C: CurveGroup> PCS<C::ScalarField> for IpaPcs<C> {
    type C = WrappedAffine<C>;
    type Proof = Proof<C::Affine>;
    type CK = Self;
    type VK = Self;
    type Params = Self;

    fn setup<R: Rng>(max_degree: usize, rng: &mut R) -> Self::Params {
        let max_coeffs = max_degree + 1;
        let log_n = ark_std::log2(max_coeffs);
        let n = 2usize.pow(log_n);
        let log_n = log_n as usize;
        assert!(max_coeffs <= n);
        //TODO 1: derive from "seed"
        //TODO 2: proj + batch affine
        let gs = (0..n).map(|_| C::Affine::rand(rng)).collect::<Vec<_>>();
        let u = C::Affine::rand(rng);
        Self { n, log_n, gs, u }
    }

    fn commit(ck: &Self, p: &Poly<C::ScalarField>) -> Result<Self::C, ()> {
        if ck.max_evals() < p.coeffs.len() {
            return Err(());
        }
        let p_comm: C::Affine = C::msm(&ck.gs[..p.coeffs.len()], &p.coeffs)
            .unwrap()
            .into_affine();
        Ok(WrappedAffine(p_comm))
    }

    fn open(ck: &Self, p: &Poly<C::ScalarField>, x: C::ScalarField) -> Result<Self::Proof, ()> {
        if ck.max_evals() < p.coeffs.len() {
            return Err(());
        }
        let x_powers: Vec<C::ScalarField> = crate::utils::powers(x).take(ck.n).collect();
        let proof = ck.ipa_open(p.coeffs.clone(), x_powers);
        Ok(proof)
    }

    fn verify(
        vk: &Self,
        c: Self::C,
        x: C::ScalarField,
        z: C::ScalarField,
        proof: Self::Proof,
    ) -> Result<(), ()> {
        vk.ipa_check(c.0, x, z, proof).then(|| ()).ok_or(())
    }
}

fn scalar_prod<F: Field>(a: &[F], b: &[F]) -> F {
    ark_std::cfg_iter!(a).zip(b).map(|(a, b)| *a * b).sum()
}

// Computes `l + xr` pointwise.
fn fold_points<C: AffineRepr>(l: &[C], r: &[C], x: &C::ScalarField) -> Vec<C> {
    assert_eq!(l.len(), r.len());
    let proj: Vec<C::Group> = ark_std::cfg_iter!(l)
        .zip(r)
        .map(|(&l, &r)| l + r * x)
        .collect();
    C::Group::normalize_batch(&proj)
}

// Computes `l + xr` pointwise.
fn fold_scalars<F: Field>(l: &[F], r: &[F], x: &F) -> Vec<F> {
    assert_eq!(l.len(), r.len());
    ark_std::cfg_iter!(l)
        .zip(r)
        .map(|(&l, &r)| l + r * x)
        .collect()
}

// n = 2^m
// Folding elements V = [A1, ..., An] with scalars [x1, ..., xm] recursively m times using formula
// V = VL || VR, Vi = VL + xi * VR pointwise, V := Vi, i = 1,...,m
// results in V = Vm = [c1A1 + ... + cnAn], where ci = prod({xj | if j-th bit of i-1 is set}).
// This function computes these ci-s.
fn final_folding_exponents<F: Field>(xs: &[F]) -> Vec<F> {
    let m = xs.len();
    let mut n = 2usize.pow(m as u32);
    let mut res = vec![F::one(); n];
    for x in xs {
        n = n / 2;
        for chunk in res.rchunks_mut(n).step_by(2) {
            for elem in chunk.iter_mut() {
                *elem *= x;
            }
        }
    }
    res
}

// Computes (1 + x_m z)(1 + x_{m-1} z^2) ... (1 + x_1 z^{2^{m-1}}).
fn evaluate_final_poly<F: Field>(xs: &[F], z: &F) -> F {
    let mut res = F::one();
    let mut z_i = z.clone();
    for x in xs.iter().rev() {
        res *= z_i * x + F::one();
        z_i = z_i.square(); //TODO: remove extra squaring
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    use ark_bls12_381::{Fr, G1Affine, G1Projective};
    use ark_ec::VariableBaseMSM;
    use ark_poly::univariate::DensePolynomial;
    use ark_poly::DenseUVPolynomial;
    use ark_poly::Polynomial;
    use ark_std::{test_rng, UniformRand};

    #[test]
    fn ipa_pc() {
        let rng = &mut test_rng();

        let log_n = 2;
        let n = 2usize.pow(log_n as u32);

        let g = (0..n).map(|_| G1Affine::rand(rng)).collect::<Vec<_>>();
        let h = G1Affine::rand(rng);

        let f = DensePolynomial::<Fr>::rand(n - 1, rng);
        let f_comm: G1Affine = G1Projective::msm(&g, &f.coeffs).unwrap().into_affine();

        let z = Fr::rand(rng);
        let z_powers: Vec<Fr> = utils::powers(z).take(n).collect();
        let v = f.evaluate(&z);

        let ipa = IpaPcs::<G1Projective> {
            n,
            log_n,
            gs: g,
            u: h,
        };
        let proof = ipa.ipa_open(f.coeffs, z_powers);
        assert!(ipa.ipa_check(f_comm, z, v, proof));
    }
}
