extern crate libc;

use std::{i32, slice};

/// Returns absolute value of a number
fn abs_internal(i: i32) -> i32 {
    i.abs()
}

/// Finds maximum value in the slice
fn max_internal(slice: &[i32]) -> i32 {
    slice.iter().max().cloned().unwrap_or(i32::MIN)
}

/// Creates simple vector with desired size filled with `0, 1, 2, ... N-1`
fn create_vec_internal(size: usize) -> Vec<u8> {
    (0..size).map(|x| x as u8).collect()
}

// *************************************************
//                  Here goes C-API
// *************************************************

#[no_mangle]
pub extern "C" fn abs(i: libc::c_int) -> libc::c_int {
    abs_internal(i)
}

#[no_mangle]
pub extern "C" fn max(slice: *const libc::c_int, count: libc::size_t) -> libc::c_int {
    let slice = unsafe { slice::from_raw_parts(slice, count) };

    max_internal(slice)
}

#[no_mangle]
pub extern "C" fn fill_vec(buffer: *mut libc::c_uchar, size: libc::size_t) {
    let vec = create_vec_internal(size as usize);

    let slice = unsafe { slice::from_raw_parts_mut(buffer, size as usize) };
    slice.copy_from_slice(&vec);
}

#[no_mangle]
pub extern "C" fn use_callback(
    callback: extern "C" fn(libc::c_int) -> libc::c_int,
    value: libc::c_int,
) -> libc::c_int {
    callback(value)
}

#[no_mangle]
pub extern "C" fn return_callback() -> extern "C" fn(libc::c_int) -> libc::c_int {
    abs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_test() {
        assert_eq!(0, abs_internal(0));
        assert_eq!(100, abs_internal(100));
        assert_eq!(100, abs_internal(-100));
    }

    #[test]
    fn max_test() {
        assert_eq!(0, max_internal(&[]));
        assert_eq!(1, max_internal(&[1]));
        assert_eq!(1, max_internal(&[1, 1]));
        assert_eq!(3, max_internal(&[1, 3, 2]));
        assert_eq!(5, max_internal(&[1, 2, 5]));
    }

    #[test]
    fn create_vec_test() {
        assert_eq!(vec![0, 1, 2], create_vec_internal(3));
    }
}
