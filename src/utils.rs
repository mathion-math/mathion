// =======================================================================
//  Copyleft mathion 2018-∞.
//  Distributed under the terms of the CC0 “No Rights Reserved”.
//  (See accompanying file COPYING or copy at
//   https://creativecommons.org/publicdomain/zero/1.0/)
// =======================================================================

//* Use from external library *//
use std::mem;

pub fn string_to_static_str(s: String) -> &'static str {
    unsafe {
        let ret = mem::transmute(&s as &str);
        mem::forget(s);
        ret
    }
}

pub fn vec_to_static_slice<T>(s: Vec<T>) -> &'static [T] {
    unsafe {
        let ret = mem::transmute(&s as &[T]);
        mem::forget(s);
        ret
    }
}

pub fn vec_to_static_mut_slice<T>(mut s: Vec<T>) -> &'static mut [T] {
    unsafe {
        let ret = mem::transmute(&mut s as &mut [T]);
        mem::forget(s);
        ret
    }
}