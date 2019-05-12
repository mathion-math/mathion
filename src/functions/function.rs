// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

//* Use from external library *//
use num_traits::pow;

//* Use from local library *//
use symbols::{Symbol, Monomial, Polynomial};
use utils::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Variables(&'static [(&'static str, f64)]);

impl Variables {
    pub fn new(variables: Vec<(&'static str, f64)>) -> Self {
        Variables(vec_to_static_slice(variables))
    }

    pub fn get(&self) -> Vec<(&'static str, f64)> {
        (&self).0.clone().to_vec()
    }

    pub fn nth(&self, nth: usize) -> (&'static str, f64) {
        (&self).get()[nth]
    }

    pub fn len(&self) -> usize {
        (&self).get().len()
    }

    pub fn push(&mut self, name: &'static str, value: f64) {
        let mut get = (&self).get();
        get.push((name, value));
        self.0 = vec_to_static_slice(get)
    }
    
    pub fn with_one(&self, name: &'static str, value: f64) -> Self {
        let mut get = (&self).get();
        get.push((name, value));
        Variables::new(get)
    }
}

pub trait Function {
    fn eval(&self, name: &'static str, value: f64) -> Self;
    fn eval_multiple(&self, inputs: Variables, without_input_variables: bool) -> Self;
    fn inner_eval(&self, inputs_zero: Variables, without_input_variables: bool) -> Self;
    fn fast_eval(&self, name: &'static str, value: f64) -> f64;
    fn fast_eval_multiple(&self, inputs: Variables) -> f64;
    fn fast_inner_eval(&self, inputs: Variables) -> f64;
    fn delta(self, variable: &'static str, dx: f64) -> f64;
    fn derivative_centered(self, variable: &'static str, dx: f64, h: f64) -> f64;
    fn derivative_normal(self, variable: &'static str, dx: f64, h: f64) -> f64;
    fn solve(self, variable: &'static str, x0: f64) -> f64;
}

impl Function for Polynomial {
    ////////////////////////
    // --- Initialize --- //
    ////////////////////////

    fn eval(&self, name: &'static str, value: f64) -> Self {
        (&self).inner_eval(Variables::new(vec![(name, value)]), true)
    }

    fn eval_multiple(&self, inputs: Variables, without_input_variables: bool) -> Self {
        (&self).inner_eval(inputs, without_input_variables)
    }
    
    fn inner_eval(&self, inputs_zero: Variables, without_input_variables: bool) -> Self {
        let inputs = inputs_zero.get();
        let mut fx = (&self).monomials();
        for i in 0..fx.len() {
            let mut m = fx[i].symbols();
            for j in 0..m.len() {
                let mut is_matched = false;
                for input in inputs.clone() {
                    let name = if m[j].is_special() {
                        let mut split1: Vec<&'static str> = m[j].name().split(|c| c == '(' || c == ')').collect();
                        let mut split2: Vec<&'static str> = split1[1].split(", ").collect();
                        let mut name = "";
                        for var in split2 {
                            if var == input.0 { name = var; break; }
                        }
                        name
                    } else { m[j].name() };
                    if name == input.0 {
                        m[j] = m[j].variable_to_real(input.1).unwrap();
                        break;
                    }
                }
                if !is_matched && (without_input_variables && m[j].is_variable()) {
                    m[j] = Symbol::from_f64(1.0);
                }
            }
            fx[i] = Monomial::new(m).sync();
        }
        Polynomial::new(fx).equal()
    }

    fn fast_eval(&self, name: &'static str, value: f64) -> f64 {
        (&self).fast_inner_eval(Variables::new(vec![(name, value)]))
    }

    fn fast_eval_multiple(&self, inputs: Variables) -> f64 {
        (&self).fast_inner_eval(inputs)
    }

    fn fast_inner_eval(&self, inputs: Variables) -> f64 {
        let inputs = inputs.get();
        let mut result = 0.0;
        for m in (&self).monomials() {
            let mut tmp_result = m.coefficient();
            for s in m.symbols() {
                if !s.is_real() {
                    for input in inputs.clone() {
                        let name = if s.is_special() {
                            let mut split1: Vec<&'static str> = s.name().split(|c| c == '(' || c == ')').collect();
                            let mut split2: Vec<&'static str> = split1[1].split(", ").collect();
                            let mut name = "";
                            for var in split2 {
                                if var == input.0 { name = var; break; }
                            }
                            name
                        } else { s.name() };
                        if name == input.0 {
                            tmp_result *= s.variable_to_real(input.1).unwrap().to_f64();
                            break;
                        }
                    }
                }
            }
            result += tmp_result;
        }
        result
    }

    fn delta(self, variable: &'static str, dx: f64) -> f64 {
        let n = (self.max_exp(variable) + 2.0).powf(2.0);
        let h = 1.0;
        let mut D: Vec<Vec<f64>> = Vec::with_capacity(n as usize);

        for i in 0..(n as usize) {
            let mut f: Vec<f64> = vec![];
            f.push((self.fast_eval(variable, dx + h / (pow(2.0, i + 1)))
                      - self.fast_eval(variable, dx - h / pow(2.0, i + 1)))
                      / ((2.0 * h) / pow(2.0, i + 1)));
            for j in 0..i {
                let f_j = f[j].clone();
                f.push(f_j + (f_j - D[i - 1][j]) / (pow(4.0, j + 1) - 1.0));
            }
            D.push(f);
        }

        (D[n as usize - 1])[n as usize - 1]
    }

    fn derivative_centered(self, variable: &'static str, dx: f64, h: f64) -> f64 {
        ( -self.fast_eval(variable, dx + (2.0 * h)) + 
        (8.0 * self.fast_eval(variable, dx + h)) - 
        (8.0 * self.fast_eval(variable, dx - h)) + 
        (self.fast_eval(variable, dx - (2.0 * h)))) / (12.0 * h)
    }

    fn derivative_normal(self, variable: &'static str, dx: f64, h: f64) -> f64 {
        (self.fast_eval(variable, dx + h) - self.fast_eval(variable, dx)) / h
    }

    fn solve(self, variable: &'static str, x0: f64) -> f64 {
        let error = 0.000000000001;
        let max = 50;
        let mut x_0 = x0;
        let mut x_n = x0;
        let mut result = self.fast_eval(variable, x_n);
        for _i in 2..(max + 1) {
            x_n = x_0 - (result / self.delta(variable, x_n));
            result = self.fast_eval(variable, x_n);
            if result.abs() < error || (x_n - x_0).abs() < error {
                break;
            }
            x_0 = x_n;
        }
        x_n
    }
}