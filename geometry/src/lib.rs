#![allow(dead_code)]

use std::borrow::Borrow;
use std::hash::Hash;
use std::rc::Rc;

use algebraeon_rings::structure::structure::{FieldStructure, OrderedRingStructure};

mod coordinates;
pub use coordinates::*;

mod ambient_space;
pub use ambient_space::*;

mod affine_subspace;
pub use affine_subspace::*;

pub mod simplexes;
