use pyo3::IntoPy;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ServerError {
    pub code: i64,
    pub kind: String,
}

#[cfg(feature = "python")]
impl IntoPy<pyo3::PyErr> for ServerError {
    fn into_py(self, _: pyo3::Python) -> pyo3::PyErr {
        pyo3::exceptions::PyValueError::new_err(format!("{} ({})", self.kind, self.code))
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Result<T> {
    Err(ServerError),
    Ok(T),
}
