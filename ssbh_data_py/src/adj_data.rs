use crate::{mesh_data, MapPy};
use pyo3::types::PyType;
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

    #[pyo3(get, set)]
    pub vertex_adjacency: PyObject,
}

#[pymethods]
impl AdjEntryData {
    #[new]
    fn new(py: Python, mesh_object_index: usize) -> PyResult<Self> {
        Ok(Self {
            mesh_object_index,
            vertex_adjacency: PyList::empty(py).into(),
        })
    }

    #[classmethod]
    fn from_mesh_object(
        _cls: &PyType,
        py: Python,
        mesh_object_index: usize,
        mesh_object: &mesh_data::MeshObjectData,
    ) -> PyResult<Self> {
        let vertex_indices: Vec<u32> = mesh_object.vertex_indices.extract(py)?;
        let positions: Vec<mesh_data::AttributeData> = mesh_object.positions.extract(py)?;
        // TODO: Avoid unwrap?
        let vertex_positions = positions.first().unwrap().data.map_py(py)?;
        let entry = ssbh_data::adj_data::AdjEntryData::from_vector_data(
            mesh_object_index,
            &vertex_positions,
            &vertex_indices,
        );
        entry.map_py(py)
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
    use crate::{run_python_code, run_python_code_numpy};
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

    #[test]
    fn vertex_adjacency_tuples() {
        run_python_code(indoc! {r#"
            e = ssbh_data_py.adj_data.AdjEntryData(3)
            assert e.mesh_object_index == 3
            e.vertex_adjacency = (-1, 3, 7)
            assert list(e.vertex_adjacency) == [-1, 3, 7]
        "#})
        .unwrap();
    }

    #[test]
    fn vertex_adjacency_numpy() {
        run_python_code_numpy(indoc! {r#"
            e = ssbh_data_py.adj_data.AdjEntryData(3)
            assert e.mesh_object_index == 3
            e.vertex_adjacency = numpy.array([-1, 3, 7])
            assert e.vertex_adjacency.tolist() == [-1, 3, 7]
        "#})
        .unwrap();
    }
}