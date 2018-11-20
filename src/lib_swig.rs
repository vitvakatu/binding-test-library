use std::i32;

use jni_c_header::*;

/// Returns absolute value of a number
fn abs_internal(i: i32) -> i32 {
    i.abs()
}

/// Finds maximum value in the slice
fn max_internal(slice: &[i32]) -> i32 {
    slice.iter().max().cloned().unwrap_or(i32::MIN)
}

/// Creates simple vector with desired size filled with `0, 1, 2, ... N-1`
fn create_vec_internal(size: usize) -> Vec<i8> {
    (0..size).map(|x| x as i8).collect()
}

pub trait Callback {
    fn callback(&self, _: i32);
}

foreign_interface!(interface RustCallback {
    self_type Callback;
    callback = Callback::callback(&self, _: i32);
});

pub fn use_callback(cb: Box<Callback>, value: i32) {
    cb.callback(value)
}

foreigner_class!(class Binding {
    self_type Binding;
    static_method abs_internal(_: i32) -> i32; alias abs;
    static_method max_internal(_: &[i32]) -> i32; alias max;
    static_method create_vec_internal(_: usize) -> Vec<i8>; alias createVec;
    static_method use_callback(_: Box<Callback>, _: i32); alias useCallback;
});

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
