// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

extern crate num_traits;

pub mod functions;
pub mod symbols;
pub mod operators;
pub mod fmt;
pub mod prelude;
pub mod numerical;
pub mod vector;
pub mod matrix;

#[macro_use]
pub mod macros;
mod utils;

#[cfg(test)]
mod tests;

//* Use from local library *//
pub use symbols::*;