use crate::rings::linear::matrix::MatrixStructure;

use super::*;

#[derive(Debug, Clone)]
pub struct EmbeddedAffineSubspace<
    FS: OrderedRingStructure + FieldStructure,
    SP: Borrow<AffineSpace<FS>> + Clone,
    ESP: Borrow<AffineSpace<FS>> + Clone,
> {
    //The ordered_field of ambient_space and subspace must match
    ambient_space: SP,
    embedded_space: ESP,
    /*
    these vectors must be equal in length to the affine dimension of subspace
    they define the embedding of subspace into ambient space
        if they are empty then they define the empty embedding
        if there is one vector then it defines the location of the embedded point
        if there are vectors [v0, v1, v2, ..., vn] then the embedding sends (a1, a2, ..., an) in subspace to v0 + a1*v1, v0 + a2*v2, ..., v0 + an*vn in ambient space
    */
    embedding_points: Vec<Vector<FS, SP>>,
}

impl<
        FS: OrderedRingStructure + FieldStructure,
        SP: Borrow<AffineSpace<FS>> + Clone,
        ESP: Borrow<AffineSpace<FS>> + From<AffineSpace<FS>> + Clone,
    > EmbeddedAffineSubspace<FS, SP, ESP>
{
    fn new_impl(ambient_space: SP, points: Vec<Vector<FS, SP>>) -> Result<Self, &'static str> {
        if !ambient_space
            .borrow()
            .are_points_affine_independent(points.iter().collect())
        {
            return Err("Affine embedding points must be affine independent");
        }
        let ordered_field = ambient_space.borrow().ordered_field();
        Ok(Self {
            ambient_space,
            embedded_space: AffineSpace::new_affine(ordered_field, points.len()).into(),
            embedding_points: points,
        })
    }

    pub fn new_empty(ambient_space: SP) -> Self {
        Self::new_impl(ambient_space, vec![]).unwrap()
    }
}

impl<FS: OrderedRingStructure + FieldStructure, SP: Borrow<AffineSpace<FS>> + Clone>
    EmbeddedAffineSubspace<FS, SP, AffineSpace<FS>>
{
    pub fn new(
        ambient_space: SP,
        root: Vector<FS, SP>,
        span: Vec<Vector<FS, SP>>,
    ) -> Result<Self, &'static str> {
        let mut points = vec![root.clone()];
        points.extend(span.iter().map(|vec| &root + vec));
        Self::new_impl(ambient_space, points)
    }
}

