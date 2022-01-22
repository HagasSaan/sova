use libc::c_char;
use std::slice;

pub unsafe fn get_array_size(argv_clone: *const *const c_char) -> usize {
    let mut array_size = 0;
    let mut p = argv_clone;
    while *p != std::ptr::null() {
        array_size += 1;
        p = p.offset(1);
    }

    array_size
}

pub unsafe fn from_c_array_pointer_to_rust_vec(
    c_array_pointer: *const *const c_char
) -> Vec<*const c_char> {
    let array_size = get_array_size(c_array_pointer);

    let rust_vec = slice::from_raw_parts(
        c_array_pointer,
        array_size
    ).to_vec();

    rust_vec
}
