// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

pub mod symbol;
pub mod monomial;
pub mod polynomial;

//* Use from local library *//
pub use symbol::{Symbol, SymbolType, Special, sin, cos, tan, exp};
pub use monomial::Monomial;
pub use polynomial::Polynomial;