impl<
        FS: OrderedRingStructure + FieldStructure,
        SP: Borrow<AffineSpace<FS>> + Clone,
        ESP: Borrow<AffineSpace<FS>> + Clone,
    > EmbeddedAffineSubspace<FS, SP, ESP>
{
    pub fn ordered_field(&self) -> Rc<FS> {
        self.ambient_space.borrow().ordered_field()
    }

    pub fn ambient_space(&self) -> SP {
        self.ambient_space.clone()
    }

    pub fn embedded_space(&self) -> ESP {
        self.embedded_space.clone()
    }

    //Let A be the affine subspace and let S be its ambient space
    //Find an affine subspace B of S obtained by linearly extending by pt
    //Return the embeddings (f, g) where f : A -> B and g : B -> S
    pub fn extend_dimension_by_point_unsafe(
        &self,
        pt: Vector<FS, SP>,
    ) -> (
        EmbeddedAffineSubspace<FS, Rc<AffineSpace<FS>>, ESP>,
        EmbeddedAffineSubspace<FS, SP, Rc<AffineSpace<FS>>>,
    ) {
        debug_assert_eq!(self.ambient_space.borrow(), pt.ambient_space().borrow());
        debug_assert!(self.unembed_point(&pt).is_none());
        let ordered_field = self.ordered_field();

        let n = self.embedded_space.borrow().affine_dimension();
        let extended_embedded_space =
            Rc::new(AffineSpace::new_affine(ordered_field.clone(), n + 1));

        (
            EmbeddedAffineSubspace {
                ambient_space: extended_embedded_space.clone(),
                embedded_space: self.embedded_space.clone(),
                // 0, e_1, e_2, ..., e_(n-1)
                embedding_points: {
                    (0..n)
                        .map(|k| {
                            Vector::construct(extended_embedded_space.clone(), |i| {
                                if k == 0 {
                                    ordered_field.zero()
                                } else {
                                    let j = k - 1;
                                    match i == j {
                                        true => ordered_field.one(),
                                        false => ordered_field.zero(),
                                    }
                                }
                            })
                        })
                        .collect()
                },
            },
            EmbeddedAffineSubspace {
                ambient_space: self.ambient_space.clone(),
                embedded_space: extended_embedded_space.clone(),
                embedding_points: {
                    let mut pts = self.embedding_points.clone();
                    pts.push(pt);
                    pts
                },
            },
        )
    }

    pub fn get_root_and_span(&self) -> Option<(Vector<FS, SP>, Vec<Vector<FS, SP>>)> {
        let mut points = self.embedding_points.iter();
        let root = points.next()?;
        let span = points.map(|pt| pt - root).collect::<Vec<_>>();
        Some((root.clone(), span))
    }

    pub fn embed_point(&self, pt: &Vector<FS, ESP>) -> Vector<FS, SP> {
        assert_eq!(pt.ambient_space().borrow(), self.embedded_space.borrow());
        let (root, span) = self.get_root_and_span().unwrap(); //pt exists in the embedded space, so the embedded space is non-empty, so has a root and span
        let mut total = root.clone();
        for (i, vec) in span.iter().enumerate() {
            total += &vec.scalar_mul(pt.coordinate(i));
        }
        total
    }

    pub fn unembed_point(&self, pt: &Vector<FS, SP>) -> Option<Vector<FS, ESP>> {
        assert_eq!(pt.ambient_space().borrow(), self.ambient_space.borrow());
        match self.get_root_and_span() {
            Some((root, span)) => {
                //solve root + x * basis = v for x
                let y = (pt - &root).into_col();
                let basis_matrix = self
                    .ambient_space
                    .borrow()
                    .cols_from_vectors(span.iter().collect());
                let x = MatrixStructure::new(self.ambient_space.borrow().ordered_field())
                    .col_solve(&basis_matrix, y);
                Some(vector_from_col(self.embedded_space(), &x?))
            }
            None => None,
        }
    }

    // pub fn embed_vector(&self, v: &Vector<FS, ESP>) -> Vector<FS, SP> {
    //     match &self.embedding {
    //         AffineSubspaceEmbedding::Empty { .. } => panic!(),
    //         AffineSubspaceEmbedding::NonEmpty {
    //             subspace,
    //             root,
    //             basis,
    //         } => {
    //             assert_eq!(v.ambient_space().borrow(), subspace.borrow());
    //             let mut total = Vector::zero(self.ambient_space.clone());
    //             for (i, b) in basis.iter().enumerate() {
    //                 total += &b.scalar_mul(v.coordinate(i));
    //             }
    //             total
    //         }
    //     }
    // }

    // pub fn unembed_vector(&self, v: &Vector<FS, SP>) -> Option<Vector<FS, ESP>> {
    //     assert_eq!(v.ambient_space().borrow(), self.ambient_space.borrow());
    //     todo!()
    // }
}

pub fn compose_affine_embeddings<
    FS: OrderedRingStructure + FieldStructure,
    SPA: Borrow<AffineSpace<FS>> + Clone,
    SPB: Borrow<AffineSpace<FS>> + Clone,
    SPC: Borrow<AffineSpace<FS>> + Clone,
