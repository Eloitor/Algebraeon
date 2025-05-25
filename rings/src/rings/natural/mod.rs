use crate::structure::*;
use algebraeon_nzq::{traits::DivMod, *};
use algebraeon_sets::structure::*;

pub mod factorization;
pub mod functions;

impl SemiRingSignature for NaturalCanonicalStructure {
    fn zero(&self) -> Self::Set {
        Natural::ZERO
    }
    fn one(&self) -> Self::Set {
        Natural::ONE
    }
    fn add(&self, a: &Self::Set, b: &Self::Set) -> Self::Set {
        a + b
    }
    fn mul(&self, a: &Self::Set, b: &Self::Set) -> Self::Set {
        a * b
    }
}

impl CharacteristicSignature for NaturalCanonicalStructure {
    fn characteristic(&self) -> Natural {
        Natural::ZERO
    }
}

impl UnitsSignature for NaturalCanonicalStructure {
    fn inv(&self, a: &Self::Set) -> Result<Self::Set, RingDivisionError> {
        self.div(&self.one(), a)
    }
}

impl IntegralDomainSignature for NaturalCanonicalStructure {
    fn div(&self, a: &Self::Set, b: &Self::Set) -> Result<Self::Set, RingDivisionError> {
        match self.quorem(a, b) {
            Some((q, r)) => {
                if r == self.zero() {
                    Ok(q)
                } else {
                    Err(RingDivisionError::NotDivisible)
                }
            }
            None => Err(RingDivisionError::DivideByZero),
        }
    }
}

impl EuclideanDivisionSignature for NaturalCanonicalStructure {
    fn norm(&self, elem: &Self::Set) -> Option<Natural> {
        if elem == &Natural::ZERO {
            None
        } else {
            Some(elem.clone())
        }
    }

    fn quorem(&self, a: &Self::Set, b: &Self::Set) -> Option<(Self::Set, Self::Set)> {
        if b == &Natural::ZERO {
            None
        } else {
            Some(a.div_mod(b))
        }
    }
}
