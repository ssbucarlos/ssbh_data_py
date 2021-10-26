use pyo3::{prelude::*, types::PyList};

mod anim_data;
mod mesh_data;
mod modl_data;
mod skel_data;

#[pymodule]
fn ssbh_data_py(py: Python, module: &PyModule) -> PyResult<()> {
    crate::mesh_data::mesh_data(py, module)?;
    crate::modl_data::modl_data(py, module)?;
    crate::skel_data::skel_data(py, module)?;
    crate::anim_data::anim_data(py, module)?;
    Ok(())
}

fn create_py_list_from_slice<T: IntoPy<U> + Copy, U: ToPyObject>(
    py: Python,
    elements: &[T],
) -> Py<PyList> {
    PyList::new(py, elements.iter().map(|m| m.into_py(py))).into()
}

// Define a mapping between types.
// This allows for deriving the Python <-> Rust conversion.
// TODO: It may be possible to use PyO3 for this in the future.
// The derive macro is mainly to automate mapping field names.
trait MapPy<T> {
    fn map_py(&self, py: Python) -> PyResult<T>;
}

// We want a conversion from Vec<T> -> Py<PyList>.
// We can't implement ToPyObject for ssbh_lib types in ssbh_data_py.
// Use MapPy<PyObject> instead to utilize the ssbh_data -> ssbh_data_py conversion.
impl<T: MapPy<PyObject>> MapPy<Py<PyList>> for Vec<T> {
    fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
        Ok(PyList::new(py, self.iter().map(|e| e.map_py(py).unwrap())).into())
    }
}

// Similarly, we need to define a conversion from Py<PyList> -> Vec<T>.
// The element type of a PyList is PyAny, so we can use a mapping from PyObject (Py<PyAny>) to T.
impl<T> MapPy<Vec<T>> for Py<PyList>
where
    PyObject: MapPy<T>,
{
    fn map_py(&self, py: Python) -> PyResult<Vec<T>> {
        Ok(self.as_ref(py).iter().map(|e| PyObject::from(e).map_py(py).unwrap()).collect())
    }
}

// Implement for primitive types.
macro_rules! map_py_impl {
    ($($t:ty),*) => {
        $(
            impl MapPy<$t> for $t {
                fn map_py(&self, _py: Python) -> PyResult<$t> {
                    Ok(self.clone())
                }
            }

            // Define the Rust <-> Python conversion to support the Vec <-> PyList conversion.
            impl MapPy<PyObject> for $t {
                fn map_py(
                    &self,
                    py: Python,
                ) -> PyResult<PyObject> {
                    Ok(self.into_py(py))
                }
            }

            impl MapPy<$t> for PyObject {
                fn map_py(&self, py: Python) -> PyResult<$t> {
                    self.extract(py)
                }
            }
        )*
    }
}

map_py_impl!(bool, u8, u16, u32, u64, u128, f32, f64, String);

macro_rules! map_py_pyobject_impl {
    ($($t:ty),*) => {
        $(
            impl MapPy<$t> for PyObject {
                fn map_py(&self, py: Python) -> PyResult<$t> {
                    self.extract(py)
                }
            }
        )*
    }
}

// TODO: Derive this?
map_py_pyobject_impl!([[f32; 4]; 4]);
impl MapPy<PyObject> for [[f32; 4]; 4] {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(create_py_list_from_slice(py, self).into())
    }
}

map_py_pyobject_impl!(Vec<u32>);
impl MapPy<PyObject> for Vec<u32> {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(create_py_list_from_slice(py, self).into())
    }
}

impl<T: Clone> MapPy<Option<T>> for Option<T> {
    fn map_py(&self, _py: Python) -> PyResult<Option<T>> {
        Ok(self.clone())
    }
}

#[cfg(test)]
fn run_python_code(code: &str) -> PyResult<()> {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();
    let ctx = [("ssbh_data_py", module)].into_py_dict(py);
    py.run(code, None, Some(ctx))
}

#[cfg(test)]
fn run_python_code_numpy(code: &str) -> PyResult<()> {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();

    // TODO: This requires numpy to be in the current Python environment,
    // which may require some configuration to run tests with github actions.
    let ctx = [
        ("ssbh_data_py", module),
        ("numpy", PyModule::import(py, "numpy").unwrap()),
    ]
    .into_py_dict(py);

    py.run(code, None, Some(ctx))
}

#[cfg(test)]
fn eval_python_code<F: Fn(Python, &PyAny)>(code: &str, f: F) {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();
    let ctx = [("ssbh_data_py", module)].into_py_dict(py);

    let result = py.eval(code, None, Some(ctx)).unwrap();
    f(py, result);
}

#[cfg(test)]
fn eval_python_code_numpy<F: Fn(Python, &PyAny)>(code: &str, f: F) {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();

    // TODO: This requires numpy to be in the current Python environment,
    // which may require some configuration to run tests with github actions.
    let ctx = [
        ("ssbh_data_py", module),
        ("numpy", PyModule::import(py, "numpy").unwrap()),
    ]
    .into_py_dict(py);

    let result = py.eval(code, None, Some(ctx)).unwrap();
    f(py, result);
}
