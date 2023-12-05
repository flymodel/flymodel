use std::path::PathBuf;

fn default_cert_source() -> CertSource {
    CertSource::System
}

#[derive(Clone, serde::Deserialize, Debug)]
pub enum CertSource {
    File(PathBuf),
    System,
}
#[derive(Clone, serde::Deserialize, Debug)]
pub enum CertFileSource {
    Club(PathBuf),
    Pair { cert: PathBuf, key: PathBuf },
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct TlsConf {
    pub tls: bool,
    #[serde(default = "default_cert_source")]
    pub ca_source: CertSource,
    pub client: Option<CertFileSource>,
}
