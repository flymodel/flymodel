use pyo3::prelude::*;

use crate::FlymodelClient;

#[pymodule]
fn flymodel_client(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FlymodelClient>()?;
    m.add_submodule(flymodel_graphql::py::submodule(py)?)?;
    Ok(())
}