>(
    a_to_b: EmbeddedAffineSubspace<FS, SPB, SPA>,
    b_to_c: EmbeddedAffineSubspace<FS, SPC, SPB>,
) -> EmbeddedAffineSubspace<FS, SPC, SPA> {
    todo!() // call b_to_c.embed on the defining points of a_to_b
}

#[cfg(test)]
mod tests {
    use malachite_q::Rational;

    use crate::rings::structure::StructuredType;

    use super::*;

    #[test]
    fn make_affine_subspace() {
        let space = AffineSpace::new_linear(Rational::structure(), 3);
        let v1 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(1), Rational::from(1)],
        );
        let v2 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(0), Rational::from(0)],
        );
        let v3 = Vector::new(
            &space,
            vec![Rational::from(0), Rational::from(1), Rational::from(0)],
        );
        let s = EmbeddedAffineSubspace::new(&space, v1, vec![v2, v3]);
        s.unwrap();

        let space = AffineSpace::new_linear(Rational::structure(), 3);
        let v1 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(1), Rational::from(1)],
        );
        let v2 = Vector::new(
            &space,
            vec![Rational::from(1), Rational::from(2), Rational::from(0)],
        );
        let v3 = Vector::new(
            &space,
            vec![Rational::from(-2), Rational::from(-4), Rational::from(0)],
        );
        let s = EmbeddedAffineSubspace::new(&space, v1, vec![v2, v3]);
        assert!(s.is_err());
    }

    #[test]
    fn affine_subspace_embed_and_unembed() {
        //1d embedded in 2d
        {
            let plane = AffineSpace::new_linear(Rational::structure(), 2);
            //the line x + y = 2
            let line = EmbeddedAffineSubspace::new(
                &plane,
                Vector::new(&plane, vec![Rational::from(1), Rational::from(1)]),
                vec![Vector::new(
                    &plane,
                    vec![Rational::from(1), Rational::from(-1)],
                )],
            )
            .unwrap();

            assert_eq!(
                line.embed_point(&Vector::new(
                    line.embedded_space(),
                    vec![Rational::from(-3)],
                )),
                Vector::new(&plane, vec![Rational::from(-2), Rational::from(4)])
            );

            assert_eq!(
                line.unembed_point(&Vector::new(
                    &plane,
                    vec![Rational::from(-1), Rational::from(3)],
                )),
                Some(Vector::new(line.embedded_space(), vec![Rational::from(-2)],))
            );

            assert_eq!(
                line.unembed_point(&Vector::new(
                    &plane,
                    vec![Rational::from(1), Rational::from(2)],
                )),
                None
            );
        }

        //2d embedded in 3d
        {
            let space = AffineSpace::new_linear(Rational::structure(), 3);
            let plane = EmbeddedAffineSubspace::new(
                &space,
                Vector::new(
                    &space,
                    vec![Rational::from(3), Rational::from(1), Rational::from(2)],
                ),
                vec![
                    Vector::new(
                        &space,
                        vec![Rational::from(4), Rational::from(2), Rational::from(1)],
                    ),
                    Vector::new(
                        &space,
                        vec![Rational::from(1), Rational::from(-1), Rational::from(2)],
                    ),
                ],
            )
            .unwrap();

            assert_eq!(
                plane.embed_point(&Vector::new(
                    plane.embedded_space(),
                    vec![Rational::from(-3), Rational::from(2)],
                )),
                Vector::new(
                    &space,
                    vec![Rational::from(-7), Rational::from(-7), Rational::from(3)]
                )
            );

            assert_eq!(
                plane.unembed_point(&Vector::new(
                    &space,
                    vec![Rational::from(0), Rational::from(-2), Rational::from(3)],
                )),
                Some(Vector::new(
                    plane.embedded_space(),
                    vec![Rational::from(-1), Rational::from(1)],
                ))
            );

            assert_eq!(
                plane.unembed_point(&Vector::new(
                    &space,
                    vec![Rational::from(1), Rational::from(2), Rational::from(2)],
                )),
                None
            );
        }
    }
}
