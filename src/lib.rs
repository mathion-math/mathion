// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

extern crate num_traits;
extern crate time;

pub mod functions;
pub mod symbols;
pub mod operators;
pub mod fmt;
pub mod prelude;
pub mod numerical;
pub mod vector;
pub mod matrix;
pub mod sort;

#[macro_use]
pub mod macros;
mod utils;

#[cfg(test)]
mod tests;

//* Use from local library *//
pub use symbols::*;