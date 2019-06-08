// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//
use std::ops::Div;

//* Use from local library *//
use symbols::{Symbol, Monomial, Polynomial};
use utils::*;

////////////////////
// --- Symbol --- //
////////////////////

impl Div for Symbol {
    type Output = Monomial;

    fn div(self, other: Symbol) -> Monomial {
        Monomial::new(vec![self, other.powf(-1.0)]).sync()
    }
}

impl Div<Monomial> for Symbol {
    type Output = Monomial;

    fn div(self, other: Monomial) -> Monomial {
        let mut symbols = vec![self];
        symbols.extend((other.powf(-1.0)).symbols());
        Monomial::new(symbols).sync()
    }
}

impl Div<f64> for Symbol {
    type Output = Monomial;

    fn div(self, other: f64) -> Monomial {
        Monomial::new(vec![self, Symbol::from_f64(1.0 / other)]).sync()
    }
}

impl Div<Symbol> for f64 {
    type Output = Monomial;

    fn div(self, other: Symbol) -> Monomial {
        Monomial::new(vec![other.powf(-1.0), Symbol::from_f64(self)]).sync()
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl Div for Monomial {
    type Output = Monomial;

    fn div(self, other: Monomial) -> Monomial {
        let mut symbols = self.symbols();
        symbols.extend((other.powf(-1.0)).symbols());
        Monomial::new(symbols).sync()
    }
}

impl Div<Symbol> for Monomial {
    type Output = Monomial;

    fn div(self, other: Symbol) -> Monomial {
        let mut symbols = self.symbols();
        symbols.push(other.powf(-1.0));
        Monomial::new(symbols).sync()
    }
}

impl Div<f64> for Monomial {
    type Output = Monomial;

    fn div(self, other: f64) -> Monomial {
        let mut symbols = self.symbols();
        symbols.push(Symbol::from_f64(1.0 / other));
        Monomial::new(symbols).sync()
    }
}

impl Div<Monomial> for f64 {
    type Output = Monomial;

    fn div(self, other: Monomial) -> Monomial {
        let mut symbols = (other.powf(-1.0)).symbols();
        symbols.push(Symbol::from_f64(self));
        Monomial::new(symbols).sync()
    }
}

////////////////////////
// --- Polynomial --- //
////////////////////////

impl Div for Polynomial {
    type Output = Polynomial;

    fn div(self, other: Polynomial) -> Polynomial {
        Polynomial::fraction(self.monomials(), (self.denominator() * other).monomials()).sync()
    }
}

impl Div<Symbol> for Polynomial {
    type Output = Polynomial;

    fn div(self, other: Symbol) -> Polynomial {
        Polynomial::fraction(self.monomials(), (self.denominator() * other).monomials()).sync()
    }
}

impl Div<f64> for Polynomial {
    type Output = Polynomial;

    fn div(self, other: f64) -> Polynomial {
        Polynomial::fraction(self.monomials(), (self.denominator() * other).monomials()).sync()
    }
}

impl Div<Monomial> for Polynomial {
    type Output = Polynomial;

    fn div(self, other: Monomial) -> Polynomial {
        Polynomial::fraction(self.monomials(), (self.denominator() * other).monomials()).sync()
    }
}
