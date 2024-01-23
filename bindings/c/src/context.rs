use crate::{cpublish_status_ok, CPublishStatus, CPublishStringView};
use std::{
    ffi::{c_char, CStr, CString},
    ptr::{null, null_mut},
};
pub struct CPublishContext {
    pub inner: publish::Context,
}

impl From<publish::Context> for CPublishContext {
    fn from(value: publish::Context) -> Self {
        Self { inner: value }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_new() -> *mut CPublishContext {
    let inner = publish::Context::default();
    Box::into_raw(Box::new(CPublishContext { inner }))
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_destroy(context: *mut CPublishContext) {
    if !context.is_null() {
        drop(Box::from_raw(context));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_get(
    context: *const CPublishContext,
    key: *const c_char,
    status: *mut crate::CPublishStatus,
) -> *const crate::CPublishValue {
    cpublish_status_ok(status);

    match context.as_ref() {
        Some(context) => match CStr::from_ptr(key).to_str() {
            Ok(key) => match context.inner.get(key) {
                Some(value) => value as *const publish::Value as *const crate::CPublishValue,
                None => null(),
            },
            Err(_) => {
                if !status.is_null() {
                    *status = CPublishStatus::new_error("key is not a valid c-string");
                }

                null()
            }
        },
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }

            null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_set(
    context: *mut CPublishContext,
    key: *const c_char,
    value: *const crate::CPublishValue,
    status: *mut crate::CPublishStatus,
) {
    cpublish_status_ok(status);

    match context.as_mut() {
        Some(context) => {
            let key = match key.as_ref() {
                Some(key) => match CStr::from_ptr(key).to_str() {
                    Ok(key) => key,
                    Err(_) => {
                        if !status.is_null() {
                            *status = CPublishStatus::new_error("key is not a valid c-string");
                        }

                        return;
                    }
                },
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("key is null");
                    }

                    return;
                }
            };

            let value = match value.as_ref() {
                Some(value) => &value.value,
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("value is null");
                    }

                    return;
                }
            };

            context.inner.set(key, value.clone());
        }
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_set_none(
    context: *mut CPublishContext,
    key: *const c_char,
    status: *mut crate::CPublishStatus,
) {
    cpublish_status_ok(status);

    match context.as_mut() {
        Some(context) => {
            let key = match key.as_ref() {
                Some(key) => match CStr::from_ptr(key).to_str() {
                    Ok(key) => key,
                    Err(_) => {
                        if !status.is_null() {
                            *status = CPublishStatus::new_error("key is not a valid c-string");
                        }

                        return;
                    }
                },
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("key is null");
                    }

                    return;
                }
            };

            context.inner.set(key, publish::Value::None);
        }
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_set_bool(
    context: *mut CPublishContext,
    key: *const c_char,
    value: bool,
    status: *mut crate::CPublishStatus,
) {
    cpublish_status_ok(status);

    match context.as_mut() {
        Some(context) => {
            let key = match key.as_ref() {
                Some(key) => match CStr::from_ptr(key).to_str() {
                    Ok(key) => key,
                    Err(_) => {
                        if !status.is_null() {
                            *status = CPublishStatus::new_error("key is not a valid c-string");
                        }

                        return;
                    }
                },
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("key is null");
                    }

                    return;
                }
            };

            context.inner.set(key, publish::Value::Boolean(value));
        }
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_set_int(
    context: *mut CPublishContext,
    key: *const c_char,
    value: i64,
    status: *mut crate::CPublishStatus,
) {
    cpublish_status_ok(status);

    match context.as_mut() {
        Some(context) => {
            let key = match key.as_ref() {
                Some(key) => match CStr::from_ptr(key).to_str() {
                    Ok(key) => key,
                    Err(_) => {
                        if !status.is_null() {
                            *status = CPublishStatus::new_error("key is not a valid c-string");
                        }

                        return;
                    }
                },
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("key is null");
                    }

                    return;
                }
            };

            context.inner.set(key, publish::Value::Integer(value));
        }
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_set_float(
    context: *mut CPublishContext,
    key: *const c_char,
    value: f64,
    status: *mut crate::CPublishStatus,
) {
    cpublish_status_ok(status);

    match context.as_mut() {
        Some(context) => {
            let key = match key.as_ref() {
                Some(key) => match CStr::from_ptr(key).to_str() {
                    Ok(key) => key,
                    Err(_) => {
                        if !status.is_null() {
                            *status = CPublishStatus::new_error("key is not a valid c-string");
                        }

                        return;
                    }
                },
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("key is null");
                    }

                    return;
                }
            };

            context.inner.set(key, publish::Value::Float(value));
        }
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_set_string(
    context: *mut CPublishContext,
    key: *const c_char,
    value: *const c_char,
    status: *mut crate::CPublishStatus,
) {
    cpublish_status_ok(status);

    match context.as_mut() {
        Some(context) => {
            let key = match key.as_ref() {
                Some(key) => match CStr::from_ptr(key).to_str() {
                    Ok(key) => key,
                    Err(_) => {
                        if !status.is_null() {
                            *status = CPublishStatus::new_error("key is not a valid c-string");
                        }

                        return;
                    }
                },
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("key is null");
                    }

                    return;
                }
            };
            let value = match value.as_ref() {
                Some(value) => match CStr::from_ptr(value).to_str() {
                    Ok(value) => value,
                    Err(_) => {
                        if !status.is_null() {
                            *status = CPublishStatus::new_error("value is not a valid c-string");
                        }

                        return;
                    }
                },
                None => {
                    if !status.is_null() {
                        *status = CPublishStatus::new_error("value is null");
                    }

                    return;
                }
            };

            context
                .inner
                .set(key, publish::Value::String(value.to_string()));
        }
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_len(
    context: *const CPublishContext,
    status: *mut crate::CPublishStatus,
) -> usize {
    cpublish_status_ok(status);

    match context.as_ref() {
        Some(context) => context.inner.len(),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }

            0
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_is_empty(
    context: *const CPublishContext,
    status: *mut crate::CPublishStatus,
) -> bool {
    cpublish_status_ok(status);

    match context.as_ref() {
        Some(context) => context.inner.is_empty(),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }

            true
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_clone(
    context: *const CPublishContext,
    status: *mut crate::CPublishStatus,
) -> *mut CPublishContext {
    cpublish_status_ok(status);

    match context.as_ref() {
        Some(context) => Box::into_raw(Box::new(CPublishContext {
            inner: context.inner.clone(),
        })),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }

            null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_iter(
    context: *const CPublishContext,
    status: *mut crate::CPublishStatus,
) -> *mut CPublishContextIter {
    cpublish_status_ok(status);

    let context = match context.as_ref() {
        Some(context) => &context.inner,
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("context is null");
            }
            return null_mut();
        }
    };

    let iter = Box::new(CPublishContextIter {
        iter: context.iter(),
        key: null_mut(),
        value: null_mut(),
    });
    Box::into_raw(iter)
}

pub struct CPublishContextIter {
    iter: publish::ContextIter<'static>,
    key: *const c_char,
    value: *const crate::CPublishValue,
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_iter_destroy(iter: *mut CPublishContextIter) {
    if !iter.is_null() {
        drop(Box::from_raw(iter));
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_iter_next(
    iter: *mut CPublishContextIter,
    status: *mut CPublishStatus,
) {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => {
            if !iter.key.is_null() {
                drop(CString::from_raw(iter.key as *mut _));
                iter.key = null();
            }

            match iter.iter.next() {
                Some((key, value)) => {
                    iter.key = match CString::new::<&str>(key.as_ref()) {
                        Ok(key) => key.into_raw(),
                        Err(_) => {
                            if !status.is_null() {
                                *status = CPublishStatus::new_error("key is not a valid c-string");
                            }
                            return;
                        }
                    };
                    iter.value = value as *const publish::Value as *const crate::CPublishValue;
                }
                None => iter.value = null(),
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
pub unsafe extern "C" fn cpublish_context_iter_is_done(
    iter: *mut CPublishContextIter,
    status: *mut CPublishStatus,
) -> bool {
    cpublish_status_ok(status);

    match iter.as_mut() {
        Some(iter) => iter.value.is_null(),
        None => {
            if !status.is_null() {
                *status = CPublishStatus::new_error("iter is null");
            }

            true
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cpublish_context_iter_key(
    iter: *mut CPublishContextIter,
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
pub unsafe extern "C" fn cpublish_context_iter_value(
    iter: *mut CPublishContextIter,
    status: *mut CPublishStatus,
) -> *const crate::CPublishValue {
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
