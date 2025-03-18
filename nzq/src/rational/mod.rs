use crate::integer::*;
use crate::natural::*;
use algebraeon_sets::structure::*;
use malachite_base::num::basic::traits::{One, OneHalf, Two, Zero};
use malachite_q::arithmetic::traits::{Approximate, SimplestRationalInInterval};
use std::{
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rational(malachite_q::Rational);

impl Rational {
    pub(crate) fn from_malachite(value: malachite_q::Rational) -> Self {
        Self(value)
    }
    pub(crate) fn to_malachite(self) -> malachite_q::Rational {
        self.0
    }
    pub(crate) fn to_malachite_ref(&self) -> &malachite_q::Rational {
        &self.0
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u8> for Rational {
    fn from(value: u8) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<u16> for Rational {
    fn from(value: u16) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<u32> for Rational {
    fn from(value: u32) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<u64> for Rational {
    fn from(value: u64) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<u128> for Rational {
    fn from(value: u128) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<usize> for Rational {
    fn from(value: usize) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<i8> for Rational {
    fn from(value: i8) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<i16> for Rational {
    fn from(value: i16) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<i32> for Rational {
    fn from(value: i32) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<i64> for Rational {
    fn from(value: i64) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<i128> for Rational {
    fn from(value: i128) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<isize> for Rational {
    fn from(value: isize) -> Self {
        Self(malachite_q::Rational::from(value))
    }
}
impl From<Natural> for Rational {
    fn from(value: Natural) -> Self {
        Self(malachite_q::Rational::from(value.to_malachite()))
    }
}
impl From<&Natural> for Rational {
    fn from(value: &Natural) -> Self {
        Self(malachite_q::Rational::from(value.to_malachite_ref()))
    }
}
impl From<Integer> for Rational {
    fn from(value: Integer) -> Self {
        Self(malachite_q::Rational::from(value.to_malachite()))
    }
}
impl From<&Integer> for Rational {
    fn from(value: &Integer) -> Self {
        Self(malachite_q::Rational::from(value.to_malachite_ref()))
    }
}

impl FromStr for Rational {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(malachite_q::Rational::from_str(s)?))
    }
}

impl Rational {
    pub const ZERO: Self = Self(malachite_q::Rational::ZERO);
    pub const ONE: Self = Self(malachite_q::Rational::ONE);
    pub const TWO: Self = Self(malachite_q::Rational::TWO);
    pub const ONE_HALF: Self = Self(malachite_q::Rational::ONE_HALF);
}

impl AddAssign<Rational> for Rational {
    fn add_assign(&mut self, rhs: Rational) {
        self.0.add_assign(rhs.0)
    }
}
impl AddAssign<&Rational> for Rational {
    fn add_assign(&mut self, rhs: &Rational) {
        self.0.add_assign(&rhs.0)
    }
}

impl SubAssign<Rational> for Rational {
    fn sub_assign(&mut self, rhs: Rational) {
        self.0.sub_assign(rhs.0)
    }
}
impl SubAssign<&Rational> for Rational {
    fn sub_assign(&mut self, rhs: &Rational) {
        self.0.sub_assign(&rhs.0)
    }
}

impl MulAssign<Rational> for Rational {
    fn mul_assign(&mut self, rhs: Rational) {
        self.0.mul_assign(rhs.0)
    }
}
impl MulAssign<&Rational> for Rational {
    fn mul_assign(&mut self, rhs: &Rational) {
        self.0.mul_assign(&rhs.0)
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        Rational(self.0.neg())
    }
}
impl Neg for &Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        Rational((&self.0).neg())
    }
}

impl Add<Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        Rational(self.0.add(rhs.0))
    }
}
impl Add<&Rational> for Rational {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        Rational(self.0.add(&rhs.0))
    }
}
impl Add<Rational> for &Rational {
    type Output = Rational;

    fn add(self, rhs: Rational) -> Self::Output {
        Rational((&self.0).add(rhs.0))
    }
}
impl Add<&Rational> for &Rational {
    type Output = Rational;

    fn add(self, rhs: &Rational) -> Self::Output {
        Rational((&self.0).add(&rhs.0))
    }
}

impl Sub<Rational> for Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        Rational(self.0.sub(rhs.0))
    }
}
impl Sub<&Rational> for Rational {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        Rational(self.0.sub(&rhs.0))
    }
}
impl Sub<Rational> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: Rational) -> Self::Output {
        Rational((&self.0).sub(rhs.0))
    }
}
impl Sub<&Rational> for &Rational {
    type Output = Rational;

    fn sub(self, rhs: &Rational) -> Self::Output {
        Rational((&self.0).sub(&rhs.0))
    }
}

impl Mul<Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        Rational(self.0.mul(rhs.0))
    }
}
impl Mul<&Rational> for Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        Rational(self.0.mul(&rhs.0))
    }
}
impl Mul<Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: Rational) -> Self::Output {
        Rational((&self.0).mul(rhs.0))
    }
}
impl Mul<&Rational> for &Rational {
    type Output = Rational;

