#[macro_use] extern crate cpython;

use cpython::{PyResult, Python};

py_module_initializer!(libfactorial_rs, initlibfactorial_rs, PyInit_libfactorial_rs, |py, module| {
    module.add(py, "__doc__", "Tail-recursive factorial written in Rust.")?;
    module.add(py, "factorial", py_fn!(py, factorial_py(n: u64)))?;
    Ok(())
});

fn factorial(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        factorial(n - 1, n * acc)
    }
}

fn factorial_py(_: Python, n: u64) -> PyResult<u64> {
    Ok(factorial(n, 1))
}
