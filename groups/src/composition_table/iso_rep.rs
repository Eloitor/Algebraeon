use super::group::*;
use super::homomorphism::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone)]
pub enum IsomorphismClass {
    Trivial,
    Cyclic(usize),
    Dihedral(usize),
    Quaternion,
    Alternating(usize),
    Symmetric(usize),
    DirectProduct(Box<BTreeMap<IsomorphismClass, usize>>), //count how many of each isomorphic factor
    Unknown(usize),
}

pub fn isomorphism_class(group: &FiniteGroup) -> IsomorphismClass {
    IsomorphismClass::from_group(group)
}

impl IsomorphismClass {
    fn check_state(&self) -> Result<(), &'static str> {
        match self {
            Self::Trivial => {}
            Self::Cyclic(n) => {
                if *n == 0 {
                    return Err("C0 is not a group");
                }
            }
            Self::Dihedral(n) => {
                if *n == 0 {
                    return Err("D0 is not a group");
                }
            }
            Self::Quaternion => {}
            Self::Alternating(_n) => {}
            Self::Symmetric(_n) => {}
            Self::DirectProduct(_factors) => {
                todo!();
            }
            Self::Unknown(n) => {
                if *n == 0 {
                    return Err("Unknown group with 0 elements is not valid");
                }
            }
        }

        Ok(())
    }

    pub fn from_group(group: &FiniteGroup) -> Self {
        let n = group.size();

        //trivial
        if n == 1 {
            return Self::Trivial;
        }

        //cyclic
        match find_isomorphism(group, &examples::cyclic_group_structure(n)) {
            Some(_f) => {
                return Self::Cyclic(n);
            }
            None => {}
        }

        //direct products
        let mut nsgs = group.normal_subgroups();
        nsgs.sort_by_key(|(nsg, _gens)| nsg.size());
        for (nsg, _gens) in &nsgs {
            nsg.check_state().unwrap();
            if nsg.size() != 1 && nsg.size() != group.size() {
                //non-trivial normal subgroup
                let nsg_group = nsg.subgroup().to_group();
                let quo_group = nsg.quotient_group();
                let prod_group = direct_product_structure(&nsg_group, &quo_group);
                nsg_group.check_state().unwrap();
                quo_group.check_state().unwrap();
                let isom_result = find_isomorphism(&prod_group, &group);
                match isom_result {
                    Some(_f) => {
                        return IsomorphismClass::from_group(&nsg_group)
                            * IsomorphismClass::from_group(&quo_group);
                    }
                    None => {}
                }
            }
        }

        debug_assert_eq!(false, group.is_abelian());

        //quaternion
        match find_isomorphism(group, &examples::quaternion_group_structure()) {
            Some(_f) => {
                return Self::Quaternion;
            }
            None => {}
        }

        //symmetric
        let mut k = 0;
        let mut k_fact = 1;
        while k_fact < n {
            k += 1;
            k_fact *= k;
        }
        if n == k_fact {
            match find_isomorphism(group, &examples::symmetric_group_structure(k)) {
                Some(_f) => {
                    return Self::Symmetric(k);
                }
                None => {}
            }
        }

        //alternating
        let mut k = 2;
        let mut half_k_fact = 1;
        while half_k_fact < n {
            k += 1;
            half_k_fact *= k;
        }
        if n == half_k_fact {
            match find_isomorphism(group, &examples::alternating_group_structure(k)) {
                Some(_f) => {
                    return Self::Alternating(k);
                }
                None => {}
            }
        }

        //dihedral
        if n % 2 == 0 {
            match find_isomorphism(group, &examples::dihedral_group_structure(n / 2)) {
                Some(_f) => {
                    return Self::Dihedral(n / 2);
                }
                None => {}
            }
        }

        IsomorphismClass::Unknown(n)
    }

    pub fn to_group(&self) -> Result<FiniteGroup, ()> {
        match self {
            Self::Trivial => Ok(examples::cyclic_group_structure(1)),
            Self::Cyclic(n) => Ok(examples::cyclic_group_structure(*n)),
            Self::Dihedral(n) => Ok(examples::dihedral_group_structure(*n)),
            Self::Quaternion => Ok(examples::quaternion_group_structure()),
            Self::Alternating(n) => Ok(examples::alternating_group_structure(*n)),
            Self::Symmetric(n) => Ok(examples::symmetric_group_structure(*n)),
            Self::DirectProduct(factors) => {
                let mut factors_list = vec![];
                for (factor, power) in factors.iter() {
                    for _i in 0..*power {
                        factors_list.push(factor);
                    }
                }
                let mut prod_group = examples::trivial_group_structure();
                for factor in factors_list {
                    match factor.to_group() {
                        Ok(factor_group) => {
                            prod_group = direct_product_structure(&factor_group, &prod_group);
                        }
                        Err(()) => {
                            return Err(());
                        }
                    }
                }
                Ok(prod_group)
            }
            Self::Unknown(_n) => Err(()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Trivial => "T".to_owned(),
            Self::Cyclic(n) => "C".to_owned() + &n.to_string(),
            Self::Dihedral(n) => "D".to_owned() + &n.to_string(),
            Self::Quaternion => "Q8".to_owned(),
            Self::Alternating(n) => "A".to_owned() + &n.to_string(),
            Self::Symmetric(n) => "S".to_owned() + &n.to_string(),
            Self::DirectProduct(factors) => {
                let mut ans = "".to_owned();
                let mut first = true;
                for (factor, power) in factors.iter() {
                    for _i in 0..*power {
                        if !first {
                            ans += "x";
                        }
                        ans += &factor.to_string();
                        first = false;
                    }
                }
                ans
            }
            Self::Unknown(n) => "Unknown".to_owned() + &n.to_string(),
        }
    }
}

impl std::ops::Mul<IsomorphismClass> for IsomorphismClass {
    type Output = IsomorphismClass;

    fn mul(self, other: IsomorphismClass) -> Self::Output {
        match self {
            IsomorphismClass::Trivial => {
                return self;
            }
            _ => {}
        }
        match other {
            IsomorphismClass::Trivial => {
                return other;
            }
            _ => {}
        }

        let mut factors: BTreeMap<IsomorphismClass, usize> = BTreeMap::new();

        match self {
            IsomorphismClass::DirectProduct(fs) => {
                for (f, p) in fs.iter() {
                    *factors.entry(f.clone()).or_insert(0) += p;
                }
            }
            _ => {
                *factors.entry(self).or_insert(0) += 1;
            }
        }

        match other {
            IsomorphismClass::DirectProduct(fs) => {
                for (f, p) in fs.iter() {
                    *factors.entry(f.clone()).or_insert(0) += p;
                }
            }
            _ => {
                *factors.entry(other).or_insert(0) += 1;
            }
        }

        IsomorphismClass::DirectProduct(Box::new(factors))
    }
}

#[cfg(test)]
mod isom_class_tests {
    use super::*;

    #[test]
    fn test() {
        let g = examples::klein_four_structure();
        let i = IsomorphismClass::from_group(&g);
        println!("{:?}", i.to_string());
        assert_eq!(
            i,
            IsomorphismClass::DirectProduct(Box::new(BTreeMap::from([(
                IsomorphismClass::Cyclic(2),
                2
            )])))
        )
    }
}
