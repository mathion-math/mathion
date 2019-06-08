// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from local library *//
use symbols::Polynomial;
use matrix::Matrix;

// Gauss elimination - a equal to b (b_ij, i = 1)
/*pub fn gauss(a: Matrix, b: Matrix) -> Variables {

}*/

pub fn solve_by_lu(a: Matrix, b: Matrix) -> Matrix {
    assert_eq!(a.rows, a.columns); // Is a square matrix?

    let (l, u) = a.lu();
    println!("{} {}", l, u);
    let c: Matrix = {
        let mut tmp: Matrix = Matrix::blank(a.size(), 1);
        for i in 0..a.size() {
            let sum = {
                let mut tmp2: Polynomial = Polynomial::blank();
                for j in 0..(a.size() - 1) {
                    tmp2 = tmp2 + (l.nth(i, j) * tmp.nth(j, 0));
                }
                tmp2
            };
            tmp.set(i, 0, (b.nth(i, 0) - sum) / l.nth(i, i));
        }
        tmp
    };

    let mut x: Matrix = Matrix::blank(a.size(), 1);
    for i in 0..a.size() {
        let sum = {
            let mut tmp: Polynomial = Polynomial::blank();
            for j in 0..i {
                tmp = tmp + (u.nth(a.size() - i - 1, a.size() - j - 1) * x.nth(a.size() - j - 1, 0));
            }
            tmp
        };
        x.set(a.size() - i - 1, 0, c.nth(a.size() - i - 1, 0) - sum);
    }
    x
}