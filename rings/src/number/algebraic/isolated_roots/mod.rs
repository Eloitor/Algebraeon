use std::collections::HashSet;
use std::rc::Rc;
use std::str::FromStr;

use complex::ComplexAlgebraic;
use complex::ComplexAlgebraicRoot;
use itertools::Itertools;
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::traits::OneHalf;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::arithmetic::traits::SimplestRationalInInterval;
use malachite_q::Rational;
use real::RealAlgebraic;

use crate::number::algebraic::bisection_gen::RationalSimpleBetweenGenerator;
use crate::number::algebraic::number_field::*;
use crate::polynomial::polynomial::*;
use crate::ring_structure::cannonical::*;
use crate::ring_structure::structure::*;
use crate::structure::*;

pub mod poly_tools;
pub mod complex;
pub mod real;

#[cfg(test)]
mod tests;

use poly_tools::*;

fn rat_to_string(a: Rational) -> String {
    if a == 0 {
        return "0".into();
    }
    let neg = a < Rational::from(0);
    let (mant, exp): (f64, _) = a
        .sci_mantissa_and_exponent_with_rounding(
            malachite_base::rounding_modes::RoundingMode::Nearest,
        )
        .unwrap();
    let mut b = (2.0 as f64).powf(exp as f64) * mant;
    if neg {
        b = -b;
    }
    b = (1000.0 * b).round() / 1000.0;
    b.to_string()
}

//write target as a polynomial expression of generator if possible, otherwise return None
pub fn as_poly_expr(
    target: &ComplexAlgebraic,
    generator: &ComplexAlgebraic,
) -> Option<Polynomial<Rational>> {
    //idea: factor the minimal polynomial of target in the algebraic number field generated by the generator
    //loop through the linear factors (x - a) and check if a = target

    //let K = Q[generator]
    let gen_anf = new_anf(generator.min_poly());
    let gen_anf_poly = PolynomialStructure::new(gen_anf.clone().into());

    //the minimal polynomial of target in K[x]
    let target_min_poly = target
        .min_poly()
        .apply_map(|c| Polynomial::constant(c.clone()));

    let target_min_poly_factored = gen_anf_poly.factor(&target_min_poly).unwrap();
    let mut generator = generator.clone();
    for (factor, _factor_mult) in target_min_poly_factored.factors() {
        //the factor should be monic
        debug_assert!(gen_anf.equal(&factor.leading_coeff().unwrap(), &gen_anf.one()));
        if factor.degree().unwrap() == 1 {
            let possible_embedded_target = gen_anf.neg(&factor.coeff(0));
            if generator.apply_poly(&possible_embedded_target) == *target {
                return Some(possible_embedded_target);
            }
        }
    }
    None
}

pub fn anf_pair_primitive_element_theorem(
    a: &ComplexAlgebraic,
    b: &ComplexAlgebraic,
) -> (
    ComplexAlgebraic,
    Integer,
    Integer,
    Polynomial<Rational>,
    Polynomial<Rational>,
) {
    //try g = a
    match as_poly_expr(b, a) {
        Some(q) => {
            return (a.clone(), Integer::ONE, Integer::ZERO, Polynomial::var(), q);
        }
        None => {}
    }

    //try g = b
    match as_poly_expr(a, b) {
        Some(p) => {
            return (b.clone(), Integer::ZERO, Integer::ONE, p, Polynomial::var());
        }
        None => {}
    }

    let mut nontrivial_linear_combinations =
        malachite_q::exhaustive::exhaustive_rationals().map(|r| (r.numerator(), r.denominator()));
    nontrivial_linear_combinations.next().unwrap();
    for (x, y) in nontrivial_linear_combinations {
        let gen = ComplexAlgebraic::add(
            &ComplexAlgebraic::mul(
                &ComplexAlgebraic::Real(RealAlgebraic::Rational(Rational::from(x.clone()))),
                a,
            ),
            &ComplexAlgebraic::mul(
                &ComplexAlgebraic::Real(RealAlgebraic::Rational(Rational::from(y.clone()))),
                b,
            ),
        );

        match as_poly_expr(a, &gen) {
            Some(a_rel_gen) => {
                let anf = new_anf(gen.min_poly());
                //gen = xa + yb
                //so b = (gen - xa) / y
                let b_rel_gen = anf.mul(
                    &anf.add(
                        &Polynomial::var(),
                        &anf.mul(&a_rel_gen, &Polynomial::constant(Rational::from(-&x))),
                    ),
                    &Polynomial::constant(Rational::from_integers(Integer::from(1), y.clone())),
                );
                #[cfg(debug_assertions)]
                {
                    let mut gen_mut = gen.clone();
                    assert_eq!(a, &gen_mut.apply_poly(&a_rel_gen));
                    assert_eq!(b, &gen_mut.apply_poly(&b_rel_gen));
                }
                return (gen, x, y, a_rel_gen, b_rel_gen);
            }
            None => {}
        }
    }
    unreachable!()
}

/*
input: non-empty list of complex algebraic numbers (a_1, a_2, ..., a_n)
output: (g, p_1, p_2, ..., p_n) such that Q[a_1, a_2, ..., a_n] = Q[g]
        moreover a_i=p_i(g)
*/
pub fn anf_multi_primitive_element_theorem(
    nums: Vec<&ComplexAlgebraic>,
) -> (ComplexAlgebraic, Vec<Polynomial<Rational>>) {
    #[cfg(debug_assertions)]
    let orig_nums = nums.clone();

    assert!(!nums.is_empty());
    let mut nums = nums.into_iter();
    let mut g = nums.next().unwrap().clone();
    let mut p = vec![Polynomial::var()];
    for num in nums {
        let (new_g, _x, _y, old_g_poly, num_poly) = anf_pair_primitive_element_theorem(&g, num);
        let new_g_anf = new_anf(new_g.min_poly());
        p = p
            .into_iter()
            .map(|old_p| new_g_anf.reduce(&Polynomial::compose(&old_p, &old_g_poly)))
            .collect();
        p.push(num_poly);
        g = new_g;
    }
    #[cfg(debug_assertions)]
    {
        let n = orig_nums.len();
        assert_eq!(n, p.len());
        for i in 0..n {
            assert_eq!(orig_nums[i], &g.apply_poly(&p[i]));
        }
    }
    (g, p)
}
