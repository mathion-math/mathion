// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

pub mod ode;
pub mod integral;
pub mod linear_equations;

pub use self::ode::{solve_ode4, solve_ode6};
pub use self::integral::*;
pub use self::linear_equations::*;