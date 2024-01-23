use std::{ffi::CStr, ptr::null_mut};

use crate::{cpublish_context_destroy, cpublish_status_ok, CPublishContext, CPublishStatus};

#[repr(C)]
pub struct CPublishBasePublish {
    pub pre_publish_fn: unsafe extern "C" fn(
        publish: *const Self,
        context: *const CPublishContext,
        status: *mut CPublishStatus,
    ) -> *mut CPublishContext,
    pub rollback_pre_publish_fn: unsafe extern "C" fn(
        publish: *const Self,
        context: *const CPublishContext,
        status: *mut CPublishStatus,
    ),
    pub publish_fn: unsafe extern "C" fn(
        publish: *const Self,
        context: *const CPublishContext,
        status: *mut CPublishStatus,
    ) -> *mut CPublishContext,
    pub rollback_publish_fn: unsafe extern "C" fn(
        publish: *const Self,
        context: *const CPublishContext,
        status: *mut CPublishStatus,
    ),
    pub post_publish_fn: unsafe extern "C" fn(
        publish: *const Self,
        context: *const CPublishContext,
        status: *mut CPublishStatus,
    ) -> *mut CPublishContext,
    pub rollback_post_publish_fn: unsafe extern "C" fn(
        publish: *const Self,
        context: *const CPublishContext,
        status: *mut CPublishStatus,
    ),
}

