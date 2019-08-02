// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

pub mod symbol;
pub mod monomial;
pub mod polynomial;

//* Use from local library *//
pub use symbol::{Symbol, SymbolType, Special, sin, cos, tan, exp, log};
pub use monomial::Monomial;
pub use polynomial::Polynomial;
