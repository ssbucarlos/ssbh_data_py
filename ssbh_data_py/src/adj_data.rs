use crate::MapPy;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::SsbhData;
use ssbh_data_py_derive::MapPy;

create_exception!(ssbh_data_py, AdjDataError, pyo3::exceptions::PyException);

pub fn adj_data(py: Python, module: &PyModule) -> PyResult<()> {
    let adj_data = PyModule::new(py, "adj_data")?;
    adj_data.add_class::<AdjData>()?;
    adj_data.add_class::<AdjEntryData>()?;
    adj_data.add_function(wrap_pyfunction!(read_adj, adj_data)?)?;

    module.add_submodule(adj_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.adj_data")]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::adj_data::AdjData)]
struct AdjData {
    #[pyo3(get, set)]
    pub entries: Py<PyList>,
}

#[pymethods]
impl AdjData {
    #[new]
    fn new(py: Python) -> PyResult<Self> {
        Ok(AdjData {
            entries: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py)?
            .write_to_file(path)
            .map_err(|e| AdjDataError::new_err(format!("{}", e)))
    }
}

#[pyclass(module = "ssbh_data_py.adj_data")]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::adj_data::AdjEntryData)]
struct AdjEntryData {
    #[pyo3(get, set)]
    pub mesh_object_index: usize,

    // TODO: Should this be PyObject to allow numpy arrays, tuples, etc?
    #[pyo3(get, set)]
    pub vertex_adjacency: Py<PyList>,
}

#[pymethods]
impl AdjEntryData {
    #[new]
    fn new(py: Python, mesh_object_index: usize) -> PyResult<Self> {
        Ok(AdjEntryData {
            mesh_object_index,
            vertex_adjacency: PyList::empty(py).into(),
        })
    }
}

#[pyfunction]
fn read_adj(py: Python, path: &str) -> PyResult<AdjData> {
    ssbh_data::adj_data::AdjData::from_file(path)
        .map_err(|e| AdjDataError::new_err(format!("{}", e)))?
        .map_py(py)
}

#[cfg(test)]
mod tests {
    use crate::run_python_code;
    use indoc::indoc;

    #[test]
    fn create_adj() {
        run_python_code(indoc! {r#"
            a = ssbh_data_py.adj_data.AdjData()
            assert a.entries == []
        "#})
        .unwrap();
    }

    #[test]
    fn create_adj_entry() {
        run_python_code(indoc! {r#"
            e = ssbh_data_py.adj_data.AdjEntryData(3)
            assert e.mesh_object_index == 3
            assert e.vertex_adjacency == []
        "#})
        .unwrap();
    }
}
