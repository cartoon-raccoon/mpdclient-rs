#![allow(non_upper_case_globals)]

use std::ffi::{CString, c_char, CStr};
use std::ptr;
use std::io::{Error as IoError, ErrorKind};

use libmpdclient_sys::*;

use crate::{MpdError, MpdErrorKind, Result, ServerError};

pub struct RawMpdClient {
    conn: *mut mpd_connection,
}

impl RawMpdClient {
    pub fn new_default() -> Result<Self> {
        Self::_new(None, None, None)
    }

    pub fn new(host: &str, port: u16, timeout: u32) -> Result<Self> {
        let host = CString::new(host)
            .expect("hostname should not have null bytes");
        Self::_new(Some(host.as_ptr()), Some(port), Some(timeout))
    }

    fn _new(host: Option<*const c_char>, port: Option<u16>, timeout: Option<u32>) -> Result<Self> {
        let host = if let Some(host) = host { host } else { ptr::null() };
        let port = if let Some(port) = port { port } else { 0 };
        let timeout = if let Some(timeout) = timeout { timeout } else { 0 };

        // SAFETY: this struct owns this pointer and frees it when dropped
        let conn = unsafe { //todo: better error handling - make this return error
            let conn = mpd_connection_new(host, port as u32, timeout);
            if conn.is_null() {
                std::process::abort();
            }

            conn
        };

        let mut ret = Self { conn };
        ret.handle_error()?;

        Ok(ret)
    }

    /// If a mpdclient function returns an error indication,
    /// this gets the error string from the underlying connection.
    fn get_error_msg(&mut self) -> String {
        let cstr = unsafe {
            let raw = mpd_connection_get_error_message(self.conn);
            assert!(!raw.is_null());
            /* SAFETY:
            ptr should be valid as it was returned by a C function

            we assert that ptr is non-null

            we get a *const i8 which cannot be mutated, and immediately
            wrap it in a CStr, which is then immediately converted into an
            owned CString, so it is not mutated */
            CString::from(CStr::from_ptr(raw))
        };

        cstr.into_string().expect("error message did not contain valid UTF-8")
    }

    /// Handle any error that might crop up.
    /// 
    /// Aborts on MPD_ERROR_OOM.
    fn handle_error(&mut self) -> Result<()> {
        let errcode = unsafe {mpd_connection_get_error(self.conn)};

        if errcode == mpd_error_MPD_ERROR_SUCCESS {
            return Ok(())
        }

        // all rust strings are valid UTF-8
        let errmsg = self.get_error_msg();

        match errcode {
            mpd_error_MPD_ERROR_OOM => std::process::abort(),
            mpd_error_MPD_ERROR_ARGUMENT => {
                // todo handle this internally
            }
            mpd_error_MPD_ERROR_STATE => {
                // todo handle this internally
            },
            mpd_error_MPD_ERROR_TIMEOUT => return Err(
                MpdError {
                    kind: MpdErrorKind::Io(IoError::from(ErrorKind::TimedOut)), 
                    errmsg,
                }
            ),
            mpd_error_MPD_ERROR_SYSTEM => {
                let errno = unsafe {mpd_connection_get_system_error(self.conn)};
                return Err(MpdError {
                    kind: MpdErrorKind::Io(IoError::from_raw_os_error(errno)),
                    errmsg,
                })
            }
            mpd_error_MPD_ERROR_RESOLVER => return Err(
                MpdError {
                    kind: MpdErrorKind::UnknownHost,
                    errmsg,
                }
            ),
            mpd_error_MPD_ERROR_CLOSED => return Err(
                MpdError {
                    kind: MpdErrorKind::Io(IoError::from(ErrorKind::ConnectionAborted)),
                    errmsg,
                }
                
            ),
            mpd_error_MPD_ERROR_SERVER => return Err(
                MpdError {
                    kind: MpdErrorKind::Server(self.process_server_error()),
                    errmsg,
                }
            ),
            _ => unreachable!()
        }

        Ok(())
    }

    fn process_server_error(&mut self) -> ServerError {
        todo!()
    }
}

impl Drop for RawMpdClient {
    fn drop(&mut self) {
        // SAFETY: we own this data and don't give out any
        // references to it, so we don't risk any double
        // frees or UAFs
        unsafe {
            mpd_connection_free(self.conn);
        }
    }
}