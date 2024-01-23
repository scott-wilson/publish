use crate::{cpublish_status_ok, CPublishBasePublish, CPublishContext, CPublishStatus};
use std::ptr::{null, null_mut};

#[no_mangle]
pub unsafe extern "C" fn cpublish_run(
    publish: *const CPublishBasePublish,
    status: *mut CPublishStatus,
) -> *mut CPublishContext {
    cpublish_status_ok(status);

    let publish = match publish.as_ref() {
        Some(publish) => publish,
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("publish is null");
            }
            null()
        }
    };

    let rt = match tokio::runtime::Builder::new_current_thread().build() {
        Ok(rt) => rt,
        Err(err) => {
            if !status.is_null() {
                *status = CPublishStatus::new_error(format!("failed to build runtime: {}", err));
            }
            return null_mut();
        }
    };

    let result = rt.block_on(async { ::publish::run(unsafe { &*publish }).await });

    match result {
        Ok(context) => {
            let c_context = CPublishContext::from(context);
            Box::into_raw(Box::new(c_context))
        }
        Err(err) => {
            if !status.is_null() {
                *status = CPublishStatus::new_error(err.to_string());
            }
            null_mut()
        }
    }
}
