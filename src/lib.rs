extern crate libc;

#[macro_use]
extern crate log;

mod jni_c_header;

include!(concat!(env!("OUT_DIR"), "/lib.rs"));