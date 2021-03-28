mod func;

use crate::func::{process, sum_as_string};

use cpython::{py_fn, py_module_initializer, PyObject, PyResult, Python};

py_module_initializer!(libtree, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "sum_as_string",
        py_fn!(py, py_sum_as_string(a: i64, b: i64)),
    )?;
    m.add(py, "process", py_fn!(py, py_process()))?;
    Ok(())
});

fn py_sum_as_string(_: Python, a: i64, b: i64) -> PyResult<String> {
    Ok(sum_as_string(a, b))
}

fn py_process(py: Python) -> PyResult<PyObject> {
    process();
    Ok(py.None())
}
