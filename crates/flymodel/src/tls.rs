use std::{fs::File, io::BufReader, path::PathBuf};

fn default_cert_source() -> CertSource {
    CertSource::System
}

#[derive(Clone, serde::Deserialize, Debug)]
pub enum CertSource {
    File(PathBuf),
    System,
}

#[derive(Clone, serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum CertFileSource {
    Club(PathBuf),
    Pair {
        cert_file: PathBuf,
        key_file: PathBuf,
    },
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct TlsConf {
    #[serde(default = "default_cert_source")]
    pub ca_source: CertSource,
    pub certs: CertFileSource,
}

impl CertFileSource {
    fn rustls_config(&self) -> anyhow::Result<rustls::ServerConfig> {
        const H1_ALPN: &[u8] = b"http/1.1";
        const H2_ALPN: &[u8] = b"h2";

        let mut config = match self {
            CertFileSource::Club(..) => todo!("club files"),
            CertFileSource::Pair {
                cert_file,
                key_file,
            } => {
                let cert_file = File::open(cert_file.canonicalize()?)?;
                let key_file = File::open(key_file.canonicalize()?)?;

                let mut cert_file = BufReader::new(cert_file);
                let mut key_file = BufReader::new(key_file);

                let cert_chain =
                    rustls_pemfile::certs(&mut cert_file).collect::<Result<Vec<_>, _>>()?;

                let mut keys =
                    rustls_pemfile::pkcs8_private_keys(&mut key_file)
                        .collect::<Result<Vec<_>, _>>()?;

                rustls::ServerConfig::builder()
                    .with_no_client_auth()
                    .with_single_cert(
                        cert_chain,
                        rustls::pki_types::PrivateKeyDer::Pkcs8(keys.remove(0)),
                    )
            }
        }?;

        config
            .alpn_protocols
            .append(&mut vec![H1_ALPN.to_vec(), H2_ALPN.to_vec()]);

        Ok(config)
    }
}

impl TlsConf {
    pub fn server_config(&self) -> anyhow::Result<rustls::ServerConfig> {
        self.certs.rustls_config()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::TlsConf;

    #[test]
    fn load_server_certpair() -> anyhow::Result<()> {
        let conf = TlsConf {
            ca_source: super::CertSource::System,
            certs: super::CertFileSource::Pair {
                cert_file: PathBuf::from("../../test-certs/localhost.crt"),
                key_file: PathBuf::from("../../test-certs/localhost.key"),
            },
        };
        conf.server_config()?;
        Ok(())
    }
}
