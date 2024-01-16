use pyo3::{exceptions::PyTypeError, intern, prelude::*};

pub(crate) struct PublishWrapper {
    inner: PyObject,
}

impl PublishWrapper {
    pub(crate) fn new(inner: PyObject) -> Self {
        Self { inner }
    }
}

#[async_trait::async_trait]
impl publish::Publish for PublishWrapper {
    async fn pre_publish<'a>(
        &self,
        context: &'a publish::Context,
    ) -> Result<std::borrow::Cow<'a, publish::Context>, publish::Error> {
        let context_view = crate::Context::from(context.clone()).to_view();

        let result = Python::with_gil(|py| {
            self.inner
                .call_method1(py, intern!(py, "pre_publish"), (context_view.into_py(py),))
        })
        .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        let result = Python::with_gil(|py| pyo3_asyncio::tokio::into_future(result.as_ref(py)))
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?
            .await
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        Python::with_gil(|py| {
            let obj = result.to_object(py);
            let obj_ref = obj.as_ref(py);

            if obj_ref.is_instance_of::<crate::Context>() {
                let context = result.extract::<crate::Context>(py)?;
                Ok(std::borrow::Cow::Owned(context.inner))
            } else if obj_ref.is_instance_of::<crate::ContextView>() {
                Ok(std::borrow::Cow::Borrowed(context))
            } else {
                Err(PyTypeError::new_err(format!(
                    "Expected a Context or ContextView, got {}",
                    obj_ref.get_type().name()?
                )))
            }
        })
        .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))
    }

    async fn rollback_pre_publish(&self, context: &publish::Context) -> Result<(), publish::Error> {
        let context: crate::Context = context.clone().into();
        let context_view = context.to_view();

        let result = Python::with_gil(|py| {
            self.inner.call_method1(
                py,
                intern!(py, "rollback_pre_publish"),
                (context_view.into_py(py),),
            )
        })
        .map_err(|err| {
            publish::Error::new_rollback(
                "Error while rolling back pre_publish",
                Box::new(err),
                None,
            )
        })?;

        Python::with_gil(|py| pyo3_asyncio::tokio::into_future(result.as_ref(py)))
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?
            .await
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        Ok(())
    }

    async fn publish<'a>(
        &self,
        context: &'a publish::Context,
    ) -> Result<std::borrow::Cow<'a, publish::Context>, publish::Error> {
        let context_view = crate::Context::from(context.clone()).to_view();

        let result = Python::with_gil(|py| {
            self.inner
                .call_method1(py, intern!(py, "publish"), (context_view.into_py(py),))
        })
        .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        let result = Python::with_gil(|py| pyo3_asyncio::tokio::into_future(result.as_ref(py)))
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?
            .await
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        Python::with_gil(|py| {
            let obj = result.to_object(py);
            let obj_ref = obj.as_ref(py);

            if obj_ref.is_instance_of::<crate::Context>() {
                let context = result.extract::<crate::Context>(py)?;
                Ok(std::borrow::Cow::Owned(context.inner))
            } else if obj_ref.is_instance_of::<crate::ContextView>() {
                Ok(std::borrow::Cow::Borrowed(context))
            } else {
                Err(PyTypeError::new_err(format!(
                    "Expected a Context or ContextView, got {}",
                    obj_ref.get_type().name()?
                )))
            }
        })
        .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))
    }

    async fn rollback_publish(&self, context: &publish::Context) -> Result<(), publish::Error> {
        let context: crate::Context = context.clone().into();
        let context_view = context.to_view();

        let result = Python::with_gil(|py| {
            self.inner.call_method1(
                py,
                intern!(py, "rollback_publish"),
                (context_view.into_py(py),),
            )
        })
        .map_err(|err| {
            publish::Error::new_rollback("Error while rolling back publish", Box::new(err), None)
        })?;

        Python::with_gil(|py| pyo3_asyncio::tokio::into_future(result.as_ref(py)))
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?
            .await
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        Ok(())
    }

    async fn post_publish<'a>(
        &self,
        context: &'a publish::Context,
    ) -> Result<std::borrow::Cow<'a, publish::Context>, publish::Error> {
        let context_view = crate::Context::from(context.clone()).to_view();

        let result = Python::with_gil(|py| {
            self.inner
                .call_method1(py, intern!(py, "post_publish"), (context_view.into_py(py),))
        })
        .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        let result = Python::with_gil(|py| pyo3_asyncio::tokio::into_future(result.as_ref(py)))
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?
            .await
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        Python::with_gil(|py| {
            let obj = result.to_object(py);
            let obj_ref = obj.as_ref(py);

            if obj_ref.is_instance_of::<crate::Context>() {
                let context = result.extract::<crate::Context>(py)?;
                Ok(std::borrow::Cow::Owned(context.inner))
            } else if obj_ref.is_instance_of::<crate::ContextView>() {
                Ok(std::borrow::Cow::Borrowed(context))
            } else {
                Err(PyTypeError::new_err(format!(
                    "Expected a Context or ContextView, got {}",
                    obj_ref.get_type().name()?
                )))
            }
        })
        .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))
    }

    async fn rollback_post_publish(
        &self,
        context: &publish::Context,
    ) -> Result<(), publish::Error> {
        let context: crate::Context = context.clone().into();
        let context_view = context.to_view();

        let result = Python::with_gil(|py| {
            self.inner.call_method1(
                py,
                intern!(py, "rollback_post_publish"),
                (context_view.into_py(py),),
            )
        })
        .map_err(|err| {
            publish::Error::new_rollback(
                "Error while rolling back post_publish",
                Box::new(err),
                None,
            )
        })?;

        Python::with_gil(|py| pyo3_asyncio::tokio::into_future(result.as_ref(py)))
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?
            .await
            .map_err(|err| publish::Error::new_publish(err.to_string(), Some(Box::new(err))))?;

        Ok(())
    }
}
