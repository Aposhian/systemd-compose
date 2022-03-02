use std::{path::{Path, PathBuf}, error::Error};
use crate::compose::Compose;
use systemd_schema::prelude::*;

pub fn generate_files(compose_file_path: PathBuf, output_directory: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>>{
    unimplemented!();
}

pub fn convert(compose: Compose) -> Result<Vec<Service>, Box<dyn Error>> {
    unimplemented!();
}