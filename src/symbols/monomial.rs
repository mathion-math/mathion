// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//

//* Use from local library *//
use super::{Symbol, Polynomial};
use sort::HeapSort;
use utils::*;

#[derive(PartialEq, Clone, Copy)]
pub struct Monomial {
    pub symbols: &'static [Symbol]
}

impl Monomial {
    ////////////////////////
    // --- Initialize --- //
    ////////////////////////

    pub fn new(symbols: Vec<Symbol>) -> Self {
        Monomial {
            symbols: vec_to_static_slice::<Symbol>(symbols)
        }
    }

    pub fn from_f64(num: f64) -> Self {
        Monomial::new(vec![Symbol::from_f64(num)])
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        (&self).symbols.to_vec()
    }

    pub fn filter_not_real(&self) -> Self {
        Monomial::new((&self).symbols()
            .into_iter()
            .filter(|symbol| !symbol.is_real() )
            .collect())
    }

    pub fn filter_variables(&self) -> Self {
        Monomial::new((&self).symbols()
            .into_iter()
            .filter(|symbol| symbol.is_variable() )
            .collect())
    }

    pub fn coefficient(&self) -> f64 {
        let mut coefficient = 1.0;

        for symbol in (&self).symbols() {
            if symbol.is_real() {
                coefficient *= symbol.to_f64();
            }
        }

        coefficient
    }

    pub fn exps(&self) -> f64 {
        let mut exps = 1.0;

        for symbol in (&self).filter_not_real().symbols() {
            exps += symbol.exp();
        }

        exps
    }

    pub fn number_of_variables(&self) -> usize {
        let symbols = (&self).filter_variables();
        symbols.len()
    }

    pub fn nth(&self, index: usize) -> Symbol {
        (&self).symbols()[index]
    }

    pub fn len(&self) -> usize {
        (&self).symbols().len()
    }
    
    pub fn minus(&self) -> Self {
        let mut symbols = (&self).symbols();
        symbols.push(Symbol::from_f64(-1.0));
        Monomial::new(symbols)
    }

    pub fn powf(&self, exp: f64) -> Self {
        let mut symbols = (&self).symbols();
        for i in 0..symbols.len() {
            symbols[i] = symbols[i].powf(exp);
        }
        Monomial::new(symbols)
    }

    pub fn find_zero(&self) -> bool {
        for x in self.symbols() {
            if x.is_real() && x.name() == "0" {
                return true;
            }
        }
        false
    }

    ////////////////////
    // --- Output --- //
    ////////////////////

    pub fn output_string(&self) -> String {
        let mut output = String::new();
        for i in 0..self.len() {
            output.push_str(string_to_static_str((&self).nth(i).output_string()));
            if i != self.len() - 1 { output.push_str("*"); }
        }
        format!("{}", output)
    }

    pub fn output_not_real(&self) -> String {
        let mut output = String::new();
        let not_real = (&self).filter_not_real();
        for i in 0..not_real.len() {
            output.push_str(string_to_static_str(not_real.nth(i).output_string()));
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
        self.nth(0).to_f64() == 0.0
    }

    ///////////////////////
    // --- Functions --- //
    ///////////////////////
    
    pub fn sync(&self) -> Self {
        let mut after_symbols: Vec<Symbol> = vec![];

        for x in self.symbols() { // O(n^2)
            let mut is_found = false;
            for i in 0..after_symbols.len() {
                if x.is_real() && x.name() == "0" {
                    after_symbols = vec![Symbol::from_f64(0.0)];
                    return Monomial::new(after_symbols);
                }
                if x.name() == after_symbols[i].clone().name() {
                    after_symbols[i] = Symbol::new(x.name(), x.symbol_type(), x.exp() + after_symbols[i].clone().exp());
                    is_found = true;
                } else if x.is_real() && after_symbols[i].clone().is_real() {
                    let mul = (x.to_f64() * after_symbols[i].clone().to_f64()).to_string();
                    after_symbols[i] = Symbol::new(string_to_static_str(mul), x.symbol_type(), 1.0);
                    is_found = true;
                }
            }
            if !is_found {
                after_symbols.push(x);
            }
        }

        let mut heap_sort = HeapSort::new(after_symbols, |a: &Symbol, b: &Symbol| (a.name().as_bytes().clone()[0] > b.name().as_bytes().clone()[0]).cmp(&false));
        
        Monomial::new(heap_sort.sort())
    }

    /////////////////////
    // --- Convert --- //
    /////////////////////

    pub fn to_f64(self) -> f64 {
        if self.is_real() { self.nth(0).to_f64() } else { 0.0 }
    }

    pub fn to_polynomial(self) -> Polynomial {
        Polynomial::new(vec![self])
    }
}