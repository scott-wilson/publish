use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use cpublish::*;

#[test]
fn test_run_success() {
    unsafe {
        pub unsafe extern "C" fn pre_publish_should_pass(
            publish: *const CPublishBasePublish,
            context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            cpublish_status_ok(status);
            assert!(!publish.is_null());
            assert!(!context.is_null());
            assert!(!status.is_null());

            let ctx = cpublish_context_clone(context, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            let key = CString::new("pre_publish").unwrap();

            cpublish_context_set_bool(ctx, key.as_ptr(), true, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            ctx
        }
        pub unsafe extern "C" fn publish_should_pass(
            publish: *const CPublishBasePublish,
            context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            cpublish_status_ok(status);
            assert!(!publish.is_null());
            assert!(!context.is_null());
            assert!(!status.is_null());

            let ctx = cpublish_context_clone(context, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            let key = CString::new("publish").unwrap();

            cpublish_context_set_bool(ctx, key.as_ptr(), true, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            ctx
        }
        pub unsafe extern "C" fn post_publish_should_pass(
            publish: *const CPublishBasePublish,
            context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            cpublish_status_ok(status);
            assert!(!publish.is_null());
            assert!(!context.is_null());
            assert!(!status.is_null());

            let ctx = cpublish_context_clone(context, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            let key = CString::new("post_publish").unwrap();

            cpublish_context_set_bool(ctx, key.as_ptr(), true, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            ctx
        }

        pub unsafe extern "C" fn rollback_should_pass(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            cpublish_status_ok(status);
        }

        let mut publish = cpublish_publish_new_default();
        publish.pre_publish_fn = pre_publish_should_pass;
        publish.rollback_pre_publish_fn = rollback_should_pass;
        publish.publish_fn = publish_should_pass;
        publish.rollback_publish_fn = rollback_should_pass;
        publish.post_publish_fn = post_publish_should_pass;
        publish.rollback_post_publish_fn = rollback_should_pass;

        let mut status = CPublishStatus::new_ok();
        let context = cpublish_run(&publish, &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );

        let key = CString::new("pre_publish").unwrap();
        let pre_publish_result = cpublish_context_get(context, key.as_ptr(), &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert_eq!(cpublish_value_bool(pre_publish_result, &mut status), true);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );

        let key = CString::new("publish").unwrap();
        let publish_result = cpublish_context_get(context, key.as_ptr(), &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert_eq!(cpublish_value_bool(publish_result, &mut status), true);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );

        let key = CString::new("post_publish").unwrap();
        let post_publish_result = cpublish_context_get(context, key.as_ptr(), &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert_eq!(cpublish_value_bool(post_publish_result, &mut status), true);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
    }
}
