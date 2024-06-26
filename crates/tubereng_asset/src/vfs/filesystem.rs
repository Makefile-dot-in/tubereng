use log::trace;

use super::VirtualFileSystem;
use crate::{AssetError, Result};

pub struct FileSystem;
impl VirtualFileSystem for FileSystem {
    fn read_bytes(&self, path: &str) -> Result<Vec<u8>> {
        trace!("Reading bytes from {}", path);
        std::fs::read(path).map_err(|_| AssetError::ReadFailed)
    }
}
