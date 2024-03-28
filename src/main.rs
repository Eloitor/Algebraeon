#![allow(dead_code, warnings, unused)]

#[macro_use]
extern crate glium;

pub mod drawing;
pub mod finite_group_tables;
pub mod geometry;
pub mod groups;
pub mod numbers;
pub mod rings;
pub mod sets;

/*
fn main() {
    fn random_point(dim: usize, rad: f64) -> Vector {
        let mut rng = rand::thread_rng();
        Vector::new(
            (0..dim)
                .map(|_i| QQ.from_f64_approx(rng.gen_range(-rad..rad)))
                .collect(),
        )
    }

    let shape = convex_hull(
        2,
        (0..24)
            .map(|i| random_point(2, f64::sqrt((i + 1) as f64)))
            .collect(),
    )
    .as_simplicial_complex();

    // let shape = convex_hull(
    //     2,
    //     vec![
    //         Point::new(vec![Rational::from(1), Rational::from(1)]),
    //         Point::new(vec![Rational::from(0), Rational::from(0)]),
    //         Point::new(vec![Rational::from(2), Rational::from(2)]),
    //     ],
    // )
    // .as_simplicial_complex();

    let a = shape;
    let b = a.clone().simplify();
    // let shape = shape.as_shape();

    // let (a, b) = shape.interior_and_boundary();
    // a.check().unwrap();
    // b.check().unwrap();

    drawing::canvas2d::Shape2dCanvas::run(|canvas| {
        canvas.draw_shape(&a.as_shape(), (1.0, 0.0, 0.0));
        canvas.draw_shape(&b.as_shape(), (0.0, 1.0, 0.0));
    });


    //where I left with the simplicial stuff:
    //simplification of simplicial complex is done
    //combining simplicial complexes is not done
    //the plan is to recursively compare pairs of simplexes and refine them until every pair intersects trivially
}
*/

// fn todo() {
//     // let s = NaturalPrimeGenerator::new();
//     // for p in s {
//     //     println!("{}", p);
//     // }

//     // let x = Integer::from(10000);
//     // let f = ZZ.factor(&x);
//     // println!("{:?}", f);

//     let f = Polynomial::from_coeffs(vec![
//         Integer::from(1),
//         Integer::from(0),
//         Integer::from(0),
//         Integer::from(0),
//         Integer::from(0),
//         Integer::from(1),
//     ]);
//     let roots = f.all_complex_roots();

//     for root in roots.iter() {
//         println!("{:?}", root);
//     }

//     let a = ComplexAlgebraic::sum(roots.iter().collect());

//     println!("{:?}", a);
// }

fn main() {
    use std::marker::PhantomData;
    use std::str::FromStr;
    use std::task::Poll;

    use crate::drawing::canvas2d::*;
    use crate::drawing::Canvas;
    use crate::geometry::convex_simplicial_complex::*;
    use crate::geometry::vector::*;
    use crate::geometry::*;
    use crate::groups::group::*;
    use crate::groups::permutation::*;
    use crate::rings::number::algebraic::number_field::*;
    use crate::rings::number::modulo::*;
    use crate::rings::polynomial::polynomial::*;
    use crate::rings::ring_structure::cannonical::*;
    use crate::rings::ring_structure::quotient::*;
    use crate::rings::ring_structure::structure::*;
    use crate::rings::structure::*;
    use malachite_nz::integer::Integer;
    use malachite_nz::natural::Natural;
    use malachite_q::Rational;
    use crate::rings::number::algebraic::isolated_roots::*;
    use crate::rings::linear::matrix::*;


    use crate::rings::polynomial::polynomial::*;

    let i = &ComplexAlgebraic::i().into_ring();

        let mat = Matrix::from_rows(vec![
            vec![
                (1 + 0 * i).into_set(),
                (1 * i).into_set(),
            ],
            vec![
                (1 + 0 * i).into_set(),
                (1 + 0 * i).into_set(),
            ],

        ]);
        mat.pprint();
        mat.gram_schmidt_col_orthogonalization().pprint();

        let mat = Matrix::from_rows(vec![
            vec![
                (-2 + 2 * i).into_set(),
                (7 + 3 * i).into_set(),
                (7 + 3 * i).into_set(),
                (-5 + 4 * i).into_set(),
            ],
            vec![
                (3 + 3 * i).into_set(),
                (-2 + 4 * i).into_set(),
                (6 + 2 * i).into_set(),
                (-1 + 4 * i).into_set(),
            ],
            vec![
                (2 + 2 * i).into_set(),
                (8 + 0 * i).into_set(),
                (-9 + 1 * i).into_set(),
                (-7 + 5 * i).into_set(),
            ],
            vec![
                (8 + 2 * i).into_set(),
                (-9 + 0 * i).into_set(),
                (6 + 3 * i).into_set(),
                (-4 + 4 * i).into_set(),
            ],
        ]);
        mat.pprint();
        mat.clone().gram_schmidt_col_orthogonalization().pprint();
        mat.presentation_matrix().unwrap().pprint();
        mat.presentation_matrix().unwrap().smith_algorithm().1.pprint();
}
