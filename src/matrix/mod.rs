// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
// =======================================================================

use symbols::{Symbol,  Monomial, Polynomial};
use utils::*;

pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub entry: &'static mut [&'static mut[Polynomial]]
}

impl Matrix {
    pub fn new(rows: usize, columns: usize, entry: Vec<Vec<Polynomial>>) -> Self {
        let mut result = vec![];

        for i in 0..rows {
            let mut entry_i: Vec<Polynomial> = vec![Polynomial::blank(); columns];
            if entry.len() != 0{
                for j in 0..columns {
                    entry_i[j] = entry[i][j];
                }
            }
            result.push(vec_to_static_mut_slice(entry_i));
        }
        Self {
            rows,
            columns,
            entry: vec_to_static_mut_slice(result)
        }
    }

    pub fn blank(row: usize, column: usize) -> Self {
        Matrix::new(row, column, (0..row).map(|_x| (0..column).map(|_x| Polynomial::blank()).collect()).collect())
    }

    pub fn nth(&self, row: usize, column: usize) -> Polynomial {
        self.entry[row][column]
    }

    pub fn det(&self) -> Result<Polynomial, ()> {
        if self.columns != self.rows { return Err(()); }

        let mut result = Polynomial::blank();
        let mut i = 0;

        for _ in 0..self.columns {
            let mut det_i1 = Polynomial::from_f64(1.0);
            for j in 0..self.rows {
                det_i1 = det_i1 * self.entry[j][i];
                i = if i + 1 == self.columns { 0 } else { i + 1 };
            }
            result = result + det_i1.sync();
            i = 0;
            let mut det_i2 = Polynomial::from_f64(1.0);;
            for j in 0..self.rows {
                det_i2 = det_i2 * self.entry[j][i];
                i = if i == 0 { self.columns - 1 } else { i - 1 };
            }
            result = result - det_i2.sync();
            i = 0;
        }
        Ok(result.equal())
    }

    pub fn transposed(&self) -> Self {
        let mut result: Vec<Vec<Polynomial>> = vec![vec![Polynomial::blank(); (&self).rows]; (&self).columns];
        for i in 0..self.columns {
            for j in 0..self.rows {
                result[i][j] = self.nth(j, i);
            }
        }
        Matrix::new(self.rows, self.columns, result)
    }

    /// LU by Crout method
    pub fn lu(&self) -> (Matrix, Matrix) {
        let n = self.rows - 1;
        let (l, u) = ( Matrix::new(self.rows, self.columns, vec![]), Matrix::new(self.rows, self.columns, vec![]) );
        for i in 0..=n {
            l.entry[i][0] = self.nth(i, 0);
            if i + 1 <= n && !l.nth(0, 0).is_zero() { u.entry[0][i + 1] = self.nth(0, i + 1) / l.nth(0, 0) };
            u.entry[i][i] = Polynomial::from_f64(1.0); // TODO: Remove it
        }

        
        for j in 1..=(n - 1) {
            for i in j..=n {
                let sum =  { let mut tmp = Polynomial::blank(); for k in 0..=(j - 1)  {  tmp = tmp + (l.nth(i, k) * u.nth(k, j)) }; tmp };
                l.entry[i][j] = self.nth(i, j) - sum;
            }
            for k in (j + 1)..=n {
                let sum =  { let mut tmp = Polynomial::blank(); for i in 0..=(j - 1)  {  tmp = tmp + (l.nth(j, i) * u.nth(i, k)) }; tmp };
                u.entry[j][k] = (self.nth(j, k) - sum) / l.nth(j, j);
            }
        }

        let sum =  { let mut tmp = Polynomial::blank(); for k in 0..=(n - 1)  {  tmp = tmp + l.nth(n, k) * u.nth(k, n) }; tmp };
        l.entry[n][n] = self.nth(n, n) - sum;

        (l, u)
    }

    pub fn output_string(&self) -> String {
        let mut output = String::new();
        for i in 0..self.rows {
            for j in 0..self.columns {
                output.push_str(&self.nth(i, j).output_string());
                if j != self.columns - 1 { output.push_str(" "); }
            }
            output.push_str("\n");
        }
        format!("{}", output)
    }

    pub fn set(&mut self, row: usize, column: usize, var: Polynomial) {
        self.entry[row][column] = var;
    }

    pub fn size(&self) -> usize {
        assert_eq!(self.rows, self.columns); // Is a square matrix?
        self.rows
    }

    /*pub fn remove_column(&self, column: usize) -> Self {
        
    }

    pub fn remove_row(&self, row: usize) -> Self {

    }

    pub fn cofactor(&self) -> Result<Self, ()> {
        
    }*/
}
