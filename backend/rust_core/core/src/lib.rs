use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// This is the function that will be exposed to Python.
#[pyfunction]
fn some_function() -> String {
    "Hello from Rust!".to_string()
}

/// This is the module that will be exposed to Python.
#[pymodule]
fn rust_backend(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(some_function, py)?)?;
    Ok(())
}
