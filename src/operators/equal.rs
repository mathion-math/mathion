// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//
use std::cmp::PartialEq;

//* Use from local library *//
use symbols::*;
use functions::*;
use utils::string_to_static_str;

////////////////////
// --- Symbol --- //
////////////////////

impl PartialEq<Symbol> for &'static str {
    fn eq(&self, other: &Symbol) -> bool {
        self == &(string_to_static_str(other.output_string()))
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl PartialEq<Monomial> for &'static str {
    fn eq(&self, other: &Monomial) -> bool {
        self == &(string_to_static_str(other.output_string()))
    }
}

//////////////////////
// --- Fraction --- //
//////////////////////

/*impl PartialEq<Fraction> for &'static str {
    fn eq(&self, other: &Fraction) -> bool {
        self == &(string_to_static_str(other.output_string()))
    }
}*/

////////////////////////
// --- Polynomial --- //
////////////////////////

impl PartialEq<Polynomial> for &'static str {
    fn eq(&self, other: &Polynomial) -> bool {
        self == &(string_to_static_str(other.output_string()))
    }
}
