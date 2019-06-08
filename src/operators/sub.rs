// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//
use std::ops::Sub;

//* Use from local library *//
use symbols::{Symbol, SymbolType, Monomial, Polynomial};
use utils::string_to_static_str;

////////////////////
// --- Symbol --- //
////////////////////

impl Sub for Symbol {
    type Output = Polynomial;

    fn sub(self, other: Symbol) -> Polynomial {
        Polynomial::new(vec![self.to_monomial(), other.to_monomial().minus()]).sync()
    }
}

impl Sub<Monomial> for Symbol {
    type Output = Polynomial;

    fn sub(self, other: Monomial) -> Polynomial {
        Polynomial::new(vec![self.to_monomial(), other.minus()]).sync()
    }
}

impl Sub<Polynomial> for Symbol {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![self.to_monomial()];
        monomials.extend((-1.0 * other).monomials());
        Polynomial::new(monomials).sync()
    }
}

impl Sub<f64> for Symbol {
    type Output = Polynomial;

    fn sub(self, other: f64) -> Polynomial {
        Polynomial::new(vec![self.to_monomial(), Symbol::from_f64(other).to_monomial().minus()]).sync()
    }
}

impl Sub<Symbol> for f64 {
    type Output = Polynomial;

    fn sub(self, other: Symbol) -> Polynomial {
        Polynomial::new(vec![Symbol::from_f64(self).to_monomial(), other.to_monomial().minus()]).sync()
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl Sub for Monomial {
    type Output = Polynomial;

    fn sub(self, other: Monomial) -> Polynomial {
        Polynomial::new(vec![self, other.minus()]).sync()
    }
}

impl Sub<Symbol> for Monomial {
    type Output = Polynomial;

    fn sub(self, other: Symbol) -> Polynomial {
        Polynomial::new(vec![self, other.to_monomial().minus()]).sync()
    }
}

impl Sub<Polynomial> for Monomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![self];
        monomials.extend((-1.0 * other).monomials());
        Polynomial::new(monomials).sync()
    }
}

impl Sub<f64> for Monomial {
    type Output = Polynomial;

    fn sub(self, other: f64) -> Polynomial {
        Polynomial::new(vec![self, Symbol::from_f64(other).to_monomial().minus()]).sync()
    }
}

impl Sub<Monomial> for f64 {
    type Output = Polynomial;

    fn sub(self, other: Monomial) -> Polynomial {
        Polynomial::new(vec![Symbol::from_f64(self).to_monomial(), other.minus()]).sync()
    }
}

////////////////////////
// --- Polynomial --- //
////////////////////////

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        if self.denominator() != other.denominator() {
            let mut monomials = (self * other.denominator()).monomials();
            monomials.extend((-1.0 * other * self.denominator()).monomials());
            Polynomial::fraction(monomials, (other.denominator() * self.denominator()).monomials()).sync()
        } else {
            let mut monomials = self.monomials();
            monomials.extend((-1.0 * other).monomials());
            Polynomial::fraction(monomials, self.denominator().monomials()).sync()
        }
    }
}

impl Sub<Symbol> for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Symbol) -> Polynomial {
        self - other.to_polynomial()
    }
}

impl Sub<Monomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Monomial) -> Polynomial {
        self - other.to_polynomial()
    }
}

impl Sub<f64> for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: f64) -> Polynomial {
        self - Polynomial::from_f64(other)
    }
}

impl Sub<Polynomial> for f64 {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        other - Polynomial::from_f64(self)
    }
}


