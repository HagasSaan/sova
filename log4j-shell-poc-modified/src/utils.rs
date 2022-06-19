use std::ffi::CStr;
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

pub unsafe fn from_arr_ptr_to_vec(
    c_array_pointer: *const *const c_char
) -> Option<Vec<String>> {
    if c_array_pointer.is_null() {
        return None
    }

    let array_size = get_array_size(c_array_pointer);

    let rust_vec = slice::from_raw_parts(
        c_array_pointer,
        array_size
    ).to_vec();

    let rust_vec: Vec<String> = rust_vec
        .into_iter()
        .map(
            |el| {
                CStr::from_ptr(el)
                    .to_str()
                    .expect("not correct utf8")
                    .into()
            }
        )
        .collect();

    Some(rust_vec)
}

pub unsafe fn from_pointer_to_string(path_clone: *const c_char) -> String {
    CStr::from_ptr(path_clone)
        .to_str()
        .expect("not correct utf8")
        .into()
}
