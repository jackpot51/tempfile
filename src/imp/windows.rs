use std::os::windows::fs::OpenOptionsExt;
use std::os::windows::io::{FromRawHandle, AsRawHandle, RawHandle};
use std::path::Path;
use std::io;
use std::fs::{File, OpenOptions};
use ::libc::{self, DWORD, HANDLE};
use ::util::tmpname;

const ACCESS: DWORD     = libc::FILE_GENERIC_READ
                        | libc::FILE_GENERIC_WRITE;
const SHARE_MODE: DWORD = libc::FILE_SHARE_DELETE
                        | libc::FILE_SHARE_READ
                        | libc::FILE_SHARE_WRITE;
const FLAGS: DWORD      = libc::FILE_ATTRIBUTE_HIDDEN
                        | libc::FILE_ATTRIBUTE_TEMPORARY
                        | libc::FILE_FLAG_DELETE_ON_CLOSE; 

extern "system" {
    // TODO: move to external crate.
    fn ReOpenFile(hOriginalFile: HANDLE,
                  dwDesiredAccess: DWORD,
                  dwShareMode: DWORD,
                  dwFlags: DWORD) -> HANDLE;
}

pub fn create(dir: &Path) -> io::Result<File> {

    let mut opts = OpenOptions::new();
    opts.desired_access(ACCESS as i32)
        .share_mode(SHARE_MODE as i32)
        .creation_disposition(libc::CREATE_NEW as i32)
        .flags_and_attributes(FLAGS as i32);

    loop {
        return match opts.open(dir.join(&tmpname())) {
            Ok(f) => Ok(f),
            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => Err(e),
        };
    }
}

pub fn reopen(f: &File) -> io::Result<File> {
    let h = f.as_raw_handle();
    unsafe {
        let h = ReOpenFile(h as HANDLE, ACCESS, SHARE_MODE, 0);
        if h == libc::INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(FromRawHandle::from_raw_handle(h as RawHandle))
        }
    }
}
