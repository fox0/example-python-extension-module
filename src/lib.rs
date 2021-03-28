mod func;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(func::sum_as_string(a, b))
}

#[pyfunction]
fn process() {
    func::process()
}

#[pymodule]
fn libtree(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__doc__", "This module is implemented in Rust.")?;
    // let ffi_wrapper_fun = raw_pycfunction!(some_fun);
    // let docs = "Some documentation string with null-termination\0";
    // let py_cfunction =
    //     PyCFunction::new_with_keywords(ffi_wrapper_fun, "function_name", docs, module.into())?;
    // module.add_function(py_cfunction)
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
