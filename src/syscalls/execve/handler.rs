use std::error::Error;
use std::ffi::CStr;
use std::mem;
use std::time::Instant;

use log::{debug, info, warn};

use crate::syscalls::common::analyzer::Analyzer;
use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::logger::setup_logger;
use crate::syscalls::common::{configuration, utils};
use crate::syscalls::execve::record::Record;

lazy_static! {
    static ref ORIGINAL_EXECVE: extern "C" fn(
        *const libc::c_char,
        *const *const libc::c_char,
        *const *const libc::c_char,
    ) -> libc::c_int = unsafe {
        let fn_name = CStr::from_bytes_with_nul(b"execve\0").unwrap();
        let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

        mem::transmute(fn_ptr)
    };
}

#[no_mangle]
pub unsafe extern "C" fn execve(
    pathname: *const libc::c_char,
    argv: *const *const libc::c_char,
    envp: *const *const libc::c_char,
) -> libc::c_int {
    let start_time = Instant::now();

    let configuration = configuration::load_configuration();

    match setup_logger(&configuration.logfile_path) {
        Ok(_) => {}
        Err(e) => {
            debug!(
                "Could not setup logger: {:?}, path: {:?}, call: execve",
                e.source(),
                &configuration.logfile_path
            );
        }
    }

    info!("execve ran");

    let pathname_str = utils::from_pointer_to_string(pathname);
    let argv_vec_str = utils::from_arr_ptr_to_vec(argv);
    let envp_vec_str = utils::from_arr_ptr_to_vec(envp);

    let record: Record = Record {
        pathname: pathname_str,
        argv: argv_vec_str,
        envp: envp_vec_str,
    };

    let analyzer = Analyzer::new(configuration.rules.execve);

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

    ORIGINAL_EXECVE(pathname, argv, envp)
}
