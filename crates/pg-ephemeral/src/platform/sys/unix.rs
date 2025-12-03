use libc::{c_char, geteuid, gethostname, getpwuid};
use std::{ffi::CStr, io};

use super::{SysInfo, SysT};

#[derive(Debug)]
pub struct Sys {
    /// User who is responsible for running current process
    user: String,

    /// Host name
    hostname: String,

    /// Checking whether the user is root
    is_root: bool,
}

impl SysT for Sys {
    fn new() -> std::io::Result<Self> {
        let user = Self::current_username()?;
        let hostname = Self::hostname()?;
        let is_root = Self::is_root();

        Ok(Self {
            user,
            hostname,
            is_root,
        })
    }

    fn spawn() {}

    fn kill() {}
}

impl SysInfo for Sys {
    fn user(&self) -> String {
        self.user.clone()
    }

    fn sysname(&self) -> String {
        self.hostname.clone()
    }

    fn has_root_privilege(&self) -> bool {
        self.is_root
    }
}

impl Sys {
    fn current_username() -> io::Result<String> {
        unsafe {
            let uid = geteuid();
            let pwd = getpwuid(uid);

            if pwd.is_null() {
                return Err(io::Error::other("pwd is not valid"));
            }

            let name_ptr = (*pwd).pw_name;

            if name_ptr.is_null() {
                return Err(io::Error::other("failed to retrieve the user"));
            }

            let user = CStr::from_ptr(name_ptr)
                .to_str()
                .ok()
                .map(|s| s.to_string())
                .ok_or_else(|| io::Error::other("failed to decode the user name pointer"))?;

            Ok(user)
        }
    }

    fn hostname() -> io::Result<String> {
        unsafe {
            let mut buffer = ['\0' as c_char; 256];

            if gethostname(buffer.as_mut_ptr(), buffer.len()) != 0 {
                return Err(io::Error::last_os_error());
            }

            let host_str = CStr::from_ptr(buffer.as_ptr() as *const i8);
            let host = host_str
                .to_str()
                .ok()
                .map(|s| s.to_string())
                .ok_or_else(|| io::Error::other("failed to decode the hostname pointer"))?;

            Ok(host)
        }
    }

    fn is_root() -> bool {
        unsafe { libc::geteuid() == 0 }
    }
}
