use pyo3::{
    prelude::*,
    types::{PyBool, PyFloat, PyInt, PyString},
};

#[derive(Debug, Clone)]
pub(crate) struct Value {
    inner: publish::Value,
}

impl<'py> FromPyObject<'py> for Value {
    fn extract(obj: &'py PyAny) -> PyResult<Self> {
        if obj.is_none() {
            Ok(Self {
                inner: publish::Value::None,
            })
        } else if obj.is_instance_of::<PyBool>() {
            Ok(Self {
                inner: publish::Value::Boolean(obj.extract::<bool>()?),
            })
        } else if obj.is_instance_of::<PyInt>() {
            Ok(Self {
                inner: publish::Value::Integer(obj.extract::<i64>()?),
            })
        } else if obj.is_instance_of::<PyFloat>() {
            Ok(Self {
                inner: publish::Value::Float(obj.extract::<f64>()?),
            })
        } else if obj.is_instance_of::<PyString>() {
            Ok(Self {
                inner: publish::Value::String(obj.extract::<String>()?),
            })
        } else if let Ok(value) = obj.extract::<std::collections::HashMap<String, Value>>() {
            Ok(Self {
                inner: publish::Value::Object(
                    value.into_iter().map(|(k, v)| (k, v.inner)).collect(),
                ),
            })
        } else if let Ok(value) = obj.extract::<Vec<Value>>() {
            Ok(Self {
                inner: publish::Value::Array(value.into_iter().map(|v| v.inner).collect()),
            })
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                "Cannot convert {} to Value",
                obj.get_type().name()?
            )))
        }
    }
}

impl IntoPy<PyObject> for Value {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self.inner {
            publish::Value::None => py.None(),
            publish::Value::Boolean(value) => value.to_object(py),
            publish::Value::Integer(value) => value.to_object(py),
            publish::Value::Float(value) => value.to_object(py),
            publish::Value::String(value) => value.to_object(py),
            publish::Value::Array(value) => value
                .into_iter()
                .map(|v| Value { inner: v })
                .collect::<Vec<Value>>()
                .to_object(py),
            publish::Value::Object(value) => value
                .into_iter()
                .map(|(k, v)| (k, Value { inner: v }))
                .collect::<std::collections::HashMap<String, Value>>()
                .to_object(py),
        }
    }
}

impl ToPyObject for Value {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match &self.inner {
            publish::Value::None => py.None(),
            publish::Value::Boolean(value) => value.to_object(py),
            publish::Value::Integer(value) => value.to_object(py),
            publish::Value::Float(value) => value.to_object(py),
            publish::Value::String(value) => value.to_object(py),
            publish::Value::Array(value) => value
                .iter()
                .map(|v| Value { inner: v.clone() })
                .collect::<Vec<Value>>()
                .to_object(py),
            publish::Value::Object(value) => value
                .iter()
                .map(|(k, v)| (k, Value { inner: v.clone() }))
                .collect::<std::collections::HashMap<&String, Value>>()
                .to_object(py),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct Context {
    pub(crate) inner: publish::Context,
}

impl From<Context> for publish::Context {
    fn from(value: Context) -> Self {
        value.inner
    }
}

impl From<publish::Context> for Context {
    fn from(value: publish::Context) -> Self {
        Self { inner: value }
    }
}

#[pymethods]
impl Context {
    #[new]
    #[pyo3(signature = (value = None))]
    fn new(value: Option<std::collections::HashMap<String, Value>>) -> Self {
        let inner = match value {
            Some(value) => publish::Context::new(value.into_iter().map(|(k, v)| (k, v.inner))),
            None => publish::Context::default(),
        };
        Self { inner }
    }

    fn get(&self, key: &str) -> Option<Value> {
        self.inner.get(key).map(|value| Value {
            inner: value.clone(),
        })
    }

    fn set(&mut self, key: &str, value: Value) {
        self.inner.set(key, value.inner);
    }

    fn copy(&self) -> Context {
        self.clone()
    }

    pub(crate) fn to_view(&self) -> ContextView {
        ContextView {
            inner: self.clone(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct ContextView {
    pub(crate) inner: Context,
}

#[pymethods]
impl ContextView {
    fn get(&self, key: &str) -> Option<Value> {
        self.inner.get(key)
    }

    fn copy(&self) -> Context {
        self.inner.clone()
    }

    fn to_view(&self) -> ContextView {
        self.clone()
    }
}
