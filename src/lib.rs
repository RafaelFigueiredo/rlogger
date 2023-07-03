use std::collections::HashMap;

use pyo3::prelude::*;

mod internal;

#[pyfunction]
fn debug(message: String, context: Option<HashMap<&str,&str>>){
    internal::logger::debug(&message, context)
}

#[pyfunction]
fn info(message: String, context: Option<HashMap<&str,&str>>){
    internal::logger::info(&message, context)
}
#[pyfunction]
fn warn(message: String, context: Option<HashMap<&str,&str>>){
    internal::logger::warn(&message, context)
}
#[pyfunction]
fn error(message: String, context: Option<HashMap<&str,&str>>){
    internal::logger::error(&message, context)
}
#[pyfunction]
fn critical(message: String, context: Option<HashMap<&str,&str>>){
    internal::logger::critical(&message, context)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rlogger(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(warn, m)?)?;
    m.add_function(wrap_pyfunction!(error, m)?)?;
    m.add_function(wrap_pyfunction!(critical, m)?)?;
    Ok(())
}