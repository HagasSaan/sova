#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate log;

use std::ffi::CStr;
use std::mem;

use analyzer::Analyzer;
use behaviour::Behaviour;
use configuration::Configuration;
use logger::setup_logger;
use record::Record;
use rule::{ConditionType, Rule};
use log::{debug, error, info, trace, warn};

mod utils;
mod analyzer;
mod behaviour;
mod rule;
mod record;
mod configuration;
mod logger;

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
    let configuration = get_configuration();

    setup_logger("/tmp/sova.log").unwrap();
    info!("execv runned");
    let path_str = utils::from_pointer_to_string(path.clone());
    let argv_vec_str = utils::from_arr_ptr_to_vec(argv.clone());

    let record: Record = Record {
        path: path_str,
        argv: argv_vec_str,
        envp: None
    };

    let analyzer = Analyzer::new(configuration);

    let behaviour = analyzer.analyze(record);

    match behaviour {
        Behaviour::KillSystem => {
            warn!("Killing system");
            // TODO: kill system, really
        },
        Behaviour::KillProcess => {
            warn!("Killing process");
            return;
        },
        Behaviour::LogOnly => {
            info!("Logging only");
        },
    };

    ORIGINAL_EXECV(path, argv)
}

fn get_configuration() -> Configuration {
    let rules = vec![
        Rule {
            subject: String::from("path"),
            condition: ConditionType::MustBeIn,
            objects: vec![
                String::from("cat"),
                String::from("/bin/cat"),
                String::from("/usr/bin/cat"),
            ],
            behaviour_on_violation: Behaviour::KillProcess,
        }
    ];

    let configuration = Configuration {
        behaviour_on_incidents: Behaviour::KillSystem,
        rules,
    };

    configuration
}


lazy_static! {
    static ref ORIGINAL_EXECVE: extern fn(
        *const libc::c_char,
        *const *const libc::c_char,
        *const *const libc::c_char,
    ) = unsafe {
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
) {
    let configuration = get_configuration();

    setup_logger("/tmp/sova.log").unwrap();
    info!("execve runned");
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

    match behaviour {
        Behaviour::KillSystem => {
            warn!("Killing system");
            // TODO: kill system, really
        },
        Behaviour::KillProcess => {
            warn!("Killing process");
            return;
        },
        Behaviour::LogOnly => {
            info!("Logging only");
        },
    };

    ORIGINAL_EXECVE(path, argv, envp)
}
