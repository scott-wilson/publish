use std::{
    ffi::{c_char, CStr, CString},
    ptr::null,
};

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum CPublishStatusType {
    CPublishStatusTypeOk,
    CPublishStatusTypeError,
}

#[repr(C)]
pub struct CPublishStatus {
    pub status: CPublishStatusType,
    pub message: *const c_char,
}

impl Drop for CPublishStatus {
    fn drop(&mut self) {
        unsafe {
            if !self.message.is_null() {
                drop(CString::from_raw(self.message as *mut _));
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_status_ok(status: *mut CPublishStatus) {
    if let Some(status) = status.as_mut() {
        // We need to replace the current status value with a new one without
        // dropping the old one, otherwise we may get a segfault.
        let old_value = std::mem::replace(status, CPublishStatus::new_ok());
        std::mem::forget(old_value);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_status_error(
    status: *mut CPublishStatus,
    message: *const c_char,
) {
    if let Some(status) = status.as_mut() {
        let message = CStr::from_ptr(message).to_string_lossy();
        let old_value = std::mem::replace(status, CPublishStatus::new_error(message));
        std::mem::forget(old_value);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_status_destroy(status: *mut CPublishStatus) {
    if !status.is_null() {
        status.drop_in_place();
    }
}

impl CPublishStatus {
    pub fn new_ok() -> Self {
        Self {
            status: CPublishStatusType::CPublishStatusTypeOk,
            message: null(),
        }
    }

    pub fn new_error<T: AsRef<str>>(message: T) -> Self {
        let message = CString::new(message.as_ref()).unwrap();
        Self {
            status: CPublishStatusType::CPublishStatusTypeError,
            message: message.into_raw(),
        }
    }
}
