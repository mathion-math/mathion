// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

//* Use from external library *//

//* Use from local library *//
use super::{Symbol, Monomial, Special};
use utils::*;

#[derive(PartialEq, Clone, Copy)]
pub struct Polynomial {
    pub monomials: &'static [Monomial],
    pub denominator: &'static [Monomial]
}

impl Polynomial {
    ////////////////////////
    // --- Initialize --- //
    ////////////////////////

    pub fn new(monomials: Vec<Monomial>) -> Self {
        Polynomial {
            monomials: vec_to_static_slice::<Monomial>(monomials),
            denominator: vec_to_static_slice::<Monomial>(vec![Monomial::from_f64(1.0)])
        }
    }

    pub fn fraction(monomials: Vec<Monomial>, denominator: Vec<Monomial>) -> Self {
        Polynomial {
            monomials: vec_to_static_slice::<Monomial>(monomials),
            denominator: vec_to_static_slice::<Monomial>(denominator)
        }
    }

    pub fn blank() -> Self {
        Polynomial::new(vec![Monomial::new(vec![Symbol::from_f64(0.0)])])
    }

    pub fn from_f64(num: f64) -> Self {
        Polynomial::new(vec![Monomial::new(vec![Symbol::from_f64(num)])])
    }

    pub fn monomials(&self) -> Vec<Monomial> {
        (&self).monomials.to_vec()
    }

    pub fn denominator(&self) -> Self {
        Self::new(self.denominator.to_vec())
    }

    pub fn nth(&self, index: usize) -> Monomial {
        (&self).monomials()[index].clone()
    }

    pub fn len(&self) -> usize {
        self.monomials().len()
    }

    pub fn filter_not_real(&self) -> Self {
        let mut monomials: Vec<Monomial> = vec![];
        for m in self.monomials() {
            monomials.push(m.filter_not_real());
        }
        Self::new(monomials)
    }

    pub fn variables(&self) -> Vec<Symbol> {
        let mut variables: Vec<Symbol> = vec![];
        for m in (&self).filter_not_real().monomials() {
            for s in m.filter_not_real().symbols() {
                if variables.iter().position(|&v| v == s || s.special().unwrap_or(Special::Sin(Polynomial::new(vec![]))).polynomial().variables().iter().position(|&v2| v2 == v).is_some()).is_none() {
                    variables.push(s);
                }
            }
        }
        variables
    }

    pub fn variable_names(&self) -> Vec<&'static str> {
        let mut variables: Vec<&'static str> = vec![];
        for m in (&self).filter_not_real().monomials() {
            for s in m.filter_not_real().symbols() {
                if variables.iter().position(|&v| v == s).is_none() {
                    variables.push(s.name());
                }
            }
        }
        variables
    }

    pub fn variables_output(&self) -> &'static str {
        let mut variables = String::new();
        let names = (&self).variable_names();
        for i in 0..(&self).variable_names().len() {
            variables.push_str(names[i]);
            if i != (&self).variable_names().len() - 1 {
                variables.push_str(", ");
            }
        }
        string_to_static_str(variables)
    }

    pub fn max_exp(&self, variable: &'static str) -> f64 {
        let mut exp = 0.0;
        for m in (&self).monomials() {
            for s in m.symbols() {
                let is_match: bool = (s.name() == variable) && s.is_variable();
                if is_match && ((s.exp() > exp) || (s.exp() < 0.0 && s.exp() < exp)) {
                    exp = s.exp();
                }
            }
        }
        exp
    }

    pub fn is_denominator_blank(&self) -> bool {
        self.denominator().len() == 1 && self.denominator().nth(0).to_f64() == 1.0
    }

    ////////////////////
    // --- Output --- //
    ////////////////////

    pub fn output_string(&self) -> String {
        let mut output = String::new();
        for i in 0..self.len() {
            output.push_str(&self.monomials()[i].output_string());
            if !self.is_denominator_blank() {
                output.push_str(" / ");
                output.push_str(&self.denominator().output_string());
            }
            if i != self.len() - 1 { output.push_str(" + "); }
        }
        format!("{}", output)
    }

    ////////////////////
    // --- Equals --- //
    ////////////////////
    
    pub fn is_real(&self) -> bool {
        (&self).len() == 1 && (&self).nth(0).is_real()
    }

    pub fn is_zero(&self) -> bool {
        (&self).len() == 1 && (&self).nth(0).is_zero()
    }

    ///////////////////////
    // --- Functions --- //
    ///////////////////////

    pub fn sync(&self) -> Self {
        let mut after_monomials: Vec<Monomial> = vec![];
        let mut is_fraction = true;

        let denominator: Monomial = if self.denominator().is_real() { self.denominator().nth(0) } else { Monomial::from_f64(1.0) };

        for x in self.monomials() {
            if !x.find_zero() {
                let mut is_found = false;
                for i in 0..after_monomials.len() {
                    if x.output_not_real() == after_monomials[i].output_not_real() {
                        after_monomials[i] = (x.coefficient() + after_monomials[i].coefficient().clone()) * after_monomials[i].filter_not_real().clone();
                        is_found = true;
                    } else if x.is_real() && after_monomials[i].clone().is_real() {
                        let add = Symbol::from_f64(x.nth(0).to_f64() + after_monomials[i].clone().nth(0).to_f64());
                        after_monomials[i] = add.to_monomial();
                    }
                }
                if !is_found {
                    after_monomials.push(x);
                }
            }
        }

        if denominator.to_f64() != 1.0 && denominator.is_real() {
            is_fraction = false; 
            for i in 0..after_monomials.len() {
                if !after_monomials[i].is_zero() && !denominator.is_zero() {
                    after_monomials[i] = after_monomials[i] / denominator;
                }
            }
        }

        after_monomials.sort_unstable_by(|a, b| (a.exps() < b.exps() && a.number_of_variables() <= b.number_of_variables()).cmp(&false));

        /*for _i in 0..after_monomials.len() {
            for j in 0..after_monomials.len() - 1 {
                if after_monomials[j].exps() < after_monomials[j + 1].exps() && after_monomials[j].number_of_variables() <= after_monomials[j + 1].number_of_variables() {
                    after_monomials.swap(j, j + 1);
                }
            }
        }*/

        if is_fraction {
            Polynomial::fraction(after_monomials, self.denominator().monomials())
        } else {
            Polynomial::new(after_monomials)
        }
    }

    pub fn equal(&self) -> Self {
        let mut after_monomials: Vec<Monomial> = vec![];

        for x in self.monomials() {
            if !x.find_zero() {
                let mut is_found = false;
                for i in 0..after_monomials.len() {
                    if x.output_not_real() == after_monomials[i].output_not_real() {
                        after_monomials[i] = (x.coefficient() + after_monomials[i].coefficient().clone()) * after_monomials[i].filter_not_real().clone();
                        is_found = true;
                    } else if x.is_real() && after_monomials[i].clone().is_real() {
                        let add = Symbol::from_f64(x.nth(0).to_f64() + after_monomials[i].clone().nth(0).to_f64());
                        after_monomials[i] = add.to_monomial();
                    }
                }
                if !is_found {
                    after_monomials.push(x);
                }
            }
        }

        Polynomial::new(after_monomials)
    }

    /////////////////////
    // --- Convert --- //
    /////////////////////

    pub fn to_f64(self) -> f64 {
        if self.is_real() { self.nth(0).to_f64() } else { 0.0 }
    }

    pub fn to_polynomial(self) -> Self {
        self
    }
}