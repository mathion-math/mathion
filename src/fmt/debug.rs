// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//
use std::fmt;

//* Use from local library *//
use symbols::*;
use matrix::*;

////////////////////
// --- Symbol --- //
////////////////////

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}

//////////////////////
// --- Monomial --- //
//////////////////////

impl fmt::Debug for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}

//////////////////////
// --- Fraction --- //
//////////////////////

/*impl fmt::Debug for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}*/

////////////////////////
// --- Polynomial --- //
////////////////////////

impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}

////////////////////
// --- Matrix --- //
////////////////////

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (&self).output_string())
    }
}