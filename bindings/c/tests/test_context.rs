use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use proptest::prelude::*;

use cpublish::*;

fn arb_value() -> impl Strategy<Value = publish::Value> {
    let leaf = prop_oneof![
        Just(publish::Value::None),
        any::<bool>().prop_map(publish::Value::Boolean),
        any::<i64>().prop_map(publish::Value::Integer),
        any::<f64>().prop_map(publish::Value::Float),
        "[^\0]*".prop_map(publish::Value::String),
    ];

    leaf.prop_recursive(8, 256, 10, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 0..10).prop_map(publish::Value::Array),
            prop::collection::hash_map("[^\0]*", inner, 0..10).prop_map(publish::Value::Object),
        ]
    })
}

unsafe fn to_c_value(value: &publish::Value, status: *mut CPublishStatus) -> *mut CPublishValue {
    cpublish_status_ok(status);

    match value {
        publish::Value::None => cpublish_value_new_none(),
        publish::Value::Boolean(v) => cpublish_value_new_bool(*v),
        publish::Value::Integer(v) => cpublish_value_new_int(*v),
        publish::Value::Float(v) => cpublish_value_new_float(*v),
        publish::Value::String(v) => {
            let v = CString::new::<&str>(v.as_ref()).unwrap();
            cpublish_value_new_string(v.as_ptr())
        }
        publish::Value::Array(v) => {
            let array = cpublish_value_new_array();
            for value in v {
                let value = to_c_value(value, status);

                if !status.is_null() && (*status).status != CPublishStatusType::CPublishStatusTypeOk
                {
                    return null_mut();
                }

                cpublish_value_array_push(array, value, status);

                if !status.is_null() && (*status).status != CPublishStatusType::CPublishStatusTypeOk
                {
                    return null_mut();
                }
            }
            array
        }
        publish::Value::Object(v) => {
            let object = cpublish_value_new_object();
            for (key, value) in v {
                let key = CString::new::<&str>(key.as_ref()).unwrap();
                let value = to_c_value(value, status);

                if !status.is_null() && (*status).status != CPublishStatusType::CPublishStatusTypeOk
                {
                    return null_mut();
                }

                cpublish_value_object_insert(object, key.as_ptr(), value, status);

                if !status.is_null() && (*status).status != CPublishStatusType::CPublishStatusTypeOk
                {
                    return null_mut();
                }
            }
            object
        }
    }
}

proptest! {
    #[test]
    fn test_cpublish_context_set_success(v in prop::collection::hash_map("[^\0]*", arb_value(), 0..10)) {
        unsafe {
            let context = cpublish_context_new();
            assert!(!context.is_null());
            let mut status = CPublishStatus::new_ok();

            for (key, value) in v.iter() {
                let key = CString::new(key.as_str()).unwrap();
                let in_value = to_c_value(value, &mut status);
                cpublish_context_set(context, key.as_ptr(), in_value, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                assert!(!out_value.is_null());
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

                assert_eq!(&(*out_value).value, value);
            }

            assert_eq!(cpublish_context_len(context, &mut status), v.len());
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            if v.len() == 0 {
                assert!(cpublish_context_is_empty(context, &mut status));
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            let iter = cpublish_context_iter(context, &mut status);
            assert!(!iter.is_null());
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            while !cpublish_context_iter_is_done(iter, &mut status) {
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                let key = cpublish_context_iter_key(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                let value = cpublish_context_iter_value(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                let out_value = cpublish_context_get(context, key.string, &mut status);
                assert!(!out_value.is_null());
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

                assert_eq!(&(*out_value).value, &(*value).value);

                cpublish_context_iter_next(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            cpublish_context_iter_destroy(iter);
        }
    }

    #[test]
    fn test_cpublish_context_set_helpers_success(v in prop::collection::hash_map("[^\0]*", arb_value(), 0..10)) {
        unsafe {
            let context = cpublish_context_new();
            assert!(!context.is_null());
            let mut status = CPublishStatus::new_ok();

            for (key, value) in v.iter() {
                let key = CString::new(key.as_str()).unwrap();


                match value {
                    publish::Value::None => {
                        cpublish_context_set_none(context, key.as_ptr(), &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                        assert!(!out_value.is_null());
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        assert_eq!(&(*out_value).value, value);
                    },
                    publish::Value::Boolean(v) => {
                        cpublish_context_set_bool(context, key.as_ptr(), *v, &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                        assert!(!out_value.is_null());
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        assert_eq!(&(*out_value).value, value);
                    },
                    publish::Value::Integer(v) => {
                        cpublish_context_set_int(context, key.as_ptr(), *v, &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                        assert!(!out_value.is_null());
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        assert_eq!(&(*out_value).value, value);
                    },
                    publish::Value::Float(v) => {
                        cpublish_context_set_float(context, key.as_ptr(), *v, &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                        assert!(!out_value.is_null());
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        assert_eq!(&(*out_value).value, value);
                    },
                    publish::Value::String(v) => {
                        let c_v = CString::new(v.as_str()).unwrap();
                        cpublish_context_set_string(context, key.as_ptr(), c_v.as_ptr(), &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                        assert!(!out_value.is_null());
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        assert_eq!(&(*out_value).value, value);
                    },
                    publish::Value::Array(_) => {
                        let c_v = to_c_value(value, &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        cpublish_context_set(context, key.as_ptr(), c_v, &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                        assert!(!out_value.is_null());
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        assert_eq!(&(*out_value).value, value);
                    },
                    publish::Value::Object(_) => {
                        let c_v = to_c_value(value, &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        cpublish_context_set(context, key.as_ptr(), c_v, &mut status);
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        let out_value = cpublish_context_get(context, key.as_ptr(), &mut status);
                        assert!(!out_value.is_null());
                        assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                        assert_eq!(&(*out_value).value, value);
                    },
                };
            }

            assert_eq!(cpublish_context_len(context, &mut status), v.len());
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            if v.len() == 0 {
                assert!(cpublish_context_is_empty(context, &mut status));
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }
        }
    }

    #[test]
    fn test_cpublish_context_clone_success(v in prop::collection::hash_map("[^\0]*", arb_value(), 0..10)) {
        unsafe {
            let context = cpublish_context_new();
            assert!(!context.is_null());
            let mut status = CPublishStatus::new_ok();

            for (key, value) in v.iter() {
                let key = CString::new(key.as_str()).unwrap();
                let in_value = to_c_value(value, &mut status);
                cpublish_context_set(context, key.as_ptr(), in_value, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            let cloned_context = cpublish_context_clone(context, &mut status);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            let context_len = cpublish_context_len(context, &mut status);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            let cloned_context_len = cpublish_context_len(cloned_context, &mut status);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            assert_eq!(context_len, cloned_context_len);

            let iter = cpublish_context_iter(context, &mut status);

            while !cpublish_context_iter_is_done(iter, &mut status) {
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                let key = cpublish_context_iter_key(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

                let value = cpublish_context_iter_value(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                let cloned_value = cpublish_context_get(cloned_context, key.string, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

                assert_eq!(&(*value).value, &(*cloned_value).value);

                cpublish_context_iter_next(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            cpublish_context_iter_destroy(iter);
        }
    }
}
