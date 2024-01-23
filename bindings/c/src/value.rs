use crate::{cpublish_status_ok, CPublishStatus, CPublishString, CPublishStringView};
use std::{
    ffi::{c_char, CStr, CString},
    ptr::{null, null_mut},
};

#[derive(Clone, Debug)]
pub struct CPublishValue {
    pub value: publish::Value,
}

impl From<publish::Value> for CPublishValue {
    fn from(value: publish::Value) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum CPublishValueType {
    CPublishValueTypeNone,
    CPublishValueTypeBoolean,
    CPublishValueTypeInteger,
    CPublishValueTypeFloat,
    CPublishValueTypeString,
    CPublishValueTypeArray,
    CPublishValueTypeObject,
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_none() -> *mut CPublishValue {
    let value = publish::Value::None;
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_bool(value: bool) -> *mut CPublishValue {
    let value = publish::Value::Boolean(value);
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_int(value: i64) -> *mut CPublishValue {
    let value = publish::Value::Integer(value);
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_float(value: f64) -> *mut CPublishValue {
    let value = publish::Value::Float(value);
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_string(value: *const c_char) -> *mut CPublishValue {
    let value = publish::Value::String(CStr::from_ptr(value).to_string_lossy().into_owned());
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_array() -> *mut CPublishValue {
    let value = publish::Value::Array(Vec::new());
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_array_with_capacity(
    capacity: usize,
) -> *mut CPublishValue {
    let value = publish::Value::Array(Vec::with_capacity(capacity));
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_object() -> *mut CPublishValue {
    let value = publish::Value::Object(std::collections::HashMap::new());
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_new_object_with_capacity(
    capacity: usize,
) -> *mut CPublishValue {
    let value = publish::Value::Object(std::collections::HashMap::with_capacity(capacity));
    Box::into_raw(Box::new(CPublishValue { value }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_destroy(value: *mut CPublishValue) {
    if !value.is_null() {
        drop(Box::from_raw(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_type(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> CPublishValueType {
    cpublish_status_ok(status);

    if value.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("value is null");
        }
        return CPublishValueType::CPublishValueTypeNone;
    }

    match (*value).value {
        publish::Value::None => CPublishValueType::CPublishValueTypeNone,
        publish::Value::Boolean(_) => CPublishValueType::CPublishValueTypeBoolean,
        publish::Value::Integer(_) => CPublishValueType::CPublishValueTypeInteger,
        publish::Value::Float(_) => CPublishValueType::CPublishValueTypeFloat,
        publish::Value::String(_) => CPublishValueType::CPublishValueTypeString,
        publish::Value::Array(_) => CPublishValueType::CPublishValueTypeArray,
        publish::Value::Object(_) => CPublishValueType::CPublishValueTypeObject,
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_bool(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> bool {
    cpublish_status_ok(status);

    match value.as_ref() {
        Some(value) => match value.value {
            publish::Value::Boolean(value) => value,
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not a boolean");
                }
                false
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_int(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> i64 {
    cpublish_status_ok(status);

    match value.as_ref() {
        Some(value) => match value.value {
            publish::Value::Integer(value) => value,
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not an integer");
                }
                0
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_float(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> f64 {
    cpublish_status_ok(status);

    match value.as_ref() {
        Some(value) => match value.value {
            publish::Value::Float(value) => value,
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not a float");
                }
                0.0
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            0.0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_string(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> CPublishString {
    cpublish_status_ok(status);

    match value.as_ref() {
        Some(value) => match &value.value {
            publish::Value::String(value) => CPublishString::new(value),
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not a string");
                }
                CPublishString::new("")
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            CPublishString::new("")
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_array_len(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> usize {
    cpublish_status_ok(status);

    match value.as_ref() {
        Some(value) => match &value.value {
            publish::Value::Array(value) => value.len(),
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not an array");
                }
                0
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_array_get(
    value: *const CPublishValue,
    index: usize,
    status: *mut CPublishStatus,
) -> *const CPublishValue {
    cpublish_status_ok(status);

    match value.as_ref() {
        Some(value) => match &value.value {
            publish::Value::Array(value) => match value.get(index) {
                Some(value) => value as *const publish::Value as *const CPublishValue,
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("index out of bounds");
                    }
                    null()
                }
            },
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not an array");
                }
                null()
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_array_push(
    value: *mut CPublishValue,
    item: *const CPublishValue,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    if item.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("item is null");
        }
        return;
    }

    match value.as_mut() {
        Some(value) => match &mut value.value {
            publish::Value::Array(value) => value.push((*item).value.clone()),
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not an array");
                }
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_array_iter(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> *mut CPublishValueIterArray {
    cpublish_status_ok(status);

    let value = match value.as_ref() {
        Some(value) => value,
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }

            return null_mut();
        }
    };

    let iter = match &value.value {
        publish::Value::Array(value) => {
            let iter = Box::new(CPublishValueIterArray {
                iter: if value.len() > 0 {
                    Some(value.iter())
                } else {
                    None
                },
                value: null_mut(),
            });
            Box::into_raw(iter)
        }
        _ => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is not an array");
            }
            null_mut()
        }
    };

    cpublish_value_iter_array_next(iter, status);

    match status.as_ref() {
        Some(_) => iter,
        None => {
            cpublish_value_iter_array_destroy(iter);
            null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_object_len(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> usize {
    cpublish_status_ok(status);

    match value.as_ref() {
        Some(value) => match &value.value {
            publish::Value::Object(value) => value.len(),
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not an object");
                }
                0
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_object_get(
    value: *const CPublishValue,
    key: *const c_char,
    status: *mut CPublishStatus,
) -> *const CPublishValue {
    cpublish_status_ok(status);

    if key.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("key is null");
        }
        return null();
    }

    let key = CStr::from_ptr(key).to_string_lossy();

    match value.as_ref() {
        Some(value) => match &value.value {
            publish::Value::Object(value) => match value.get(key.as_ref()) {
                Some(value) => value as *const publish::Value as *const CPublishValue,
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("value is not in object");
                    }
                    null()
                }
            },
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not in object");
                }
                null()
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
            null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_object_insert(
    value: *mut CPublishValue,
    key: *const c_char,
    item: *const CPublishValue,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    if key.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("key is null");
        }
        return;
    }

    if item.is_null() {
        if !status.is_null() {
            *status = CPublishStatus::new_error("item is null");
        }
        return;
    }

    match value.as_mut() {
        Some(value) => match &mut value.value {
            publish::Value::Object(value) => {
                let key = CStr::from_ptr(key).to_string_lossy().to_string();
                value.insert(key, (*item).value.clone());
            }
            _ => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("value is not an array");
                }
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_object_iter(
    value: *const CPublishValue,
    status: *mut CPublishStatus,
) -> *mut CPublishValueIterObject {
    cpublish_status_ok(status);

    let value = match value.as_ref() {
        Some(value) => value,
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is null");
            }

            return null_mut();
        }
    };

    let iter = match &value.value {
        publish::Value::Object(value) => {
            let iter = Box::new(CPublishValueIterObject {
                iter: if value.len() > 0 {
                    Some(value.iter())
                } else {
                    None
                },
                key: null_mut(),
                value: null_mut(),
            });
            Box::into_raw(iter)
        }
        _ => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("value is not an object");
            }
            null_mut()
        }
    };

    cpublish_value_iter_object_next(iter, status);

    match status.as_ref() {
        Some(_) => iter,
        None => {
            cpublish_value_iter_object_destroy(iter);
            null_mut()
        }
    }
}

pub struct CPublishValueIterArray {
    iter: Option<std::slice::Iter<'static, publish::Value>>,
    value: *const CPublishValue,
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_array_destroy(iter: *mut CPublishValueIterArray) {
    if !iter.is_null() {
        drop(Box::from_raw(iter));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_array_next(
    iter: *mut CPublishValueIterArray,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => match &mut iter.iter {
            Some(iter_inner) => match iter_inner.next() {
                Some(value) => iter.value = value as *const publish::Value as *const CPublishValue,
                None => {
                    iter.value = null();
                    iter.iter = None;
                }
            },
            None => {
                iter.value = null();
                iter.iter = None;
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_array_is_done(
    iter: *mut CPublishValueIterArray,
    status: *mut CPublishStatus,
) -> bool {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => iter.value.is_null() && iter.iter.is_none(),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }

            true
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_array_value(
    iter: *mut CPublishValueIterArray,
    status: *mut CPublishStatus,
) -> *const CPublishValue {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => iter.value,
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }

            null()
        }
    }
}

pub struct CPublishValueIterObject {
    iter: Option<std::collections::hash_map::Iter<'static, String, publish::Value>>,
    key: *const c_char,
    value: *const CPublishValue,
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_object_destroy(iter: *mut CPublishValueIterObject) {
    if !iter.is_null() {
        drop(Box::from_raw(iter));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_object_next(
    iter: *mut CPublishValueIterObject,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => {
            if !iter.key.is_null() {
                drop(CString::from_raw(iter.key as *mut _));
                iter.key = null();
            }

            match &mut iter.iter {
                Some(iter_inner) => match iter_inner.next() {
                    Some((key, value)) => {
                        iter.key = match CString::new::<&str>(key.as_ref()) {
                            Ok(key) => key.into_raw(),
                            Err(_) => {
                                if !status.is_null() {
                                    *status =
                                        CPublishStatus::new_error("key is not a valid c-string");
                                }
                                return;
                            }
                        };
                        iter.value = value as *const publish::Value as *const CPublishValue
                    }
                    None => {
                        iter.value = null();
                        iter.iter = None;
                    }
                },
                None => {
                    iter.value = null();
                    iter.iter = None;
                }
            }
        }
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_object_is_done(
    iter: *mut CPublishValueIterObject,
    status: *mut CPublishStatus,
) -> bool {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => iter.value.is_null() && iter.iter.is_none(),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }

            true
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_object_key(
    iter: *mut CPublishValueIterObject,
    status: *mut CPublishStatus,
) -> CPublishStringView {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => CPublishStringView { string: iter.key },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }

            CPublishStringView { string: null() }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_value_iter_object_value(
    iter: *mut CPublishValueIterObject,
    status: *mut CPublishStatus,
) -> *const CPublishValue {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => iter.value,
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }

            null()
        }
    }
}
