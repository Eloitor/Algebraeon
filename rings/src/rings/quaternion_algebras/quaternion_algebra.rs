#[derive(Debug, Clone)]
struct QuaternionAlgebraStructure<Field: FieldSignature> {
    base: Field,
    a: Field::Set,
    b: Field::Set,
}

#[derive(Debug, Clone)]
struct QuaternionAlgebraElement {}

impl<Field: FieldSignature> PartialEq for QuaternionAlgebraStructure<Field> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.base.equal(&self.a, &other.a)
            && self.base.equal(&self.b, &other.b)
    }
}

impl<Field: FieldSignature> Eq for QuaternionAlgebraStructure<Field> {}

impl<Field: FieldSignature> Signature for QuaternionAlgebraStructure<Field> {}

impl<Field: FieldSignature> SetSignature for QuaternionAlgebraStructure<Field> {
    type Set = QuaternionAlgebraElement;

    fn is_element(&self, x: &Self::Set) -> bool {
        true
    }
}
