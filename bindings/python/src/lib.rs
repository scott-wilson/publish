use pyo3::prelude::*;

mod context;
mod publish;
mod publish_wrapper;
mod runner;

use context::{Context, ContextView};
use publish::Publish;
use runner::run;

#[pymodule]
fn pypublish(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;

    m.add_class::<Context>()?;
    m.add_class::<ContextView>()?;
    m.add_class::<Publish>()?;

    Ok(())
}
