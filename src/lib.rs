#[macro_use] extern crate cpython;
//use cpython::{PyResult, Python, PyLong, ToPyObject};
use cpython::*;

extern crate num_bigint;
use num_bigint::BigUint;

// TODO: Fix this to use `concat_idents!` once it comes stable.
py_module_initializer!(factorial, initfactorial, PyInit_factorial, |py, module| {
    module.add(py, "__doc__", "Tail-recursive factorial written in Rust.")?;
    module.add(py, "factorial", py_fn!(py, factorial_py(n: PyLong)))?;
    Ok(())
});

fn factorial(n: BigUint, acc: BigUint) -> BigUint {
    if n == BigUint::from(0u64) {
        acc
    } else {
        factorial(n.clone() - BigUint::from(1u64), n * acc)
    }
}

fn pylong_to_biguint(py: Python, n: PyLong) -> PyResult<BigUint> {
    let locals = PyDict::new(py);
    locals.set_item(py, "n", n)?;

    // XXX: Straight-up using `eval` feels like cheating. I'll have to look deeper into the source
    // code or documentation.
    let pystr_n = py.eval("str(n)", None, Some(&locals))?.cast_into::<PyString>(py).unwrap();

    Ok(pystr_n.to_string(py)?.parse::<BigUint>().unwrap())
}

fn biguint_to_pylong(py: Python, n: BigUint) -> PyResult<PyLong> {
    let locals = PyDict::new(py);
    locals.set_item(py, "str_n", n.to_string().to_py_object(py))?;

    // TODO: Check for Py 2/3
    Ok(py.eval("int(str_n)", None, Some(&locals))?.cast_into::<PyLong>(py).unwrap())
}

fn factorial_py(py: Python, n: PyLong) -> PyResult<PyLong> {
    let big_n = pylong_to_biguint(py, n)?;
    let result = factorial(big_n, BigUint::from(1u64));

    biguint_to_pylong(py, result)
}
