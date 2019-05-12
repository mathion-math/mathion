// =======================================================================
//  Copyleft physics-rs 2018-∞.
//  Distributed under the terms of the Unlicense.
//  (See accompanying file UNLICENSE or copy at
//   https://unlicense.org/)
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