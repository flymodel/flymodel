use std::fmt::Display;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ServerError {
    pub code: i64,
    pub kind: String,
}

#[cfg(feature = "python")]
impl pyo3::IntoPy<pyo3::PyErr> for ServerError {
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

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.kind, self.code)
    }
}

impl<T> Result<T> {
    pub fn map_err<E>(self, f: impl FnOnce(ServerError) -> E) -> std::result::Result<T, E> {
        match self {
            Result::Err(e) => Err(f(e)),
            Result::Ok(v) => Ok(v),
        }
    }
}
