// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//
use std::ops::Mul;
use num_traits::pow;

//* Use from local library *//
use symbols::{Symbol, SymbolType, Monomial, Polynomial};
use utils::string_to_static_str;

////////////////////
// --- Symbol --- //
////////////////////

impl Mul for Symbol {
    type Output = Monomial;

    fn mul(self, other: Symbol) -> Monomial {
        Monomial::new(vec![self, other]).sync()
    }
}

impl Mul<Monomial> for Symbol {
    type Output = Monomial;

    fn mul(self, other: Monomial) -> Monomial {
        let mut symbols = vec![self];
        symbols.extend(other.symbols());
        Monomial::new(symbols).sync()
    }
}

impl Mul<Polynomial> for Symbol {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![];

        for monomial in other.monomials() {
            monomials.push(self * monomial);
        }

        Polynomial::new(monomials).sync()
    }
}

impl Mul<f64> for Symbol {
    type Output = Monomial;

    fn mul(self, other: f64) -> Monomial {
        Monomial::new(vec![self, Symbol::from_f64(other)]).sync()
    }
}

impl Mul<Symbol> for f64 {
    type Output = Monomial;

    fn mul(self, other: Symbol) -> Monomial {
        Monomial::new(vec![other, Symbol::from_f64(self)]).sync()
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl Mul for Monomial {
    type Output = Monomial;

    fn mul(self, other: Monomial) -> Monomial {
        let mut symbols = self.symbols();
        symbols.extend(other.symbols());
        Monomial::new(symbols).sync()
    }
}

impl Mul<Symbol> for Monomial {
    type Output = Monomial;

    fn mul(self, other: Symbol) -> Monomial {
        let mut symbols = self.symbols();
        symbols.push(other);
        Monomial::new(symbols).sync()
    }
}

impl Mul<Polynomial> for Monomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![];

        for monomial in other.monomials() {
            monomials.push(self.clone() * monomial);
        }

        Polynomial::new(monomials).sync()
    }
}

impl Mul<f64> for Monomial {
    type Output = Monomial;

    fn mul(self, other: f64) -> Monomial {
        let mut symbols = self.symbols();
        symbols.push(Symbol::from_f64(other));
        Monomial::new(symbols).sync()
    }
}

impl Mul<Monomial> for f64 {
    type Output = Monomial;

    fn mul(self, other: Monomial) -> Monomial {
        let mut symbols = other.symbols();
        symbols.push(Symbol::from_f64(self));
        Monomial::new(symbols).sync()
    }
}

////////////////////////
// --- Polynomial --- //
////////////////////////

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![];
        let mut deno_monomials = vec![];
        for x in self.monomials() {
            for y in other.monomials() {
                monomials.push(x.clone() * y);
            }
        }
        for x in self.denominator().monomials() {
            for y in other.denominator().monomials() {
                deno_monomials.push(x.clone() * y);
            }
        }
        Polynomial::fraction(monomials, deno_monomials).sync()
    }
}

impl Mul<Symbol> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Symbol) -> Polynomial {
        let mut monomials = vec![];
        for x in self.monomials() {
            monomials.push(x.clone() * other);
        }
        Polynomial::new(monomials).sync()
    }
}

impl Mul<Monomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Monomial) -> Polynomial {
        let mut monomials = vec![];
        for x in self.monomials() {
            monomials.push(x.clone() * other.clone());
        }
        Polynomial::new(monomials).sync()
    }
}

impl Mul<f64> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: f64) -> Polynomial {
        let mut monomials = vec![];
        let constant = Symbol::from_f64(other);
        for x in self.monomials() {
            monomials.push(x.clone() * constant);
        }
        Polynomial::fraction(monomials, self.denominator().monomials()).sync()
    }
}

impl Mul<Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![];
        let constant = Symbol::from_f64(self);
        for x in other.monomials() {
            monomials.push(x.clone() * constant);
        }
        Polynomial::fraction(monomials, other.denominator().monomials()).sync()
    }
}