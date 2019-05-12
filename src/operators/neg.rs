// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

//* Use from external library *//
use std::ops::Neg;

//* Use from local library *//
use symbols::{Symbol, SymbolType, Monomial, Polynomial};
use utils::string_to_static_str;

////////////////////
// --- Symbol --- //
////////////////////

impl Neg for Symbol {
    type Output = Monomial;

    fn neg(self) -> Monomial {
        self.to_monomial().minus()
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl Neg for Monomial {
    type Output = Monomial;

    fn neg(self) -> Monomial {
        self.minus()
    }
}

////////////////////////
// --- Polynomial --- //
////////////////////////

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Polynomial {
        -1.0 * self
    }
}
