use crate::{
    polynomial::{polynomial::*, quotient::*},
    structure::structure::*,
};
use algebraeon_nzq::natural::*;
use algebraeon_sets::structure::*;
use itertools::Itertools;

use super::modulo::Modulo;

impl<FS: FiniteFieldStructure> FiniteUnitsStructure for FieldExtensionStructure<FS> {
    fn all_units(&self) -> Vec<Self::Set> {
        let mut all_base_elements = vec![self.ring().coeff_ring().zero()];
        for unit in self.ring().coeff_ring().all_units() {
            all_base_elements.push(unit);
        }

        let mut all_base_elements_product = (0..self.degree())
            .map(|_| &all_base_elements)
            .multi_cartesian_product();

        // Pop the all-zeros element
        all_base_elements_product.next().unwrap();

        // What remains is the coefficients for all non-zero elements in self
        all_base_elements_product
            .map(|coeffs| {
                self.ring().reduce_poly(Polynomial::from_coeffs(
                    coeffs.into_iter().cloned().collect(),
                ))
            })
            .collect()
    }
}

impl<FS: FiniteFieldStructure> FiniteFieldStructure for FieldExtensionStructure<FS> {
    fn characteristic_and_power(&self) -> (Natural, Natural) {
        let (p, t) = self.ring().coeff_ring().characteristic_and_power();
        let d = Natural::from(self.degree());
        (p, d * t)
    }
}

pub fn new_finite_field_extension<FS: FiniteFieldStructure>(
    finite_field: FS,
    poly: <PolynomialStructure<FS> as Structure>::Set,
) -> FieldExtensionStructure<FS>
where
    PolynomialStructure<FS>: UniqueFactorizationStructure,
{
    FieldExtensionStructure::<FS>::new_field(
        PolynomialStructure::new(finite_field.into()).into(),
        poly,
    )
}

pub fn f9() -> FieldExtensionStructure<CannonicalStructure<Modulo<3>>> {
    use crate::number::finite_fields::modulo::*;
    new_finite_field_extension::<CannonicalStructure<Modulo<3>>>(
        CannonicalStructure::<Modulo<3>>::new().into(),
        Polynomial::from_coeffs(vec![1, 1, 2]),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_f9_elements() {
        let f9 = f9();

        let (p, t) = f9.characteristic_and_power();
        assert_eq!(p, 3u32.into());
        assert_eq!(t, 2u32.into());

        let mut c = 0;
        for x in f9.all_elements() {
            println!("{:?}", x);
            c += 1;
        }
        assert_eq!(c, 9);

        let f9_poly = PolynomialStructure::new(f9.clone().into());
        let poly = Polynomial::from_coeffs(
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 1, 5, 1, 1, 24, 14, 12, 4, 4, 34, 234, 23, 423,
                4, 234, 7, 4,
            ]
            .into_iter()
            .map(|c| f9.from_int(c))
            .collect(),
        );
        println!(
            "{}",
            f9_poly
                .factorize_monic(&poly)
                .unwrap()
                .factorize_squarefree()
                .factorize_distinct_degree()
                .factorize_cantor_zassenhaus()
        );
    }
}
