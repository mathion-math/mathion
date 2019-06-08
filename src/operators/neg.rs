// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
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
