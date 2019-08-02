// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//
use num_traits::pow;

//* Use from local library *//
use super::{Monomial, Polynomial};
use functions::*;
use utils::string_to_static_str;

#[derive(PartialEq, Clone, Copy)]
pub enum SymbolType {
    Variable,
    Real,
    Imaginary,
    Special(Special)
}

#[derive(PartialEq, Clone, Copy)]
pub enum Special {
    Sin(Polynomial),
    Cos(Polynomial),
    Tan(Polynomial),
    Exp(Polynomial),
    Log(Polynomial, f64)
}

pub fn sin(input: Polynomial, exp: f64) -> Symbol {
    let variables = input.variables_output();
    let name = string_to_static_str(format!("sin({})", variables));
    Symbol::new(name, SymbolType::Special(Special::Sin(input)), exp)
}

pub fn cos(input: Polynomial, exp: f64) -> Symbol {
    let variables = input.variables_output();
    let name = string_to_static_str(format!("cos({})", variables));
    Symbol::new(name, SymbolType::Special(Special::Cos(input)), exp)
}

pub fn tan(input: Polynomial, exp: f64) -> Symbol {
    let variables = input.variables_output();
    let name = string_to_static_str(format!("tan({})", variables));
    Symbol::new(name, SymbolType::Special(Special::Tan(input)), exp)
}

pub fn exp(input: Polynomial, exp: f64) -> Symbol {
    let variables = input.variables_output();
    let name = string_to_static_str(format!("exp({})", variables));
    Symbol::new(name, SymbolType::Special(Special::Exp(input)), exp)
}

pub fn log(input: Polynomial, base: f64, exp: f64) -> Symbol {
    let variables = input.variables_output();
    let name = string_to_static_str(format!("log_{}({})", base, variables));
    Symbol::new(name, SymbolType::Special(Special::Log(input, base)), exp)
}

impl Special {
    pub fn value(&self, input: f64) -> f64 {
        let polynomial = (&self).polynomial();
        if polynomial.is_real() {
            (&self).inner_value(Variables::new(vec![]))
        } else {
            let first_variable = polynomial.variables()[0];
            (&self).inner_value(Variables::new(vec![(first_variable.name(), input)]))
        }
    }

    pub fn polynomial(&self) -> Polynomial {
        match (&self) {
            Special::Sin(f_x) |
            Special::Cos(f_x) |
            Special::Tan(f_x) |
            Special::Exp(f_x) => {
                *f_x
            },
            Special::Log(f_x, _base) => {
                *f_x
            },
            _ => Symbol::from_f64(0.0).to_polynomial()
        }
    }

    pub fn names(&self) -> Vec<&'static str> {
        (&self).polynomial().variable_names()
    }

    pub fn inner_value(&self, inputs: Variables) -> f64 {
        let input: f64 = match (&self) {
            Special::Sin(f_x) |
            Special::Cos(f_x) |
            Special::Tan(f_x) |
            Special::Exp(f_x) => {
                if f_x.is_real() { 
                    f_x.to_f64()
                } else {
                    f_x.eval_multiple(inputs, false).to_f64()
                }
            },
            Special::Log(f_x, _base) => {
                if f_x.is_real() { 
                    f_x.to_f64()
                } else {
                    f_x.eval_multiple(inputs, false).to_f64()
                }
            },
            _ => 0.0
        };

        match (&self) {
            Special::Sin(_) => {
                input.sin()
            },
            Special::Cos(_) => {
                input.cos()
            },
            Special::Tan(_) => {
                input.tan()
            },
            Special::Exp(_) => {
                input.exp()
            },
            Special::Log(_, base) => {
                input.log2() / base.log2()
            }
            _ => { 0.0 }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Symbol {
    pub name: &'static str,
    pub symbol_type: SymbolType,
    pub exp: f64
}

impl Symbol {
    ////////////////////////
    // --- Initialize --- //
    ////////////////////////

    pub fn new(name: &'static str, symbol_type: SymbolType, exp: f64) -> Self {
        Self {
            name: name,
            symbol_type: symbol_type,
            exp: exp,
        }
    }

    pub fn from_f64(input: f64) -> Self {
        Self {
            name: string_to_static_str(format!("{}", input)),
            symbol_type: SymbolType::Real,
            exp: 1.0
        }
    }

    pub fn name(&self) -> &'static str {
        (&self).name
    }

    pub fn symbol_type(&self) -> SymbolType {
        (&self).symbol_type
    }

    pub fn exp(&self) -> f64 {
        (&self).exp
    }

    pub fn powf(&self, exp: f64) -> Self {
        Symbol {
            name: (&self).name(),
            symbol_type: (&self).symbol_type(),
            exp: (&self).exp() * exp
        }
    }

    pub fn variable_to_real(self, real: f64) -> Result<Symbol, ()> {
        let mut symbol = (&self).clone();
        if !symbol.is_variable() && !symbol.is_special() {
            return Err(())
        }
        let result = match (&self).symbol_type() {
            SymbolType::Special(special) => {
                special.value(real).powf(symbol.exp)
            },
            _ => { real.powf(symbol.exp) }
        };
        symbol.name = string_to_static_str(format!("{}", result));
        symbol.symbol_type = SymbolType::Real;
        symbol.exp = 1.0;
        Ok(symbol)
    }

    pub fn special(&self) -> Result<Special, ()> {
        match self.symbol_type() {
            SymbolType::Special(special) => {
                Ok(special)
            },
            _ => { Err(()) }
        }
    }

    ////////////////////
    // --- Output --- //
    ////////////////////

    pub fn output_string(&self) -> String {
        format!("{}^{}", self.name(), self.exp())
    }

    ////////////////////
    // --- Equals --- //
    ////////////////////

    pub fn is_variable(&self) -> bool {
        (&self).symbol_type() == SymbolType::Variable
    }

    pub fn is_real(&self) -> bool {
        (&self).symbol_type() == SymbolType::Real
    }

    pub fn is_imaginary(&self) -> bool {
        (&self).symbol_type() == SymbolType::Imaginary
    }

    pub fn is_special(&self) -> bool {
        match (&self).symbol_type() {
            SymbolType::Special(_) => {
                true
            },
            _ => { false }
        }
    }

    /////////////////////
    // --- Convert --- //
    /////////////////////
    
    pub fn to_f64(&self) -> f64 {
        if (&self).exp() > 0.0 {
            pow((&self).name().parse::<f64>().unwrap_or(0.0), (&self).exp() as usize)
        } else if (&self).exp() == 0.0 {
            1.0
        } else {
            pow(1.0 / (&self).name().parse::<f64>().unwrap_or(0.0), (&self).exp().abs() as usize)
        }
    }
    
    pub fn to_monomial(&self) -> Monomial {
        Monomial::new(vec![*(&self).clone()])
    }

    pub fn to_polynomial(&self) -> Polynomial {
        Polynomial::new(vec![(&self).to_monomial()])
    }
}
