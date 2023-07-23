#![allow(dead_code)]

use super::ring::*;

#[derive(Debug)]
pub enum MatOppErr {
    DimMissmatch,
    InvalidIndex,
    NotSquare,
}

#[derive(Debug, Clone)]
pub struct Matrix<R: ComRing> {
    dim1: usize,
    dim2: usize,
    transpose: bool,
    elems: Vec<R>, //length self.rows * self.cols. row r and column c is index c + r * self.cols
}

impl<R: ComRing> PartialEq for Matrix<R> {
    fn eq(&self, other: &Self) -> bool {
        let rows = self.rows();
        let cols = self.cols();
        if rows != other.rows() {
            false
        } else if cols != other.cols() {
            false
        } else {
            for c in 0..cols {
                for r in 0..rows {
                    if self.at(r, c).unwrap() != other.at(r, c).unwrap() {
                        return false;
                    }
                }
            }
            true
        }
    }
}

impl<R: ComRing> Matrix<R> {
    fn check_invariants(&self) -> Result<(), &'static str> {
        if self.elems.len() != self.dim1 * self.dim2 {
            return Err("matrix entries has the wrong length");
        }
        Ok(())
    }

    pub fn from_rows(rows_elems: Vec<Vec<R>>) -> Self {
        let rows = rows_elems.len();
        assert!(rows >= 1);
        let cols = rows_elems[0].len();
        for r in 1..rows {
            assert_eq!(rows_elems[r].len(), cols);
        }
        let mut mat = Self::zero(rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                *mat.at_mut(r, c).unwrap() = rows_elems[r][c].clone()
            }
        }
        mat
    }

    pub fn rows(&self) -> usize {
        match self.transpose {
            false => self.dim1,
            true => self.dim2,
        }
    }

    pub fn cols(&self) -> usize {
        match self.transpose {
            false => self.dim2,
            true => self.dim1,
        }
    }

    fn rc_to_idx(&self, r: usize, c: usize) -> usize {
        match self.transpose {
            false => c + r * self.dim2,
            true => r + c * self.dim2,
        }
    }

    pub fn at(&self, r: usize, c: usize) -> Result<&R, MatOppErr> {
        if r >= self.rows() {
            Err(MatOppErr::InvalidIndex)
        } else if c >= self.cols() {
            Err(MatOppErr::InvalidIndex)
        } else {
            let idx = self.rc_to_idx(r, c);
            Ok(&self.elems[idx])
        }
    }

    pub fn at_mut(&mut self, r: usize, c: usize) -> Result<&mut R, MatOppErr> {
        if r >= self.rows() {
            Err(MatOppErr::InvalidIndex)
        } else if c >= self.cols() {
            Err(MatOppErr::InvalidIndex)
        } else {
            let idx = self.rc_to_idx(r, c);
            Ok(&mut self.elems[idx])
        }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        let mut elems = Vec::with_capacity(rows * cols);
        for _i in 0..rows * cols {
            elems.push(R::zero());
        }
        Self {
            dim1: rows,
            dim2: cols,
            transpose: false,
            elems,
        }
    }

    pub fn ident(n: usize) -> Self {
        let mut elems = Vec::with_capacity(n * n);
        for r in 0..n {
            for c in 0..n {
                match r == c {
                    true => elems.push(R::one()),
                    false => elems.push(R::zero()),
                }
            }
        }
        Self {
            dim1: n,
            dim2: n,
            transpose: false,
            elems,
        }
    }
}

impl<R: ComRing + std::fmt::Display> Matrix<R> {
    pub fn pprint(&self) {
        let mut str_rows = vec![];
        for r in 0..self.rows() {
            str_rows.push(vec![]);
            for c in 0..self.cols() {
                str_rows[r].push(self.at(r, c).unwrap().to_string());
            }
        }
        let cols_widths: Vec<usize> = (0..self.cols())
            .map(|c| {
                (0..self.rows())
                    .map(|r| str_rows[r][c].len())
                    .fold(0usize, |a, b| a.max(b))
            })
            .collect();

        for r in 0..self.rows() {
            for c in 0..self.cols() {
                while str_rows[r][c].len() < cols_widths[c] {
                    str_rows[r][c].push(' ');
                }
            }
        }
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                if c != 0 {
                    print!("    ");
                }
                print!("{}", str_rows[r][c]);
            }
            print!("\n");
        }
    }
}

impl<R: ComRing> Matrix<R> {
    pub fn transpose(mut self) -> Self {
        self.transpose_mut();
        self
    }

