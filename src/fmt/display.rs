// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

//* Use from external library *//
use std::fmt;

//* Use from local library *//
use symbols::*;
use matrix::*;

////////////////////
// --- Symbol --- //
////////////////////

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}

//////////////////////
// --- Fraction --- //
//////////////////////

/*impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}*/

////////////////////////
// --- Polynomial --- //
////////////////////////

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}

////////////////////
// --- Matrix --- //
////////////////////

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}