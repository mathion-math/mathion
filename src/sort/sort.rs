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

fn pyramidize_min(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return arr;
    }
    let mut array = arr.clone();
    let h0: usize = (array.len() as f64).log2().floor() as usize + 1;
    let mut i = (array.len() as f64 / 2.0).ceil() as usize;
    loop {
        array = heapify_min(array.clone(), i);
        if i == 0 {
            break;
        }
        i -= 1;
    }
    let mut X: Vec<usize> = array.to_vec();

        for i in 1..=h0 {
            let pc1 = 2_i32.pow((i - 1) as u32) as usize;
            let pc2 = pc1 << 1;
            if i == h0 {
                let mut clone_array = array.to_vec();
                let tmp: Vec<usize> = clone_array.drain((pc1 - 1)..=(array.len() - 1)).collect();
                let H_prime = pyramidize_min(tmp);
                for j in (pc1 - 1)..=(array.len() - 1) {
                    X[j] = H_prime[j + 1 - pc1];
                }
            } else {
                let mut clone_array = array.to_vec();
                let tmp: Vec<usize> = clone_array.drain((pc1 - 1)..=(pc2 - 2)).collect();
                let H_prime = pyramidize_min(tmp);
                for j in (pc1 - 1)..=(pc2 - 2) {
                    X[j] = H_prime[j + 1 - pc1];
                }
            }
        }
    for i in 1..=(h0 - 1) {
        let pc1 = 2_i32.pow((i - 1) as u32) as usize;
        let pc2 = pc1 << 1;
        let pc3 = pc2 << 1;
        if i == h0 - 1 {
            let mut clone_x1 = X.clone();
            let mut clone_x2 = X.clone();
            let mut tmp1: Vec<usize> = clone_x1.drain((pc1 - 1)..=(pc2 - 2)).collect();
            let mut tmp2: Vec<usize> = clone_x2.drain((pc2 - 1)..=(array.len() - 1)).collect();
            let M = merge(tmp1, tmp2);
            for j in (pc1 - 1)..=(pc2 - 2) {
                X[j] = M[j + 1 - pc1];
            }
            for j in (pc2 - 1)..=(array.len() - 1) {
                X[j] = M[j + 1 - pc1];
            }
        } else {
            let mut clone_x1 = X.clone();
            let mut clone_x2 = X.clone();
            let mut tmp1: Vec<usize> = clone_x1.drain((pc1 - 1)..=(pc2 - 2)).collect();
            let mut tmp2: Vec<usize> = clone_x2.drain((pc2 - 1)..=(pc3 - 2)).collect();
            let M = merge(tmp1, tmp2);
            for j in (pc1 - 1)..=(pc2 - 2) {
                X[j] = M[j + 1 - pc1];
            }
            for j in (pc2 - 1)..=(pc3 - 2) {
                X[j] = M[j + 1 - pc1];
            }
        }
    }
    X
}

fn find_min(pyramid: Vec<usize>, h: usize, h0: usize) -> usize {
    pyramid[2_i32.pow( (h0 - h) as u32) as usize - 1]
}

fn find_max(pyramid: Vec<usize>, h: usize, h0: usize) -> usize {
    if h > 1 {
        pyramid[2_i32.pow( (h0 - 1 - h) as u32) as usize - 2]
    } else {
        pyramid[pyramid.len() - 1]
    }
}

fn insert(pyramid: Vec<usize>, x: usize) -> Vec<usize> {
    merge(pyramid.to_vec(), [x].to_vec())
}

fn heapify_min(heap: Vec<usize>, index: usize) -> Vec<usize> {
    let mut tmp = heap.clone();
    let mut least = index;
    if 2*index + 1 < heap.len() && tmp[least] > tmp[2*index + 1] {
        least = 2*index + 1;
    }
    if 2*index + 2 < heap.len() && tmp[least] > tmp[2*index + 2] {
        least = 2*index + 2;
    }
    if least != index {
        tmp.swap(index, least);
        tmp = heapify_min(tmp, least);
    }
    tmp
}

fn merge(x: Vec<usize>, y: Vec<usize>) -> Vec<usize> {
    if x.len() == 0 { return y; }
    if y.len() == 0 { return x; }
    if x[0] <= y[0] {
        let mut v = x.clone();
        let u: Vec<_> = (v.drain(1..).collect());
        v.extend_from_slice(&merge(u, y));
        return v.clone();
    } else {
        let mut v = y.clone();
        let u: Vec<_> = (v.drain(1..).collect());
        v.extend_from_slice(&merge(x, u));
        return v.clone();
    }
}