    pub fn transpose_ref(&self) -> Self {
        self.clone().transpose()
    }

    pub fn transpose_mut(&mut self) {
        self.transpose = !self.transpose;
    }

    pub fn add_mut(&mut self, other: &Self) -> Result<(), MatOppErr> {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            Err(MatOppErr::DimMissmatch)
        } else {
            let rows = self.rows();
            let cols = self.cols();
            println!();
            for c in 0..cols {
                for r in 0..rows {
                    println!(
                        "{} {} {:?} {:?}",
                        r,
                        c,
                        self.at(r, c),
                        other.rc_to_idx(r, c)
                    );
                    self.at_mut(r, c).unwrap().add_mut(other.at(r, c).unwrap())
                }
            }
            Ok(())
        }
    }

    pub fn add(mut a: Self, b: Self) -> Result<Self, MatOppErr> {
        match a.add_mut(&b) {
            Ok(()) => Ok(a),
            Err(e) => Err(e),
        }
    }

    pub fn add_ref(mut a: Self, b: &Self) -> Result<Self, MatOppErr> {
        match a.add_mut(b) {
            Ok(()) => Ok(a),
            Err(e) => Err(e),
        }
    }

    pub fn add_refs(a: &Self, b: &Self) -> Result<Self, MatOppErr> {
        let mut new_a = a.clone();
        match new_a.add_mut(&b) {
            Ok(()) => Ok(new_a),
            Err(e) => Err(e),
        }
    }

    // pub fn mul(a: Self, b: Self) -> Result<Self, MatOppErr> {
    //     Self::mul_refs(&a, &b)
    // }

    // pub fn mul_lref(a: &Self, b: Self) -> Result<Self, MatOppErr> {
    //     Self::mul_refs(a, &b)
    // }

    // pub fn mul_rref(a: Self, b: &Self) -> Result<Self, MatOppErr> {
    //     Self::mul_refs(&a, b)
    // }

    pub fn mul_refs(a: &Self, b: &Self) -> Result<Self, MatOppErr> {
        let mids = a.cols();
        if mids != b.rows() {
            return Err(MatOppErr::DimMissmatch);
        }
        let rows = a.rows();
        let cols = b.cols();
        let mut s = Matrix::<R>::zero(rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                for m in 0..mids {
                    s.at_mut(r, c)
                        .unwrap()
                        .add_mut(&R::mul_refs(a.at(r, m).unwrap(), b.at(m, c).unwrap()));
                }
            }
        }
        Ok(s)
    }

    pub fn det_naive(&self) -> Result<R, MatOppErr> {
        let n = self.dim1;
        if n != self.dim2 {
            Err(MatOppErr::NotSquare)
        } else {
            let mut det = R::zero();
            for perm in super::super::sets::permutations::all_perms(n) {
                let mut prod = R::one();
                for k in 0..n {
                    prod.mul_mut(self.at(k, perm.call(k).unwrap()).unwrap());
                }
                if !perm.sign() {
                    prod.neg_mut();
                }
                det.add_mut(&prod);
            }
            Ok(det)
        }
    }
}

#[derive(Debug)]
enum ElementaryRowOppPID<R: GCDDomain> {
    //swap distinct rows
    Swap(usize, usize),
    //multiply a row by a unit
    UnitMul {
        row: usize,
        unit: R,
    },
    //row(i) -> row(i) + x*row(j)
    AddRowMul {
        i: usize,
        j: usize,
        x: R,
    },
    //apply invertible row operations to two rows
    // /a b\
    // \c d/
    //such that ad-bc is a unit
    TwoInv {
        i: usize,
        j: usize,
        a: R,
        b: R,
        c: R,
        d: R,
    },
}