    fn mul(self, rhs: &Rational) -> Self::Output {
        Rational((&self.0).mul(&rhs.0))
    }
}

impl Div<Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        Rational(self.0.div(rhs.0))
    }
}
impl Div<&Rational> for Rational {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        Rational(self.0.div(&rhs.0))
    }
}
impl Div<Rational> for &Rational {
    type Output = Rational;

    fn div(self, rhs: Rational) -> Self::Output {
        Rational((&self.0).div(rhs.0))
    }
}
impl Div<&Rational> for &Rational {
    type Output = Rational;

    fn div(self, rhs: &Rational) -> Self::Output {
        Rational((&self.0).div(&rhs.0))
    }
}

impl Rational {
    pub fn numerator(&self) -> Integer {
        //malachite returns a natural for the numerator for some
        if self >= &Rational::ZERO {
            Integer::from(Natural::from_malachite(self.0.numerator_ref().clone()))
        } else {
            -Natural::from_malachite(self.0.numerator_ref().clone())
        }
    }

    pub fn denominator(&self) -> Natural {
        Natural::from_malachite(self.0.denominator_ref().clone())
    }

    pub fn from_integers(n: impl Into<Integer>, d: impl Into<Integer>) -> Self {
        Self(malachite_q::Rational::from_integers(
            n.into().to_malachite(),
            d.into().to_malachite(),
        ))
    }

    pub fn abs(self) -> Self {
        use malachite_base::num::arithmetic::traits::Abs;
        Self(self.0.abs())
    }

    pub fn abs_ref(&self) -> Self {
        use malachite_base::num::arithmetic::traits::Abs;
        Self((&self.0).abs())
    }

    pub fn into_abs_numerator_and_denominator(self) -> (Natural, Natural) {
        let (n, d) = self.0.into_numerator_and_denominator();
        (Natural::from_malachite(n), Natural::from_malachite(d))
    }

    pub fn simplest_rational_in_closed_interval(a: &Rational, b: &Rational) -> Self {
        Self(malachite_q::Rational::simplest_rational_in_closed_interval(
            &a.0, &b.0,
        ))
    }

    pub fn simplest_rational_in_open_interval(a: &Rational, b: &Rational) -> Self {
        Self(malachite_q::Rational::simplest_rational_in_open_interval(
            &a.0, &b.0,
        ))
    }

    pub fn exhaustive_rationals() -> impl Iterator<Item = Rational> {
        malachite_q::exhaustive::exhaustive_rationals().map(|v| Rational(v))
    }

    pub fn approximate(self, max_denominator: &Natural) -> Self {
        Self(self.0.approximate(max_denominator.to_malachite_ref()))
    }

    pub fn floor(self) -> Integer {
        use malachite_base::num::arithmetic::traits::Floor;
        Integer::from_malachite(self.0.floor())
    }
    pub fn floor_ref(&self) -> Integer {
        use malachite_base::num::arithmetic::traits::Floor;
        Integer::from_malachite((&self.0).floor())
    }

    pub fn ceil(self) -> Integer {
        use malachite_base::num::arithmetic::traits::Ceiling;
        Integer::from_malachite(self.0.ceiling())
    }
    pub fn ceil_ref(&self) -> Integer {
        use malachite_base::num::arithmetic::traits::Ceiling;
        Integer::from_malachite((&self.0).ceiling())
    }

    pub fn try_from_float_simplest(x: f64) -> Result<Self, ()> {
        match malachite_q::Rational::try_from_float_simplest(x) {
            Ok(x) => Ok(Self(x)),
            Err(_) => Err(()),
        }
    }
}

impl MetaType for Rational {
    type Structure = CannonicalStructure<Rational>;

    fn structure() -> std::rc::Rc<Self::Structure> {
        CannonicalStructure::new().into()
    }
}

pub fn rat_to_string(a: Rational) -> String {
    if a == Rational::ZERO {
        return "0".into();
    }
    let neg = a < Rational::from(0);
    let (mant, exp, _): (f64, _, _) = a
        .to_malachite()
        .sci_mantissa_and_exponent_round(malachite_base::rounding_modes::RoundingMode::Nearest)
        .unwrap();
    let mut b = (2.0 as f64).powf(exp as f64) * mant;
    if neg {
        b = -b;
    }
    b = (1000.0 * b).round() / 1000.0;
    b.to_string()
}
