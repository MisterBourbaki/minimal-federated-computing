use pyo3::{
    ffi::c_str, types::{IntoPyDict, PyAnyMethods}, IntoPyObjectExt, PyErr, Python
};
use pyo3::prelude::*;

pub fn some_fun() -> Result<Vec<i32>, PyErr> {
    Python::with_gil(|py| {
        let np = py.import("numpy")?;
        let locals = [("np", np)].into_py_dict(py)?;

        let pyarray = py
            .eval(
                c_str!("np.absolute(np.array([-1, -2, -3], dtype='int32'))"),
                Some(&locals),
                None,
            )?
            .extract::<Vec<i32>>()?;

        Ok(pyarray)
        // pyarray
    })
}

const NUM: usize = 3;

pub fn get_random_numpy_vec(dim_vec: i32, num_vec: i32) -> Result<[Vec<f64>; NUM], PyErr> {
    Python::with_gil(|py| {
        let np = py.import("numpy")?;
        let dim = dim_vec.into_pyobject(py).unwrap();
        let num = num_vec.into_pyobject(py).unwrap();
        let globals = [("np", np)].into_py_dict(py)?;
        let locals = [("dim", &dim), ("num", &num)].into_py_dict(py)?;

        println!("Here is dim {:?}", &dim);

        let pyarray = py
            .eval(
                c_str!("[np.random.rand(dim) for _ in range(num)]"),
                Some(&globals),
                Some(&locals),
            )?
            .extract::<[Vec<f64>; NUM]>()?;

        Ok(pyarray)
        // pyarray
    })
}