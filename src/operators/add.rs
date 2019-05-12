// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

//* Use from external library *//
use std::ops::Add;

//* Use from local library *//
use symbols::{Symbol, SymbolType, Monomial, Polynomial};
use utils::string_to_static_str;

////////////////////
// --- Symbol --- //
////////////////////

impl Add for Symbol {
    type Output = Polynomial;

    fn add(self, other: Symbol) -> Polynomial {
        Polynomial::new(vec![self.to_monomial(), other.to_monomial()]).sync()
    }
}

impl Add<Monomial> for Symbol {
    type Output = Polynomial;

    fn add(self, other: Monomial) -> Polynomial {
        Polynomial::new(vec![self.to_monomial(), other]).sync()
    }
}

impl Add<Polynomial> for Symbol {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![self.to_monomial()];
        monomials.extend(other.monomials());
        Polynomial::new(monomials).sync()
    }
}

impl Add<f64> for Symbol {
    type Output = Polynomial;

    fn add(self, other: f64) -> Polynomial {
        Polynomial::new(vec![self.to_monomial(), Symbol::from_f64(other).to_monomial()]).sync()
    }
}

impl Add<Symbol> for f64 {
    type Output = Polynomial;

    fn add(self, other: Symbol) -> Polynomial {
        Polynomial::new(vec![Symbol::from_f64(self).to_monomial(), other.to_monomial()]).sync()
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl Add for Monomial {
    type Output = Polynomial;

    fn add(self, other: Monomial) -> Polynomial {
        Polynomial::new(vec![self, other]).sync()
    }
}

impl Add<Symbol> for Monomial {
    type Output = Polynomial;

    fn add(self, other: Symbol) -> Polynomial {
        Polynomial::new(vec![self, other.to_monomial()]).sync()
    }
}

impl Add<Polynomial> for Monomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        let mut monomials = vec![self];
        monomials.extend(other.monomials());
        Polynomial::new(monomials).sync()
    }
}

impl Add<f64> for Monomial {
    type Output = Polynomial;

    fn add(self, other: f64) -> Polynomial {
        Polynomial::new(vec![self, Symbol::from_f64(other).to_monomial()]).sync()
    }
}

impl Add<Monomial> for f64 {
    type Output = Polynomial;

    fn add(self, other: Monomial) -> Polynomial {
        Polynomial::new(vec![Symbol::from_f64(self).to_monomial(), other]).sync()
    }
}

////////////////////////
// --- Polynomial --- //
////////////////////////

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        if self.denominator() != other.denominator() {
            let mut monomials = (self * other.denominator()).monomials();
            monomials.extend((other * self.denominator()).monomials());
            Polynomial::fraction(monomials, (other.denominator() * self.denominator()).monomials()).sync()
        } else {
            let mut monomials = self.monomials();
            monomials.extend(other.monomials());
            Polynomial::fraction(monomials, self.denominator().monomials()).sync()
        }
    }
}

impl Add<Symbol> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Symbol) -> Polynomial {
        self + other.to_polynomial()
    }
}

impl Add<Monomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Monomial) -> Polynomial {
        self + other.to_polynomial()
    }
}

impl Add<f64> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: f64) -> Polynomial {
        self + Polynomial::from_f64(other)
    }
}

impl Add<Polynomial> for f64 {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        other + Polynomial::from_f64(self)
    }
}