impl<R: GCDDomain> ElementaryRowOppPID<R> {
    fn check_invariants(&self) -> Result<(), &'static str> {
        match self {
            ElementaryRowOppPID::Swap(i, j) => {
                if i == j {
                    return Err("can only swap distinct rows");
                }
            }
            ElementaryRowOppPID::AddRowMul { i, j, x: _x } => {
                if i == j {
                    return Err("can only add a multiple of a row to a distinct row");
                }
            }
            ElementaryRowOppPID::UnitMul { row: _row, unit } => {
                if !unit.clone().is_unit() {
                    return Err("can only multiply a row by a unit");
                }
            }
            ElementaryRowOppPID::TwoInv { i, j, a, b, c, d } => {
                if i == j {
                    return Err("rows must be distinct");
                }
                let m = Matrix {
                    dim1: 2,
                    dim2: 2,
                    transpose: false,
                    elems: vec![a.clone(), b.clone(), c.clone(), d.clone()],
                };
                if !m.det_naive().unwrap().is_unit() {
                    return Err("can only apply an invertible row opperation to two rows");
                }
            }
        }
        Ok(())
    }

    fn apply(&self, m: &mut Matrix<R>) {
        debug_assert!(self.check_invariants().is_ok());
        match self {
            // /0 1\
            // \1 0/
            ElementaryRowOppPID::Swap(i, j) => {
                for col in 0..m.cols() {
                    let tmp = m.at(*i, col).unwrap().clone();
                    *m.at_mut(*i, col).unwrap() = m.at(*j, col).unwrap().clone();
                    *m.at_mut(*j, col).unwrap() = tmp;
                }
            }
            // /1 x\
            // \0 1/
            ElementaryRowOppPID::AddRowMul { i, j, x } => {
                for col in 0..m.cols() {
                    let offset = R::mul_refs(m.at(*j, col).unwrap(), x);
                    m.at_mut(*i, col).unwrap().add_mut(&offset)
                }
            }
            // /u 0\
            // \0 1/
            ElementaryRowOppPID::UnitMul { row, unit } => {
                for col in 0..m.cols() {
                    m.at_mut(*row, col).unwrap().mul_mut(unit)
                }
            }
            // /a b\
            // \c d/
            ElementaryRowOppPID::TwoInv { i, j, a, b, c, d } => {
                for col in 0..m.cols() {
                    // tmp = c*row(i) + d*row(j)
                    let tmp = R::add(
                        R::mul_refs(c, m.at(*i, col).unwrap()),
                        R::mul_refs(d, m.at(*j, col).unwrap()),
                    );
                    // row(i) = a*row(i) + b*row(j)
                    *m.at_mut(*i, col).unwrap() = R::add(
                        R::mul_refs(a, m.at(*i, col).unwrap()),
                        R::mul_refs(b, m.at(*j, col).unwrap()),
                    );
                    // row(j) = tmp
                    *m.at_mut(*j, col).unwrap() = tmp;
                }
            }
        }
    }
}

impl<R: GCDDomain + std::fmt::Display> Matrix<R> {
    //TODO: replace with over a pid
    //if A:=self return (H, U, pivots) such that
    //H is in row hermite normal form
    //U is invertible
    //H=UA
    //pivots[r] is the column of the rth pivot and pivots.len() == rank(A)
    pub fn row_hermite_algorithm(mut self) -> (Self, Self, Vec<usize>) {
        //build up U by applying row opps to the identity as we go
        let mut u = Self::ident(self.rows());
        let mut pivs = vec![];

        let (mut pr, mut pc) = (0, 0);
        'pivot_loop: while pr < self.rows() {
            //find the next pivot row
            while self.at(pr, pc).unwrap() == &R::zero() {
                pc += 1;
                if pc == self.cols() {
                    break 'pivot_loop;
                }
            }
            debug_assert_ne!(self.at(pr, pc).unwrap(), &R::zero());
            pivs.push(pc);

            if pr + 1 < self.rows() {
                //reduce everything below the pivot to zero
                for r in pr + 1..self.rows() {
                    let a = self.at(pr, pc).unwrap();
                    let b = self.at(r, pc).unwrap();
                    let (d, x, y) = R::xgcd(a.clone(), b.clone());
                    debug_assert_eq!(R::add(R::mul_refs(&x, a), R::mul_refs(&y, b)), d);
                    // perform the following row opps on self
                    // / x  -b/d \
                    // \ y   a/d /
                    let row_opp = ElementaryRowOppPID::TwoInv {
                        i: pr,
                        j: r,
                        a: x,
                        b: y,
                        //TODO: compute b/d and a/d at the same time d is computed?
                        c: R::div(b.clone(), d.clone()).unwrap().neg(),
                        d: R::div(a.clone(), d.clone()).unwrap(),
                    };
                    //this will implicitly put the pivot into fav assoc form because that is what the gcd returns
                    row_opp.apply(&mut self);
                    row_opp.apply(&mut u);
                }
            } else {
                //explicitly put the pivot into fav assoc form
                let (unit, _assoc) = self.at(pr, pc).unwrap().factor_fav_assoc_ref().unwrap();
                let row_opp = ElementaryRowOppPID::UnitMul {
                    row: pr,
                    unit: unit.inv().unwrap(),
                };
                //this will implicitly put the pivot into fav assoc form because that is what the gcd returns
                row_opp.apply(&mut self);
                row_opp.apply(&mut u);
            }

            //should have eliminated everything below the pivot
            for r in pr + 1..self.rows() {
                debug_assert_eq!(self.at(r, pc).unwrap(), &R::zero());
            }
            pr += 1;
        }

