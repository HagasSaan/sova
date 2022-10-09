use std::ffi::CStr;
use std::mem;
use std::time::Instant;
use libc::c_int;
use log::{info, warn};

use crate::{configuration, utils};
use crate::analyzer::Analyzer;
use crate::behaviour::Behaviour;
use crate::logger::setup_logger;
use crate::record::Record;

lazy_static! {
    static ref ORIGINAL_EXECVE: extern fn(
        *const libc::c_char,
        *const *const libc::c_char,
        *const *const libc::c_char,
    ) -> c_int = unsafe {
        let fn_name = CStr::from_bytes_with_nul(b"execve\0").unwrap();
        let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

        mem::transmute(fn_ptr)
    };
}


#[no_mangle]
pub unsafe extern fn execve(
    path: *const libc::c_char,
    argv: *const *const libc::c_char,
    envp: *const *const libc::c_char,
) -> c_int {
    info!("execve ran");

    let start_time = Instant::now();

    let configuration = configuration::load_configuration();

    match setup_logger(&configuration.logfile_path) {
        Ok(_) => {},
        Err(e) => {
            println!("Could not setup logger: {:?}, path: {:?}", e, &configuration.logfile_path);
        },
    }

    let path_str = utils::from_pointer_to_string(path.clone());
    let argv_vec_str = utils::from_arr_ptr_to_vec(argv.clone());
    let envp_vec_str = utils::from_arr_ptr_to_vec(envp.clone());

    let record: Record = Record {
        path: path_str,
        argv: argv_vec_str,
        envp: envp_vec_str,
    };

    let analyzer = Analyzer::new(configuration);

    let behaviour = analyzer.analyze(record);

    info!("Analysis done with {} seconds", start_time.elapsed().as_secs_f64());

    match behaviour {
        Behaviour::KillProcess => {
            warn!("Killing process");
            return -1;
        },
        Behaviour::LogOnly => {
            info!("Logging only");
        },
    };

    ORIGINAL_EXECVE(path, argv, envp)
}
