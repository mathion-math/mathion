// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

use super::symbols::Polynomial;

/* /// Example Vector ///
    Dimension: 3,
    Component: [3, 5, 6],
    UnitVector: normal3 // (x, y, z),
    isUnitVector: false
*/

pub struct Vector {
    Dimension: i8,
    Component: Vec<Polynomial>,
    UnitVector: Option<Vec<Vector>>,
    isUnitVector: bool,
}


impl Vector {
    pub fn new_hard(dimension: i8, component: Vec<Polynomial>, unit_vector: Option<Vec<Vector>>, is_unit_vector: bool) -> Self {
        assert_eq!(!is_unit_vector, dimension as usize == component.len());
        assert_eq!(component.len(), dimension as usize);
        if !is_unit_vector {
            let mut tmp_vector: Vec<Vector> = vec![];
            for unit_vector in unit_vector.unwrap() {
                assert_eq!(unit_vector.size(), Polynomial::from_f64(1.0));
                tmp_vector.push(unit_vector);
            }

            Vector {
                Dimension: dimension,
                Component: component,
                UnitVector: Some(tmp_vector),
                isUnitVector: is_unit_vector
            }
        } else {
            Vector {
                Dimension: dimension,
                Component: component,
                UnitVector: unit_vector,
                isUnitVector: is_unit_vector
            }
        }
    }

    pub fn size(&self) -> Polynomial {
        let mut l_square: Polynomial = Polynomial::blank();
        for c in &(self).Component {
            l_square = l_square + c.square();
        }
        l_square
    }

    pub fn dimension(&self) -> i8 {
        self.Dimension
    }

    pub fn component(self) -> Vec<Polynomial> {
        self.Component
    }

    pub fn unit_vector(self) -> Vec<Vector> {
        assert_eq!(self.UnitVector.is_some(), true);
        self.UnitVector.unwrap()
    }

    pub fn is_unit_vector(&self) -> bool {
        self.isUnitVector
    }
}