// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//


//* Use from local library *//
use symbols::*;
use functions::*;

pub fn solve_ode4(f_x: Vec<Polynomial>, t_name: &'static str, initial: Variables, interval: (f64, f64), n: usize) -> Result<Vec<Variables>, ()> { // RK-4
    let dt = (interval.1 - interval.0);
    let h = if dt == 0.0 { 1.0 / n as f64 } else { dt / n as f64 };
    let m = f_x.len();

    let mut t = interval.0;
    let mut vars = initial.clone();
    let mut point: Vec<Variables> = vec![];
    for i in 0..n {
        {
            let (mut f_1, mut f_2, mut f_3, mut f_4) = (Vec::with_capacity(m), Vec::with_capacity(m), Vec::with_capacity(m), Vec::with_capacity(m));
            /// f_1
            let mut vars_1 = vars.with_one(t_name, t);
            for j in 0..m {
                f_1.push(h * f_x[j].fast_eval_multiple(vars_1));
            }
            /// f_2
            let mut vars_2 = vars.get();
            for j in 0..m {
                vars_2[j].1 += f_1[j] / 2.0;
            }
            vars_2.push((t_name, t + (h / 2.0)));
            for j in 0..m {
                f_2.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_2.clone())));
            }

            /// f_3
            let mut vars_3 = vars.get();
            for j in 0..m {
                vars_3[j].1 += f_2[j] / 2.0;
            }
            vars_3.push((t_name, t + (h / 2.0)));
            for j in 0..m {
                f_3.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_3.clone())));
            }

            /// f_4
            let mut vars_4 = vars.get();
            for j in 0..m {
                vars_4[j].1 += f_3[j];
            }
            vars_4.push((t_name, t + h));
            for j in 0..m {
                f_4.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_4.clone())));
            }

            /// vars
            let mut vars_vec = vars.get();
            for j in 0..m {
                vars_vec[j].1 += (f_1[j] + (2.0 * f_2[j]) + (2.0 * f_3[j]) + f_4[j]) / 6.0;
            }

            for i in 0..vars_vec.len() {
                if vars_vec[i].1 < 0.0000005 {
                    vars_vec[i].1 = 0;
                }
            }

            vars = Variables::new(vars_vec.clone());
        }
        t = interval.0 + (i as f64) * h;

        point.push(vars.with_one(t_name, t));
    }
    Ok(point)
}

pub fn solve_ode6(f_x: Vec<Polynomial>, t_name: &'static str, initial: Variables, interval: (f64, f64), n: usize) -> Result<Vec<Variables>, ()> { // RK-4
    let dt = (interval.1 - interval.0);
    let h = if dt == 0.0 { 1.0 / n as f64 } else { dt / n as f64 };
    let m = f_x.len();

    let mut t = interval.0;
    let mut vars = initial.clone();
    let mut point: Vec<Variables> = vec![];
    for i in 0..n {
        {
            let (mut f_1, mut f_2, mut f_3, mut f_4, mut f_5, mut f_6, mut f_7) = (Vec::with_capacity(m), Vec::with_capacity(m), Vec::with_capacity(m), Vec::with_capacity(m), Vec::with_capacity(m), Vec::with_capacity(m), Vec::with_capacity(m));
            
            /// f_1
            let mut vars_1 = vars.with_one(t_name, t);
            for j in 0..m {
                f_1.push(h * f_x[j].fast_eval_multiple(vars_1));
            }

            /// f_2
            let mut vars_2 = vars.get();
            for j in 0..m {
                vars_2[j].1 += f_1[j] / 3.0;
            }
            vars_2.push((t_name, t + (h / 3.0)));
            for j in 0..m {
                f_2.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_2.clone())));
            }

            /// f_3
            let mut vars_3 = vars.get();
            for j in 0..m {
                vars_3[j].1 += ((2.0 * f_2[j]) / 3.0);
            }
            vars_3.push((t_name, t + ((2.0 * h) / 3.0)));
            for j in 0..m {
                f_3.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_3.clone())));
            }

            /// f_4
            let mut vars_4 = vars.get();
            for j in 0..m {
                vars_4[j].1 += (f_1[j] / 12.0) + (f_2[j] / 3.0) + (f_3[j] / -12.0);
            }
            vars_4.push((t_name, t + (h / 3.0)));
            for j in 0..m {
                f_4.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_4.clone())));
            }

            /// f_5
            let mut vars_5 = vars.get();
            for j in 0..m {
                vars_5[j].1 += (f_1[j] / -16.0) + ((9.0 * f_2[j]) / 8.0) + ((3.0 * f_3[j]) / -16.0) + ((3.0 * f_4[j]) / -8.0);
            }
            vars_5.push((t_name, t + (h / 2.0)));
            for j in 0..m {
                f_5.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_5.clone())));
            }

            /// f_6
            let mut vars_6 = vars.get();
            for j in 0..m {
                vars_6[j].1 += ((9.0 * f_2[j]) / 8.0) + ((3.0 * f_3[j]) / -8.0) + ((3.0 * f_4[j]) / -4.0) + (f_5[j] / 2.0);
            }
            vars_6.push((t_name, t + (h / 2.0)));
            for j in 0..m {
                f_6.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_6.clone())));
            }

            /// f_7
            let mut vars_7 = vars.get();
            for j in 0..m {
                vars_7[j].1 += ((9.0 * f_1[j]) / 44.0) + ((9.0 * f_2[j]) / -11.0) + ((63.0 * f_3[j]) / 44.0) + ((18.0 * f_4[j]) / 11.0) + ((16.0 * f_6[j]) / -11.0);
            }
            vars_7.push((t_name, t + h));
            for j in 0..m {
                f_7.push(h * f_x[j].fast_eval_multiple(Variables::new(vars_7.clone())));
            }

            /// vars
            let mut vars_vec = vars.get();
            for j in 0..m {
                vars_vec[j].1 += ((11.0 * f_1[j]) / 120.0) + ((27.0 * f_3[j]) / 40.0) + ((27.0 * f_4[j]) / 40.0) + ((4.0 * f_5[j]) / -15.0) + ((4.0 * f_6[j]) / -15.0) + ((11.0 * f_7[j]) / 120.0);
            }

            vars = Variables::new(vars_vec.clone());
        }
        t = interval.0 + (i as f64) * h;
        point.push(vars.with_one(t_name, t));
    }
    Ok(point)
}