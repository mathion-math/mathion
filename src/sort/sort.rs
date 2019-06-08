// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

use std::cmp::Ordering;
use std::cmp::Ordering::Greater;

pub struct HeapSort<F, T>
    where T: std::clone::Clone, F: Fn(&T, &T) -> Ordering,
{
    HeapSize: usize,
    Array: Box<[T]>,
    Compare: F,
}

impl<F, T> HeapSort<F, T>
    where T: std::clone::Clone, F: Fn(&T, &T) -> Ordering,
{
    /// The heap sort algorithm has time complexify O(nlogn)
    pub fn new(array: Vec<T>, compare: F) -> HeapSort<F, T> {
        HeapSort {
            HeapSize: array.len(),
            Array: array.into_boxed_slice(),
            Compare: compare,
        } 
    }

    pub fn sort(&mut self) -> Vec<T> {
        if self.Array.len() != 1 && self.Array.len() != 0 {
            for i in ((self.HeapSize / 2) as usize - 1)..=0 {
                &self.heapify(i);
            }
            for i in (self.HeapSize-1)..0 {
                (*(self.Array)).swap(i, 0);
                self.HeapSize = i;
                &self.heapify(0);
            }
        }
        self.Array.to_vec()
    }

    fn at(&self, i: usize) -> T {
        self.Array[i].clone()
    }

    fn heapify(&mut self, i: usize) {
        let (left, right) = (2*i, 2*i + 1);
        let mut target = i;

        if self.HeapSize > left && (self.Compare)(&self.at(i), &self.at(left)) == Greater {
            target = left;
        }

        if self.HeapSize > right && (self.Compare)(&self.at(i), &self.at(right)) == Greater {
            target = right;
        }

        if target != i {
            &self.Array.swap(i, target);
            &self.heapify(target);
        }
    }
}