        (self, u, pivs)
    }

    pub fn col_hermite_algorithm(self) -> (Self, Self, Vec<usize>) {
        let (rh, ru, pivs) = self.transpose().row_hermite_algorithm();
        (rh.transpose(), ru.transpose(), pivs)
    }

    pub fn smith_algorithm(&self) -> (Self, Self, Self) {
        todo!();
    }
}

impl<R: EuclideanDomain + FavoriteAssociate + std::fmt::Display> Matrix<R> {
    //if A:=self return (H, U, pivots) such that
    //H is in row reduced hermite normal form
    //U is invertible
    //H=UA
    //pivots[r] is the column of the rth pivot and pivots.len() == rank(A)
    pub fn row_reduced_hermite_algorithm(self) -> (Self, Self, Vec<usize>) {
        let (mut h, mut u, pivs) = self.row_hermite_algorithm();

        for (pr, pc) in pivs.iter().enumerate().rev() {
            for r in 0..pr {
                //reduce h[r, pc] so that it has norm less than h[pr, pc]
                let a = h.at(r, *pc).unwrap();
                let b = h.at(pr, *pc).unwrap();
                //a = b*q + r
                let q = R::quo_refs(a, b).unwrap();
                let row_opp = ElementaryRowOppPID::AddRowMul {
                    i: r,
                    j: pr,
                    x: q.neg(),
                };
                row_opp.apply(&mut h);
                row_opp.apply(&mut u);
            }
        }

        (h, u, pivs)
    }

    pub fn col_reduced_hermite_algorithm(self) -> (Self, Self, Vec<usize>) {
        let (rh, ru, pivs) = self.transpose().row_reduced_hermite_algorithm();
        (rh.transpose(), ru.transpose(), pivs)
    }
}

#[cfg(test)]
mod tests {
    use malachite_nz::integer::Integer;

    use super::*;

    #[test]
    fn invariant() {
        let m: Matrix<Integer> = Matrix {
            dim1: 3,
            dim2: 4,
            transpose: false,
            elems: vec![
                Integer::from(1),
                Integer::from(2),
                Integer::from(3),
                Integer::from(4),
                Integer::from(5),
            ],
        };
        match m.check_invariants() {
            Ok(()) => panic!(),
            Err(_) => {}
        }

        let m: Matrix<Integer> = Matrix {
            dim1: 2,
            dim2: 3,
            transpose: true,
            elems: vec![
                Integer::from(1),
                Integer::from(2),
                Integer::from(3),
                Integer::from(4),
                Integer::from(5),
                Integer::from(6),
            ],
        };
        m.check_invariants().unwrap();
    }

