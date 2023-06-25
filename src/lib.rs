use pyo3::prelude::*;

mod internal;

#[pyfunction]
fn debug(message: String){
    internal::logger::debug(&message)
}

#[pyfunction]
fn info(message: String){
    internal::logger::info(&message)
}
#[pyfunction]
fn warn(message: String){
    internal::logger::warn(&message)
}
#[pyfunction]
fn error(message: String){
    internal::logger::error(&message)
}
#[pyfunction]
fn critical(message: String){
    internal::logger::critical(&message)
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