#[async_trait::async_trait]
impl publish::Publish for CPublishBasePublish {
    async fn pre_publish<'a>(
        &self,
        context: &'a publish::Context,
    ) -> Result<std::borrow::Cow<'a, publish::Context>, publish::Error> {
        let c_context = context as *const publish::Context as *const CPublishContext;
        let mut status = CPublishStatus::new_ok();

        let result = unsafe { cpublish_publish_pre_publish(self, c_context, &mut status) };

        let out_result = match status.status {
            crate::CPublishStatusType::CPublishStatusTypeOk => match unsafe { result.as_ref() } {
                Some(c_context) => Ok(std::borrow::Cow::Owned(c_context.inner.clone())),
                None => Ok(std::borrow::Cow::Borrowed(context)),
            },
            crate::CPublishStatusType::CPublishStatusTypeError => {
                let message = unsafe { CStr::from_ptr(status.message) };
                Err(publish::Error::new_publish(message.to_string_lossy(), None))
            }
        };

        if !result.is_null() {
            unsafe {
                cpublish_context_destroy(result);
            }
        }

        out_result
    }

    async fn rollback_pre_publish(&self, context: &publish::Context) -> Result<(), publish::Error> {
        let c_context = context as *const publish::Context as *const CPublishContext;
        let mut status = CPublishStatus::new_ok();

        unsafe { cpublish_publish_rollback_pre_publish(self, c_context, &mut status) };

        match status.status {
            crate::CPublishStatusType::CPublishStatusTypeOk => Ok(()),
            crate::CPublishStatusType::CPublishStatusTypeError => {
                let message = unsafe { CStr::from_ptr(status.message) };
                Err(publish::Error::new_publish(message.to_string_lossy(), None))
            }
        }
    }

    async fn publish<'a>(
        &self,
        context: &'a publish::Context,
    ) -> Result<std::borrow::Cow<'a, publish::Context>, publish::Error> {
        let c_context = context as *const publish::Context as *const CPublishContext;
        let mut status = CPublishStatus::new_ok();

        let result = unsafe { cpublish_publish_publish(self, c_context, &mut status) };

        let out_result = match status.status {
            crate::CPublishStatusType::CPublishStatusTypeOk => match unsafe { result.as_ref() } {
                Some(c_context) => Ok(std::borrow::Cow::Owned(c_context.inner.clone())),
                None => Ok(std::borrow::Cow::Borrowed(context)),
            },
            crate::CPublishStatusType::CPublishStatusTypeError => {
                let message = unsafe { CStr::from_ptr(status.message) };
                Err(publish::Error::new_publish(message.to_string_lossy(), None))
            }
        };

        if !result.is_null() {
            unsafe {
                cpublish_context_destroy(result);
            }
        }

        out_result
    }

    async fn rollback_publish(&self, context: &publish::Context) -> Result<(), publish::Error> {
        let c_context = context as *const publish::Context as *const CPublishContext;
        let mut status = CPublishStatus::new_ok();

        unsafe { cpublish_publish_rollback_publish(self, c_context, &mut status) };

        match status.status {
            crate::CPublishStatusType::CPublishStatusTypeOk => Ok(()),
            crate::CPublishStatusType::CPublishStatusTypeError => {
                let message = unsafe { CStr::from_ptr(status.message) };
                Err(publish::Error::new_publish(message.to_string_lossy(), None))
            }
        }
    }

    async fn post_publish<'a>(
        &self,
        context: &'a publish::Context,
    ) -> Result<std::borrow::Cow<'a, publish::Context>, publish::Error> {
        let c_context = context as *const publish::Context as *const CPublishContext;
        let mut status = CPublishStatus::new_ok();

        let result = unsafe { cpublish_publish_post_publish(self, c_context, &mut status) };

        let out_result = match status.status {
            crate::CPublishStatusType::CPublishStatusTypeOk => match unsafe { result.as_ref() } {
                Some(c_context) => Ok(std::borrow::Cow::Owned(c_context.inner.clone())),
                None => Ok(std::borrow::Cow::Borrowed(context)),
            },
            crate::CPublishStatusType::CPublishStatusTypeError => {
                let message = unsafe { CStr::from_ptr(status.message) };
                Err(publish::Error::new_publish(message.to_string_lossy(), None))
            }
        };

        if !result.is_null() {
            unsafe {
                cpublish_context_destroy(result);
            }
        }

        out_result
    }

    async fn rollback_post_publish(
        &self,
        context: &publish::Context,
    ) -> Result<(), publish::Error> {
        let c_context = context as *const publish::Context as *const CPublishContext;
        let mut status = CPublishStatus::new_ok();

        unsafe { cpublish_publish_rollback_post_publish(self, c_context, &mut status) };

        match status.status {
            crate::CPublishStatusType::CPublishStatusTypeOk => Ok(()),
            crate::CPublishStatusType::CPublishStatusTypeError => {
                let message = unsafe { CStr::from_ptr(status.message) };
                Err(publish::Error::new_publish(message.to_string_lossy(), None))
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_default_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) -> *mut CPublishContext {
    cpublish_status_ok(status);

    if publish.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("publish is null");
        }
        return null_mut();
    }

    if context.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("context is null");
        }
        return null_mut();
    }

    if !status.is_null() {
        *status = CPublishStatus::new_ok();
    }

    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_default_error_publish(
    #[allow(unused_variables)] publish: *const CPublishBasePublish,
    #[allow(unused_variables)] context: *const CPublishContext,
    status: *mut CPublishStatus,
) -> *mut CPublishContext {
    *status = CPublishStatus::new_error("CPublishBasePublish->publish_fn is not set");
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_default_rollback_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    if publish.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("publish is null");
        }
        return;
    }

    if context.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("context is null");
        }
        return;
    }

    if !status.is_null() {
        *status = CPublishStatus::new_ok();
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_new_default() -> CPublishBasePublish {
    CPublishBasePublish {
        pre_publish_fn: cpublish_publish_default_publish,
        rollback_pre_publish_fn: cpublish_publish_default_rollback_publish,
        publish_fn: cpublish_publish_default_error_publish,
        rollback_publish_fn: cpublish_publish_default_rollback_publish,
        post_publish_fn: cpublish_publish_default_publish,
        rollback_post_publish_fn: cpublish_publish_default_rollback_publish,
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_pre_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) -> *mut CPublishContext {
    cpublish_status_ok(status);

    match publish.as_ref() {
        Some(publish) => (publish.pre_publish_fn)(publish, context, status),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("publish is null");
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_rollback_pre_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    match publish.as_ref() {
        Some(publish) => (publish.rollback_pre_publish_fn)(publish, context, status),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("publish is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) -> *mut CPublishContext {
    cpublish_status_ok(status);

    match publish.as_ref() {
        Some(publish) => (publish.publish_fn)(publish, context, status),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("publish is null");
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_rollback_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    match publish.as_ref() {
        Some(publish) => (publish.rollback_publish_fn)(publish, context, status),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("publish is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_post_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) -> *mut CPublishContext {
    cpublish_status_ok(status);

    match publish.as_ref() {
        Some(publish) => (publish.post_publish_fn)(publish, context, status),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("publish is null");
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_publish_rollback_post_publish(
    publish: *const CPublishBasePublish,
    context: *const CPublishContext,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    match publish.as_ref() {
        Some(publish) => (publish.rollback_post_publish_fn)(publish, context, status),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("publish is null");
            }
        }
    }
}
