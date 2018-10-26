extern crate libc;
use std::slice;
use std::mem;

fn abs_internal(i: i32) -> i32 {
    i.abs()
}

extern fn abs(i: libc::c_int) -> libc::c_int {
    abs_internal(i)
}

fn max_internal(slice: &[u8]) -> u8 {
    slice.iter().max().cloned().unwrap_or(0)
}

extern fn max(slice: *const u8, count: libc::size_t) -> libc::c_int {
    let slice = unsafe {
        slice::from_raw_parts(slice, count)
    };

    max_internal(slice) as libc::c_int
}

fn create_vec_internal() -> Vec<u8> {
    vec![1, 2, 3]
}

extern fn create_vec() -> *const u8 {
    let mut vec = create_vec_internal();

    vec.shrink_to_fit();
    let ptr = vec.as_ptr();

    mem::forget(vec);

    ptr
}

extern fn use_callback(callback: extern fn(libc::c_int) -> libc::c_int, value: libc::c_int) -> libc::c_int {
    callback(value)
}

extern fn return_callback() -> extern fn(libc::c_int) -> libc::c_int {
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
    }

    #[test]
    fn create_vec_test() {
        assert_eq!(vec![1, 2, 3], create_vec_internal());
    }
}