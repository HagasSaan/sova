use std::error::Error;
use std::ffi::CStr;
use std::mem;
use std::time::Instant;

use log::{info, warn};

use crate::syscalls::common::analyzer::Analyzer;
use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::logger::setup_logger;
use crate::syscalls::common::{configuration, utils};
use crate::syscalls::open::record::Record;

lazy_static! {
    static ref ORIGINAL_OPEN: extern "C" fn(*const libc::c_char, libc::c_int, mode: libc::mode_t) -> libc::c_int = unsafe {
        let fn_name = CStr::from_bytes_with_nul(b"open\0").unwrap();
        let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

        mem::transmute(fn_ptr)
    };
}

#[no_mangle]
pub unsafe extern "C" fn open(pathname: *const libc::c_char, flags: libc::c_int, mode: libc::mode_t) -> libc::c_int {
    let start_time = Instant::now();

    let configuration = configuration::load_configuration();

    match setup_logger(&configuration.logfile_path) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Could not setup logger: {:?}, path: {:?}, call: open",
                e.source(),
                &configuration.logfile_path
            );
        }
    }

    info!("open ran");

    let pathname_str = utils::from_pointer_to_string(pathname);
    let record: Record = Record {
        pathname: pathname_str,
        flags,
        mode
    };

    let analyzer = Analyzer::new(configuration.rules.open);

    let behaviour = analyzer.analyze(record);

    info!(
        "Analysis done with {} seconds",
        start_time.elapsed().as_secs_f64()
    );

    match behaviour {
        Behaviour::KillProcess => {
            warn!("Killing process");
            return -1;
        }
        Behaviour::LogOnly => {
            info!("Logging only");
        }
    };

    let descriptor = ORIGINAL_OPEN(pathname, flags, mode);

    info!("Return value: {}", descriptor); // TODO: remove or move to analyzer

    descriptor
}
