use serde::Deserialize;

use super::enums::{ArchiveEncoding, ArchiveFormat};

#[derive(Debug, Deserialize)]
pub struct UploadBlobRequestParams {
    pub artifact_name: String,
    pub archive: Option<ArchiveFormat>,
    pub encode: Option<ArchiveEncoding>,
}
