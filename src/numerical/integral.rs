// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

//* Use from local library *//
use symbols::Polynomial;
use functions::Function;

pub fn romberg(f_x: Polynomial, variable: &'static str, i_max: usize, interval: (f64, f64)) -> Result<f64, ()> {
    let delta = interval.1 - interval.0;
    let mut R: Vec<Vec<f64>> = Vec::with_capacity(i_max);
    let mut T: Vec<f64> = Vec::with_capacity(i_max);

    for i in 0..(i_max + 1) {
        let tmp = if i == 0 { (delta * (f_x.fast_eval(variable, interval.0) + f_x.fast_eval(variable, interval.1))) / 2.0 } else {
            let h = delta / 2.0_f64.powf(i as f64);
            let mut sum = 0.0;
            for j in 1..(2.0_f64.powf(i as f64 - 1.0) as usize + 1) {
                sum += f_x.fast_eval(variable, interval.0 + (((2.0 * j as f64) - 1.0) * h));
            }
            0.5 * (T[i - 1]) + (h * sum)
        };
        T.push(tmp);
        R.push(vec![T[i]]);
    }

    for i in 1..(i_max + 1) {
        for j in i..(i_max + 1) {
            let tmp_r_j = R[j].clone();
            let tmp_r_j_minus = R[j - 1].clone();
            R[j].push(((4.0_f64.powf(i as f64) * tmp_r_j[i - 1]) - tmp_r_j_minus[i - 1]) / (4.0_f64.powf(i as f64) - 1.0));
        }
    }

    Ok(R[i_max][i_max])
}