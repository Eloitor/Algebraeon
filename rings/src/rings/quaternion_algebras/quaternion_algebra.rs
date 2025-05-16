use crate::structure::FieldSignature;
use crate::structure::RingSignature;
use crate::structure::SemiRingSignature;
use algebraeon_nzq::RationalCanonicalStructure;
use algebraeon_sets::structure::EqSignature;
use algebraeon_sets::structure::SetSignature;
use algebraeon_sets::structure::Signature;

#[derive(Debug, Clone)]
struct QuaternionAlgebraStructure<Field: FieldSignature> {
    base: Field,
    a: Field::Set,
    b: Field::Set,
}

#[derive(Debug, Clone)]
struct QuaternionAlgebraElement<Field: FieldSignature> {
    coeffs: [Field::Set; 4],
}

impl<Field: FieldSignature> PartialEq for QuaternionAlgebraStructure<Field> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.base.equal(&self.a, &other.a)
            && self.base.equal(&self.b, &other.b)
    }
}

impl<Field: FieldSignature> Eq for QuaternionAlgebraStructure<Field> {}

impl<Field: FieldSignature> EqSignature for QuaternionAlgebraStructure<Field> {
    fn equal(&self, a: &Self::Set, b: &Self::Set) -> bool {
        false
    }
}

impl<Field: FieldSignature> Signature for QuaternionAlgebraStructure<Field> {}

impl<Field: FieldSignature> SetSignature for QuaternionAlgebraStructure<Field> {
    type Set = QuaternionAlgebraElement<Field>;

    fn is_element(&self, _x: &Self::Set) -> bool {
        true
    }
}

impl<Field: FieldSignature> SemiRingSignature for QuaternionAlgebraStructure<Field> {
    fn zero(&self) -> Self::Set {
        QuaternionAlgebraElement {
            coeffs: std::array::from_fn(|_| self.base.zero()),
        }
    }

    fn one(&self) -> Self::Set {
        QuaternionAlgebraElement {
            coeffs: [
                self.base.one(),
                self.base.zero(),
                self.base.zero(),
                self.base.zero(),
            ],
        }
    }

    fn add(&self, a: &Self::Set, b: &Self::Set) -> Self::Set {
        let mut result = std::array::from_fn(|_| self.base.zero());
        for i in 0..4 {
            result[i] = self.base.add(&a.coeffs[i], &b.coeffs[i]);
        }
        QuaternionAlgebraElement { coeffs: result }
    }

    fn mul(&self, a: &Self::Set, b: &Self::Set) -> Self::Set {
        let (x0, x1, x2, x3) = (&a.coeffs[0], &a.coeffs[1], &a.coeffs[2], &a.coeffs[3]);
        let (y0, y1, y2, y3) = (&b.coeffs[0], &b.coeffs[1], &b.coeffs[2], &b.coeffs[3]);
        let base = &self.base;
        let a_param = &self.a;
        let b_param = &self.b;
        let base = &self.base;
        let is_char_2 = base.equal(&base.add(&base.one(), &base.one()), &base.zero());

        if is_char_2 {
            // implement characteristic 2 multiplication
            todo!("Characteristic 2 quaternion multiplication");
        } else {
            let ab = base.mul(a_param, b_param);

            let z0 = base.sub(
                &base.add(
                    &base.add(base.mul(x0, y0), base.mul(&base.mul(x1, y1), a_param)),
                    base.mul(&base.mul(x2, y2), b_param),
                ),
                base.mul(&base.mul(x3, y3), &ab),
            );

            let z1 = base.sub(
                &base.add(
                    &base.add(base.mul(x0, y1), base.mul(x1, y0)),
                    base.mul(&base.mul(x2, y3), b_param),
                ),
                base.mul(&base.mul(x3, y2), b_param),
            );

            let z2 = base.add(
                &base.sub(
                    &base.add(base.mul(x0, y2), base.mul(x2, y0)),
                    base.mul(&base.mul(x1, y3), a_param),
                ),
                base.mul(&base.mul(x3, y1), a_param),
            );

            let z3 = base.add(
                &base.add(
                    &base.sub(base.mul(x0, y3), base.mul(x2, y1)),
                    base.mul(x1, y2),
                ),
                base.mul(x3, y0),
            );

            QuaternionAlgebraElement {
                coeffs: [z0, z1, z2, z3],
            }
            // implement characteristic ≠ 2 multiplication
            // your current implementation goes here
        }
    }
}

impl<Field: FieldSignature> RingSignature for QuaternionAlgebraStructure<Field> {
    fn neg(&self, a: &Self::Set) -> Self::Set {
        QuaternionAlgebraElement {
            coeffs: std::array::from_fn(|i| self.base.neg(&a.coeffs[i])),
        }
    }
}

impl<Field: FieldSignature> QuaternionAlgebraStructure<Field> {
    pub fn i(&self) -> QuaternionAlgebraElement<Field> {
        QuaternionAlgebraElement {
            coeffs: [
                self.base.zero(),
                self.base.one(),
                self.base.zero(),
                self.base.zero(),
            ],
        }
    }

    pub fn j(&self) -> QuaternionAlgebraElement<Field> {
        QuaternionAlgebraElement {
            coeffs: [
                self.base.zero(),
                self.base.zero(),
                self.base.one(),
                self.base.zero(),
            ],
        }
    }

    pub fn k(&self) -> QuaternionAlgebraElement<Field> {
        QuaternionAlgebraElement {
            coeffs: [
                self.base.zero(),
                self.base.zero(),
                self.base.zero(),
                self.base.one(),
            ],
        }
    }

    pub fn equal_elements(
        &self,
        a: &QuaternionAlgebraElement<Field>,
        b: &QuaternionAlgebraElement<Field>,
    ) -> bool {
        (0..4).all(|i| self.base.equal(&a.coeffs[i], &b.coeffs[i]))
    }
}

#[cfg(test)]
mod tests {
    use algebraeon_nzq::Rational;

    use super::*;

    #[test]
    fn test_add_commutativity() {
        // Hamilton quaternion algebra: H = (-1, -1 / R)
        let field = RationalCanonicalStructure {};
        let h = QuaternionAlgebraStructure {
            base: field,
            a: -Rational::ONE,
            b: -Rational::ONE,
        };

        let i = h.i();
        let j = h.j();
        let i_plus_j = h.add(&i, &j);
        let j_plus_i = h.add(&j, &i);

        assert!(h.equal_elements(&i_plus_j, &j_plus_i));
    }
}
