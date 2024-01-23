use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use cpublish::*;

#[test]
fn test_cpublish_publish_new_default_success() {
    unsafe {
        let publish = cpublish_publish_new_default();

        assert_eq!(
            publish.pre_publish_fn as usize,
            cpublish_publish_default_publish as usize
        );
        assert_eq!(
            publish.rollback_pre_publish_fn as usize,
            cpublish_publish_default_rollback_publish as usize
        );
        assert_eq!(
            publish.publish_fn as usize,
            cpublish_publish_default_error_publish as usize
        );
        assert_eq!(
            publish.rollback_publish_fn as usize,
            cpublish_publish_default_rollback_publish as usize
        );
        assert_eq!(
            publish.post_publish_fn as usize,
            cpublish_publish_default_publish as usize
        );
        assert_eq!(
            publish.rollback_post_publish_fn as usize,
            cpublish_publish_default_rollback_publish as usize
        );
    }
}

#[test]
fn test_cpublish_publish_pre_publish_success() {
    unsafe {
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

            let key = CString::new("key").unwrap();

            cpublish_context_set_bool(ctx, key.as_ptr(), true, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            ctx
        }

        pub unsafe extern "C" fn publish_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());

            null_mut()
        }

        pub unsafe extern "C" fn rollback_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());
        }

        let mut publish = cpublish_publish_new_default();
        publish.pre_publish_fn = publish_should_pass;
        publish.rollback_pre_publish_fn = rollback_should_fail;
        publish.publish_fn = publish_should_fail;
        publish.rollback_publish_fn = rollback_should_fail;
        publish.post_publish_fn = publish_should_fail;
        publish.rollback_post_publish_fn = rollback_should_fail;
        let context = cpublish_context_new();
        let mut status = CPublishStatus::new_ok();
        let ctx = cpublish_publish_pre_publish(&publish, context, &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert!(!ctx.is_null());
        let key = CString::new("key").unwrap();
        let result = cpublish_context_get(ctx, key.as_ptr(), &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert!(!result.is_null());

        match (*result).value {
            publish::Value::Boolean(v) => assert!(v),
            _ => panic!(),
        };
    }
}

#[test]
fn test_cpublish_publish_rollback_pre_publish_success() {
    unsafe {
        pub unsafe extern "C" fn publish_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());

            null_mut()
        }

        pub unsafe extern "C" fn rollback_should_pass(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            cpublish_status_ok(status);
        }

        pub unsafe extern "C" fn rollback_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());
        }

        let mut publish = cpublish_publish_new_default();
        publish.pre_publish_fn = publish_should_fail;
        publish.rollback_pre_publish_fn = rollback_should_pass;
        publish.publish_fn = publish_should_fail;
        publish.rollback_publish_fn = rollback_should_fail;
        publish.post_publish_fn = publish_should_fail;
        publish.rollback_post_publish_fn = rollback_should_fail;
        let context = cpublish_context_new();
        let mut status = CPublishStatus::new_ok();
        cpublish_publish_rollback_pre_publish(&publish, context, &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
    }
}

#[test]
fn test_cpublish_publish_publish_success() {
    unsafe {
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

            let key = CString::new("key").unwrap();

            cpublish_context_set_bool(ctx, key.as_ptr(), true, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            ctx
        }

        pub unsafe extern "C" fn publish_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());

            null_mut()
        }

        pub unsafe extern "C" fn rollback_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());
        }

        let mut publish = cpublish_publish_new_default();
        publish.pre_publish_fn = publish_should_fail;
        publish.rollback_pre_publish_fn = rollback_should_fail;
        publish.publish_fn = publish_should_pass;
        publish.rollback_publish_fn = rollback_should_fail;
        publish.post_publish_fn = publish_should_fail;
        publish.rollback_post_publish_fn = rollback_should_fail;
        let context = cpublish_context_new();
        let mut status = CPublishStatus::new_ok();
        let ctx = cpublish_publish_publish(&publish, context, &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert!(!ctx.is_null());
        let key = CString::new("key").unwrap();
        let result = cpublish_context_get(ctx, key.as_ptr(), &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert!(!result.is_null());

        match (*result).value {
            publish::Value::Boolean(v) => assert!(v),
            _ => panic!(),
        };
    }
}

#[test]
fn test_cpublish_publish_rollback_publish_success() {
    unsafe {
        pub unsafe extern "C" fn publish_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());

            null_mut()
        }

        pub unsafe extern "C" fn rollback_should_pass(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            cpublish_status_ok(status);
        }

        pub unsafe extern "C" fn rollback_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());
        }

        let mut publish = cpublish_publish_new_default();
        publish.pre_publish_fn = publish_should_fail;
        publish.rollback_pre_publish_fn = rollback_should_fail;
        publish.publish_fn = publish_should_fail;
        publish.rollback_publish_fn = rollback_should_pass;
        publish.post_publish_fn = publish_should_fail;
        publish.rollback_post_publish_fn = rollback_should_fail;
        let context = cpublish_context_new();
        let mut status = CPublishStatus::new_ok();
        cpublish_publish_rollback_publish(&publish, context, &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
    }
}

#[test]
fn test_cpublish_publish_post_publish_success() {
    unsafe {
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

            let key = CString::new("key").unwrap();

            cpublish_context_set_bool(ctx, key.as_ptr(), true, status);

            if !status.is_null() && (*status).status == CPublishStatusType::CPublishStatusTypeError
            {
                return null_mut();
            }

            ctx
        }

        pub unsafe extern "C" fn publish_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());

            null_mut()
        }

        pub unsafe extern "C" fn rollback_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());
        }

        let mut publish = cpublish_publish_new_default();
        publish.pre_publish_fn = publish_should_fail;
        publish.rollback_pre_publish_fn = rollback_should_fail;
        publish.publish_fn = publish_should_fail;
        publish.rollback_publish_fn = rollback_should_fail;
        publish.post_publish_fn = publish_should_pass;
        publish.rollback_post_publish_fn = rollback_should_fail;
        let context = cpublish_context_new();
        let mut status = CPublishStatus::new_ok();
        let ctx = cpublish_publish_post_publish(&publish, context, &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert!(!ctx.is_null());
        let key = CString::new("key").unwrap();
        let result = cpublish_context_get(ctx, key.as_ptr(), &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
        assert!(!result.is_null());

        match (*result).value {
            publish::Value::Boolean(v) => assert!(v),
            _ => panic!(),
        };
    }
}

#[test]
fn test_cpublish_publish_rollback_post_publish_success() {
    unsafe {
        pub unsafe extern "C" fn publish_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) -> *mut CPublishContext {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());

            null_mut()
        }

        pub unsafe extern "C" fn rollback_should_pass(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            cpublish_status_ok(status);
        }

        pub unsafe extern "C" fn rollback_should_fail(
            _publish: *const CPublishBasePublish,
            _context: *const CPublishContext,
            status: *mut CPublishStatus,
        ) {
            let message = CString::new("Should not be called").unwrap();
            cpublish_status_error(status, message.as_ptr());
        }

        let mut publish = cpublish_publish_new_default();
        publish.pre_publish_fn = publish_should_fail;
        publish.rollback_pre_publish_fn = rollback_should_fail;
        publish.publish_fn = publish_should_fail;
        publish.rollback_publish_fn = rollback_should_fail;
        publish.post_publish_fn = publish_should_fail;
        publish.rollback_post_publish_fn = rollback_should_pass;
        let context = cpublish_context_new();
        let mut status = CPublishStatus::new_ok();
        cpublish_publish_rollback_post_publish(&publish, context, &mut status);
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
    }
}
