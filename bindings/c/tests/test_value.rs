use std::ffi::{CStr, CString};

use proptest::prelude::*;

use cpublish::*;

fn arb_value_array() -> impl Strategy<Value = publish::Value> {
    prop::collection::vec(arb_value(), 0..10)
        .prop_map(publish::Value::Array)
        .boxed()
}

fn arb_value_object() -> impl Strategy<Value = publish::Value> {
    prop::collection::hash_map("[^\0]*", arb_value(), 0..10)
        .prop_map(publish::Value::Object)
        .boxed()
}

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

#[test]
fn test_cpublish_value_new_none_success() {
    unsafe {
        let value = cpublish_value_new_none();
        let mut status = CPublishStatus::new_ok();

        assert!(!value.is_null());
        assert_eq!(
            cpublish_value_type(value, &mut status),
            CPublishValueType::CPublishValueTypeNone
        );
        assert_eq!(
            status.status,
            CPublishStatusType::CPublishStatusTypeOk,
            "Err: {}",
            CStr::from_ptr(status.message).to_string_lossy()
        );
    }
}

proptest! {
    #[test]
    fn test_cpublish_value_new_bool_success(v: bool) {
        unsafe {
            let value = cpublish_value_new_bool(v);

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeBoolean);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            assert_eq!(cpublish_value_bool(value, &mut status), v);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
        }
    }

    #[test]
    fn test_cpublish_value_new_int_success(v: i64) {
        unsafe {
            let value = cpublish_value_new_int(v);

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeInteger);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            assert_eq!(cpublish_value_int(value, &mut status), v);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
        }
    }

    #[test]
    fn test_cpublish_value_new_float_success(v: f64) {
        unsafe {
            let value = cpublish_value_new_float(v);

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeFloat);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            if v.is_nan() {
                assert!(cpublish_value_float(value, &mut status).is_nan());
            } else {
                assert_eq!(cpublish_value_float(value, &mut status), v);
            }

            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
        }
    }

    #[test]
    fn test_cpublish_value_new_string_success(v in "[^\0]*") {
        let c_v = CString::new(v).unwrap().into_raw();

        unsafe {
            let value = cpublish_value_new_string(c_v);

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeString);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            assert_eq!(CStr::from_ptr(cpublish_value_string(value, &mut status).string), CStr::from_ptr(c_v));
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            drop(CString::from_raw(c_v));
        }
    }

    #[test]
    fn test_cpublish_value_new_array_success(v in arb_value_array()) {
        let arr = match &v {
            publish::Value::Array(arr) => arr,
            _ => panic!(),
        };

        unsafe {
            let value = cpublish_value_new_array();

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeArray);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for item in arr {
                cpublish_value_array_push(value, &CPublishValue::from(item.clone()), &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            assert_eq!(cpublish_value_array_len(value, &mut status), arr.len());
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for index in 0..arr.len() {
                let item = cpublish_value_array_get(value, index, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!item.is_null());
                assert_eq!(&(*item).value, &arr[index]);
            }

            let iter = cpublish_value_array_iter(value, &mut status);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            let mut index = 0;

            while !cpublish_value_iter_array_is_done(iter, &mut status) {
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(index <= arr.len());

                let item = cpublish_value_iter_array_value(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!item.is_null());
                assert_eq!(&(*item).value, &arr[index]);
                index += 1;
                cpublish_value_iter_array_next(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            assert_eq!(index, arr.len());

            cpublish_value_iter_array_destroy(iter);
        }
    }

    #[test]
    fn test_cpublish_value_new_array_with_capacity_success(v in arb_value_array()) {
        let arr = match &v {
            publish::Value::Array(arr) => arr,
            _ => panic!(),
        };

        unsafe {
            let value = cpublish_value_new_array_with_capacity(arr.len());

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeArray);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for item in arr {
                cpublish_value_array_push(value, &CPublishValue::from(item.clone()), &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            assert_eq!(cpublish_value_array_len(value, &mut status), arr.len());
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for index in 0..arr.len() {
                let item = cpublish_value_array_get(value, index, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!item.is_null());
                assert_eq!(&(*item).value, &arr[index]);
            }

            let iter = cpublish_value_array_iter(value, &mut status);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            let mut index = 0;

            while !cpublish_value_iter_array_is_done(iter, &mut status) {
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(index <= arr.len());

                let item = cpublish_value_iter_array_value(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!item.is_null());
                assert_eq!(&(*item).value, &arr[index]);
                index += 1;

                cpublish_value_iter_array_next(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            assert_eq!(index, arr.len());

            cpublish_value_iter_array_destroy(iter);
        }
    }

    #[test]
    fn test_cpublish_value_new_object_success(v in arb_value_object()) {
        let obj = match &v {
            publish::Value::Object(obj) => obj,
            _ => panic!(),
        };

        unsafe {
            let value = cpublish_value_new_object();

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeObject);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for (key, item) in obj {
                let key = CString::new(key.clone()).unwrap().into_raw();
                cpublish_value_object_insert(value, key, &CPublishValue::from(item.clone()), &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                drop(CString::from_raw(key));
            }

            assert_eq!(cpublish_value_object_len(value, &mut status), obj.len());
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for (key, item) in obj {
                let c_key = CString::new(key.clone()).unwrap().into_raw();
                let value_item = cpublish_value_object_get(value, c_key, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!value_item.is_null());
                assert_eq!(&(*value_item).value, item);
                drop(CString::from_raw(c_key));
            }

            let iter = cpublish_value_object_iter(value, &mut status);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            let mut index = 0;

            while !cpublish_value_iter_object_is_done(iter, &mut status) {
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(index <= obj.len());


                let key = cpublish_value_iter_object_key(iter, &mut status);
                assert!(!key.string.is_null());
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

                let item = cpublish_value_iter_object_value(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!item.is_null());
                assert_eq!(&(*item).value, &obj[CStr::from_ptr(key.string).to_string_lossy().as_ref()]);
                index += 1;

                cpublish_value_iter_object_next(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            assert_eq!(index, obj.len());

            cpublish_value_iter_object_destroy(iter);
        }
    }

    #[test]
    fn test_cpublish_value_new_object_with_capacity_success(v in arb_value_object()) {
        let obj = match &v {
            publish::Value::Object(obj) => obj,
            _ => panic!(),
        };

        unsafe {
            let value = cpublish_value_new_object_with_capacity(obj.len());

            assert!(!value.is_null());
            let mut status = CPublishStatus::new_ok();
            assert_eq!(cpublish_value_type(value, &mut status), CPublishValueType::CPublishValueTypeObject);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for (key, item) in obj {
                let key = CString::new(key.clone()).unwrap().into_raw();
                cpublish_value_object_insert(value, key, &CPublishValue::from(item.clone()), &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                drop(CString::from_raw(key));
            }

            assert_eq!(cpublish_value_object_len(value, &mut status), obj.len());
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            for (key, item) in obj {
                let c_key = CString::new(key.clone()).unwrap().into_raw();
                let value_item = cpublish_value_object_get(value, c_key, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!value_item.is_null());
                assert_eq!(&(*value_item).value, item);
                drop(CString::from_raw(c_key));
            }

            let iter = cpublish_value_object_iter(value, &mut status);
            assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

            let mut index = 0;

            while !cpublish_value_iter_object_is_done(iter, &mut status) {
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(index <= obj.len());

                let key = cpublish_value_iter_object_key(iter, &mut status);
                assert!(!key.string.is_null());
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());

                let item = cpublish_value_iter_object_value(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
                assert!(!item.is_null());
                assert_eq!(&(*item).value, &obj[CStr::from_ptr(key.string).to_string_lossy().as_ref()]);
                index += 1;

                cpublish_value_iter_object_next(iter, &mut status);
                assert_eq!(status.status, CPublishStatusType::CPublishStatusTypeOk, "Err: {}", CStr::from_ptr(status.message).to_string_lossy());
            }

            assert_eq!(index, obj.len());

            cpublish_value_iter_object_destroy(iter);
        }
    }
}
