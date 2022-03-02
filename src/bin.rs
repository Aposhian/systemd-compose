use std::env;
use std::path::Path;
use std::fs;

mod convert;
use convert::generate_files;

// TODO: change this to be dynamic based on whether this is a system or user generator
// For now we will only worry about system generators
const COMPOSE_FILE_DIR: &'static str = "/etc/systemd-compose";

fn main() {
    let args: Vec<String> = env::args().collect();
    // https://www.freedesktop.org/software/systemd/man/systemd.generator.html#Output%20directories
    assert!(args.len() == 4);
    let normal_dir = Path::new(&args[1]);
    let _early_dir = Path::new(&args[2]);
    let _late_dir = Path::new(&args[3]);

    // Get all the files at the top level of COMPOSE_FILE_DIR
    fs::read_dir(COMPOSE_FILE_DIR).expect("COMPOSE_FILE_DIR could not be read")
        .filter_map(|result| match result {
            Ok(entry) => if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    Some(entry.path())
                } else {
                    None
                }
            } else {
                None
            },
            Err(_) => None
        })
        .for_each(|path| {
            generate_files(path, normal_dir);
        });
}