use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod rounders;

#[pyfunction]
fn get_magnitude(num: f64) -> PyResult<i32> {
    Ok(rounders::magnitude::get_magnitude(&num))
}

#[pyfunction]
fn floor(num: f64, precision: Option<i32>) -> PyResult<f64> {
    Ok(rounders::floor(num, precision))
}

#[pyfunction]
fn ceil(num: f64, precision: Option<i32>) -> PyResult<f64> {
    Ok(rounders::ceil(num, precision))
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rounders(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_magnitude))?;
    m.add_wrapped(wrap_pyfunction!(floor))?;
    m.add_wrapped(wrap_pyfunction!(ceil))?;

    Ok(())
}