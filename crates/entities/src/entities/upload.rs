use serde::Deserialize;

use super::enums::{ArchiveCompression, ArchiveFormat};

#[derive(Debug, Deserialize, Clone)]
pub struct UploadBlobRequestParams {
    pub artifact_name: String,
    pub encode: Option<ArchiveCompression>,
    pub format: Option<ArchiveFormat>,
}
