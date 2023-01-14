use std::error::Error;
use std::ffi::CStr;
use std::mem;
use std::time::Instant;

use crate::syscalls::bind::analyzer::Analyzer;
use crate::syscalls::bind::record::Record;
use crate::syscalls::common::sockaddr_in::SockaddrIn;
use log::{info, warn};

use crate::syscalls::common::behaviour::Behaviour;
use crate::syscalls::common::configuration;
use crate::syscalls::common::logger::setup_logger;

lazy_static! {
    static ref ORIGINAL_BIND: extern "C" fn(libc::c_int, *const libc::sockaddr_in, libc::socklen_t) -> libc::c_int = unsafe {
        let fn_name = CStr::from_bytes_with_nul(b"bind\0").unwrap();
        let fn_ptr = libc::dlsym(libc::RTLD_NEXT, fn_name.as_ptr());

        mem::transmute(fn_ptr)
    };
}

#[no_mangle]
pub unsafe extern "C" fn bind(
    sockfd: libc::c_int,
    addr: *const libc::sockaddr_in,
    addrlen: libc::socklen_t,
) -> libc::c_int {
    let start_time = Instant::now();

    let configuration = configuration::load_configuration();

    match setup_logger(&configuration.logfile_path) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Could not setup logger: {:?}, path: {:?}, call: bind",
                e.source(),
                &configuration.logfile_path
            );
        }
    }

    info!("bind ran");

    let addr_struct = SockaddrIn::new(*addr);

    let record: Record = Record {
        sockfd,
        addr: addr_struct,
        addrlen,
    };

    let analyzer = Analyzer::new(configuration);

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

    ORIGINAL_BIND(sockfd, addr, addrlen)
}
