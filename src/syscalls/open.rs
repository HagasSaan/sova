use std::ffi::CStr;
use std::mem;
use std::time::Instant;
use libc;
use log::info;

use crate::{configuration, utils};
use crate::logger::setup_logger;

lazy_static! {
    static ref ORIGINAL_READ: extern fn(
        *const libc::c_char,
        libc::c_int
    ) -> libc::c_int = unsafe {
        let fn_name = CStr::from_bytes_with_nul(b"open\0").unwrap();
        let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

        mem::transmute(fn_ptr)
    };
}


#[no_mangle]
pub unsafe extern fn open(
    pathname: *const libc::c_char,
    flags: libc::c_int,
) -> libc::c_int {
    let start_time = Instant::now();

    let configuration = configuration::load_configuration();

    match setup_logger(&configuration.logfile_path) {
        Ok(_) => {},
        Err(e) => {
            println!("Could not setup logger: {:?}, path: {:?}, call: open", e, &configuration.logfile_path);
        },
    }

    info!("open ran");

    let path_str = utils::from_pointer_to_string(pathname.clone());


    info!("Path: {}, flags: {}", path_str, flags); // TODO: move to analyzer


    info!("Analysis done with {} seconds", start_time.elapsed().as_secs_f64());

    let descriptor = ORIGINAL_READ(pathname, flags);

    info!("Return value: {}", descriptor);  // TODO: remove or move to analyzer

    descriptor
}
