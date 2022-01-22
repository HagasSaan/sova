#[macro_use]
extern crate lazy_static;
extern crate libc;

use std::ffi::{CStr, CString};
use std::{mem, slice};
use libc::c_char;

mod utils;

lazy_static! {
    static ref ORIGINAL_EXECV: extern fn(
        *const libc::c_char,
        *const *const libc::c_char
    ) = unsafe {
        let fn_name = CStr::from_bytes_with_nul(b"execv\0").unwrap();
        let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

        mem::transmute(fn_ptr)
    };
}

#[no_mangle]
pub unsafe extern fn execv(path: *const libc::c_char, argv: *const *const libc::c_char) {
    println!("execv runned");
    let path_clone = path.clone();
    let path_str = CStr::from_ptr(path_clone).to_str();
    println!("{:?}", path_str.expect("not correct utf8"));

    let argv_clone = argv.clone();

    let argv_vec_str = utils::from_c_array_pointer_to_rust_vec(argv_clone);

    let argv_vec_str = argv_vec_str
        .into_iter()
        .map(
            |el| {
                CStr::from_ptr(el)
                    .to_str()
                    .expect("not correct utf8")
            }
        );

    for argv_arg in argv_vec_str {
        println!("{:?}", argv_arg);
    }

    ORIGINAL_EXECV(path, argv)
}


// lazy_static! {
//     static ref ORIGINAL_EXECVE: extern fn(
//         *const libc::c_char,
//         *const *const libc::c_char,
//         *const *const libc::c_char,
//     ) = unsafe {
//         let fn_name = CStr::from_bytes_with_nul(b"execve\0").unwrap();
//         let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

//         mem::transmute(fn_ptr)
//     };
// }

// #[no_mangle]
// pub unsafe extern fn execve(
//     path: *const libc::c_char,
//     argv: *const *const libc::c_char,
//     envp: *const *const libc::c_char,
// ) {
//     println!("execve runned");
    
//     let path_clone = path.clone();
//     let rust_string = CStr::from_ptr(path_clone).to_str();
//     println!("{:?}", rust_string);
//     // let c_string = CString::new(rust_string).unwrap();
    
//     ORIGINAL_EXECVE(path, argv, envp)
// }