    #[test]
    fn transpose_eq() {
        let a: Matrix<Integer> = Matrix {
            dim1: 2,
            dim2: 2,
            transpose: false,
            elems: vec![
                Integer::from(0),
                Integer::from(1),
                Integer::from(2),
                Integer::from(3),
            ],
        };
        a.check_invariants().unwrap();

        let b: Matrix<Integer> = Matrix {
            dim1: 2,
            dim2: 2,
            transpose: true,
            elems: vec![
                Integer::from(0),
                Integer::from(2),
                Integer::from(1),
                Integer::from(3),
            ],
        };
        b.check_invariants().unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn add() {
        {
            let mut a: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(4),
                    Integer::from(5),
                    Integer::from(6),
                ],
            };
            a.check_invariants().unwrap();

            let b: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(1),
                    Integer::from(2),
                ],
            };
            b.check_invariants().unwrap();

            let c: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(2),
                    Integer::from(4),
                    Integer::from(4),
                    Integer::from(6),
                    Integer::from(6),
                    Integer::from(8),
                ],
            };
            c.check_invariants().unwrap();

            a.add_mut(&b).unwrap();

            assert_eq!(a, c);
        }

        {
            let mut a: Matrix<Integer> = Matrix {
                dim1: 3,
                dim2: 2,
                transpose: false,
                elems: vec![
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(4),
                    Integer::from(5),
                    Integer::from(6),
                ],
            };
            a.check_invariants().unwrap();

            let b: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: true,
                elems: vec![
                    Integer::from(10),
                    Integer::from(20),
                    Integer::from(30),
                    Integer::from(40),
                    Integer::from(50),
                    Integer::from(60),
                ],
            };
            b.check_invariants().unwrap();

            let c: Matrix<Integer> = Matrix {
                dim1: 3,
                dim2: 2,
                transpose: false,
                elems: vec![
                    Integer::from(11),
                    Integer::from(42),
                    Integer::from(23),
                    Integer::from(54),
                    Integer::from(35),
                    Integer::from(66),
                ],
            };
            c.check_invariants().unwrap();

            a.add_mut(&b).unwrap();

            assert_eq!(a, c);
        }

        {
            let mut a: Matrix<Integer> = Matrix {
                dim1: 3,
                dim2: 2,
                transpose: false,
                elems: vec![
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(4),
                    Integer::from(5),
                    Integer::from(6),
                ],
            };
            a.check_invariants().unwrap();

            let b: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(1),
                    Integer::from(2),
                ],
            };
            b.check_invariants().unwrap();

            match a.add_mut(&b) {
                Ok(()) => panic!(),
                Err(MatOppErr::DimMissmatch) => {}
                Err(_) => panic!(),
            }
        }

        {
            let a: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(4),
                    Integer::from(5),
                    Integer::from(6),
                ],
            };
            a.check_invariants().unwrap();

            let b: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(1),
                    Integer::from(2),
                    Integer::from(1),
                    Integer::from(2),
                ],
            };
            b.check_invariants().unwrap();

            let c: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(2),
                    Integer::from(4),
                    Integer::from(4),
                    Integer::from(6),
                    Integer::from(6),
                    Integer::from(8),
                ],
            };
            c.check_invariants().unwrap();

            assert_eq!(Matrix::<Integer>::add(a, b).unwrap(), c);
        }
    }

    #[test]
    fn mul() {
        {
            let a: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 4,
                transpose: false,
                elems: vec![
                    Integer::from(3),
                    Integer::from(2),
                    Integer::from(1),
                    Integer::from(5),
                    Integer::from(9),
                    Integer::from(1),
                    Integer::from(3),
                    Integer::from(0),
                ],
            };
            a.check_invariants().unwrap();

            let b: Matrix<Integer> = Matrix {
                dim1: 4,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(2),
                    Integer::from(9),
                    Integer::from(0),
                    Integer::from(1),
                    Integer::from(3),
                    Integer::from(5),
                    Integer::from(2),
                    Integer::from(4),
                    Integer::from(7),
                    Integer::from(8),
                    Integer::from(1),
                    Integer::from(5),
                ],
            };
            b.check_invariants().unwrap();

            let c: Matrix<Integer> = Matrix {
                dim1: 2,
                dim2: 3,
                transpose: false,
                elems: vec![
                    Integer::from(50),
                    Integer::from(42),
                    Integer::from(42),
                    Integer::from(25),
                    Integer::from(96),
                    Integer::from(26),
                ],
            };
            c.check_invariants().unwrap();

            assert_eq!(Matrix::<Integer>::mul_refs(&a, &b).unwrap(), c);
        }
    }

    #[test]
    fn det_naive() {
        let m: Matrix<Integer> = Matrix::from_rows(vec![
            vec![Integer::from(1), Integer::from(3), Integer::from(2)],
            vec![Integer::from(-3), Integer::from(-1), Integer::from(-3)],
            vec![Integer::from(2), Integer::from(3), Integer::from(1)],
        ]);
        assert_eq!(m.det_naive().unwrap(), Integer::from(-15));
    }

    #[test]
    fn hermite_algorithm() {
        for a in vec![
            Matrix::from_rows(vec![
                vec![
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(6),
                    Integer::from(2),
                ],
                vec![
                    Integer::from(5),
                    Integer::from(6),
                    Integer::from(1),
                    Integer::from(6),
                ],
                vec![
                    Integer::from(8),
                    Integer::from(3),
                    Integer::from(1),
                    Integer::from(1),
                ],
            ]),
            Matrix::from_rows(vec![
                vec![
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(6),
                    Integer::from(2),
                ],
                vec![
                    Integer::from(5),
                    Integer::from(6),
                    Integer::from(1),
                    Integer::from(6),
                ],
                vec![
                    Integer::from(8),
                    Integer::from(3),
                    Integer::from(1),
                    Integer::from(1),
                ],
            ])
            .transpose(),
            Matrix::from_rows(vec![
                vec![
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(5),
                    Integer::from(2),
                ],
                vec![
                    Integer::from(5),
                    Integer::from(6),
                    Integer::from(11),
                    Integer::from(6),
                ],
                vec![
                    Integer::from(8),
                    Integer::from(3),
                    Integer::from(11),
                    Integer::from(1),
                ],
            ]),
        ] {
            println!();
            println!("hermite reduced row algorithm for");
            a.pprint();
            let (h, u, pivs) = a.clone().row_reduced_hermite_algorithm();
            println!("H =");
            h.pprint();
            println!("pivs = {:?}", pivs);
            println!("U =");
            u.pprint();
            assert_eq!(h, Matrix::<Integer>::mul_refs(&u, &a).unwrap());
            for (pr, pc) in pivs.iter().enumerate() {
                assert!(h.at(pr, *pc).unwrap() != &Integer::zero());
                for r in 0..h.rows() {
                    if r > pr {
                        assert_eq!(h.at(r, *pc).unwrap(), &Integer::zero());
                    } else if r == pr {
                        let (_unit, assoc) =
                            h.at(r, *pc).unwrap().clone().factor_fav_assoc().unwrap();
                        assert_eq!(&assoc, h.at(r, *pc).unwrap());
                    } else {
                        assert!(h.at(r, *pc).unwrap().norm() < h.at(pr, *pc).unwrap().norm());
                    }
                }
            }

            println!();
            println!("hermite reduced col algorithm for");
            a.pprint();
            let (h, u, pivs) = a.clone().col_reduced_hermite_algorithm();
            println!("H =");
            h.pprint();
            println!("pivs = {:?}", pivs);
            println!("U =");
            u.pprint();

            assert_eq!(h, Matrix::<Integer>::mul_refs(&a, &u).unwrap());
            for (pc, pr) in pivs.iter().enumerate() {
                assert!(h.at(*pr, pc).unwrap() != &Integer::zero());
                for c in 0..h.cols() {
                    if c > pc {
                        assert_eq!(h.at(*pr, c).unwrap(), &Integer::zero());
                    } else if c == pc {
                        let (_unit, assoc) =
                            h.at(*pr, c).unwrap().clone().factor_fav_assoc().unwrap();
                        assert_eq!(&assoc, h.at(*pr, c).unwrap());
                    } else {
                        assert!(h.at(*pr, c).unwrap().norm() < h.at(*pr, pc).unwrap().norm());
                    }
                }
            }

            //integer reduced hermite normal form is unique, so we can fully check an example
            let a = Matrix::from_rows(vec![
                vec![
                    Integer::from(2),
                    Integer::from(3),
                    Integer::from(6),
                    Integer::from(2),
                ],
                vec![
                    Integer::from(5),
                    Integer::from(6),
                    Integer::from(1),
                    Integer::from(6),
                ],
                vec![
                    Integer::from(8),
                    Integer::from(3),
                    Integer::from(1),
                    Integer::from(1),
                ],
            ]);

            let expected_h = Matrix::from_rows(vec![
                vec![
                    Integer::from(1),
                    Integer::from(0),
                    Integer::from(50),
                    Integer::from(-11),
                ],
                vec![
                    Integer::from(0),
                    Integer::from(3),
                    Integer::from(28),
                    Integer::from(-2),
                ],
                vec![
                    Integer::from(0),
                    Integer::from(0),
                    Integer::from(61),
                    Integer::from(-13),
                ],
            ]);

            let expected_u = Matrix::from_rows(vec![
                vec![Integer::from(9), Integer::from(-5), Integer::from(1)],
                vec![Integer::from(5), Integer::from(-2), Integer::from(0)],
                vec![Integer::from(11), Integer::from(-6), Integer::from(1)],
            ]);

            let (h, u, pivs) = a.clone().row_reduced_hermite_algorithm();

            assert_eq!(h, expected_h);
            assert_eq!(u, expected_u);
            assert_eq!(pivs, vec![0, 1, 2]);
        }
    }

    #[test]
    fn reduced_hermite_algorithm() {
        // let m: Matrix<Integer> = Matrix::from_rows(vec![
        //     vec![Integer::from(1), Integer::from(3), Integer::from(2)],
        //     vec![Integer::from(-3), Integer::from(-1), Integer::from(-3)],
        //     vec![Integer::from(2), Integer::from(3), Integer::from(1)],
        // ]);
        // assert_eq!(m.det_naive().unwrap(), Integer::from(-15));
    }
}
