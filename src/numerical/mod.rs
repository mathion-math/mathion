// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

pub mod ode;
pub mod integral;
pub mod linear_equations;

pub use self::ode::{solve_ode4, solve_ode6};
pub use self::integral::*;
pub use self::linear_equations::*;