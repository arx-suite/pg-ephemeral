use std::os::windows::raw::HANDLE;
use std::{io, mem, ptr};

use windows_sys::Win32::Security::{
    CreateRestrictedToken, DISABLE_MAX_PRIVILEGE, GetTokenInformation, LUA_TOKEN, TOKEN_DUPLICATE,
    TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};
use windows_sys::Win32::System::WindowsProgramming::{GetComputerNameA, GetUserNameW};

use super::{SysInfo, SysT};

#[derive(Debug)]
pub struct Sys {
    /// Username of the account running the current process
    user: String,

    /// Machine name
    sysname: String,

    /// Handle to the current process
    process_handle: HANDLE,

    /// Token representing the current process user
    parent_token: HANDLE,

    /// Whether the current user is running with elevated/administrative privileges
    is_elevated: bool,

    /// Access token used for spawning the PostgreSQL child process
    /// If [`Sys::is_elevated`] is true, this will be a restricted token.
    /// Otherwise, this will reuse [`Sys::parent_token`]
    access_token: Option<HANDLE>,

    /// Handle to the spawned PostgreSQL child process, if any
    /// Only a single child process is managed at a time
    child: Option<HANDLE>,
}

impl SysT for Sys {
    fn new() -> io::Result<Self> {
        // SAFETY: current process handle always be valid, until the process end
        let process_handle = unsafe { GetCurrentProcess() };

        // sanity check
        if process_handle.is_null() {
            return Err(io::Error::other(
                "current process handle must have a valid pointer to themself",
            ));
        }

        let token_handle = Self::current_process_token(process_handle)?;
        let is_elevated = Self::is_elevated(token_handle)?;

        let access_token = if is_elevated {
            Some(Self::create_restricted_token(token_handle)?)
        } else {
            None
        };

        Ok(Self {
            user: Self::get_username()?,
            sysname: Self::get_sysname()?,
            parent_token: token_handle,
            access_token,
            process_handle,
            is_elevated,
            child: None,
        })
    }

    fn spawn() {}

    fn kill() {}
}

impl SysInfo for Sys {
    fn sysname(&self) -> String {
        self.sysname.clone()
    }

    fn user(&self) -> String {
        self.user.clone()
    }

    fn has_root_privilege(&self) -> bool {
        self.is_elevated
    }
}

impl Sys {
    fn get_username() -> io::Result<String> {
        let mut buffer = [0u16; 256];
        let mut size = buffer.len() as u32;

        unsafe {
            if GetUserNameW(buffer.as_mut_ptr(), &mut size) == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(String::from_utf16_lossy(&buffer[..(size - 1) as usize]))
        }
    }

    fn get_sysname() -> io::Result<String> {
        let mut buffer = [0u8; 256];
        let mut size = buffer.len() as u32;

        unsafe {
            if GetComputerNameA(buffer.as_mut_ptr(), &mut size) == 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(String::from_utf8_lossy(&buffer[..(size - 1) as usize]).to_string())
        }
    }

    fn current_process_token(process_handle: HANDLE) -> io::Result<HANDLE> {
        let mut token: HANDLE = ptr::null_mut();

        unsafe {
            if OpenProcessToken(process_handle, TOKEN_QUERY | TOKEN_DUPLICATE, &mut token) == 0 {
                return Err(io::Error::last_os_error());
            }
        }

        if token.is_null() {
            return Err(io::Error::other(
                "current process token handle is not valid",
            ));
        }

        Ok(token)
    }

    fn create_restricted_token(token_handle: HANDLE) -> io::Result<HANDLE> {
        let mut new_token: HANDLE = ptr::null_mut();

        unsafe {
            if CreateRestrictedToken(
                token_handle,
                DISABLE_MAX_PRIVILEGE | LUA_TOKEN,
                0,
                ptr::null(),
                0,
                ptr::null(),
                0,
                ptr::null(),
                &mut new_token,
            ) == 0
            {
                return Err(io::Error::last_os_error());
            }
        }

        if new_token.is_null() {
            return Err(io::Error::other("new token handle is not valid"));
        }

        Ok(new_token)
    }

    fn is_elevated(token_handle: HANDLE) -> io::Result<bool> {
        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = 0;

        unsafe {
            if GetTokenInformation(
                token_handle,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            ) == 0
            {
                return Err(io::Error::last_os_error());
            }
        };

        Ok(elevation.TokenIsElevated != 0)
    }
}
