use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn get_magnitude(num: &isize) -> i32 {
    if *num < 0 {
        get_negative_magnitude(num)
    } else {
        get_positive_magnitude(num)
    }
}

fn get_positive_magnitude(num: &isize) -> i32 {
    let mut i = 1;
    let mut j = 0;
    while num / i != 0 {
        i *= 10;
        j += 1;
    }
    j
}

fn get_negative_magnitude(num: &isize) -> i32 {
    let mut i = *num;
    let mut j = 0;
    while i < 0 {
        i *= 10;
        j -= 1;
    }
    j
}

#[pyfunction]
fn ceil(num: isize, precision: i32) -> PyResult<isize> {
    let magnitude = get_magnitude(&num);
    // need to before decimals
    if magnitude > 0 && magnitude >= precision {

    } else { // need to trim decimals
        
    }
    Ok(magnitude as isize)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn rounders(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(ceil))?;

    Ok(())
}