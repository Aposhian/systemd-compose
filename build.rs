// build.rs

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::Write;

use schemafy_lib::Expander;

fn main() {
    let res =reqwest::blocking::get("https://raw.githubusercontent.com/compose-spec/compose-spec/master/schema/compose-spec.json").unwrap();
    let schema = serde_json::from_str(&res.text().unwrap()).unwrap();
    let mut expander = Expander::new(Some("compose-spec"), "", &schema);
    let code = expander.expand(&schema);
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("compose-spec.rs");
    let mut code_string = String::from("use serde::{Serialize, Deserialize};\n");
    code_string.push_str(&code.to_string());
    fs::write(&dest_path, code_string).unwrap();
    Command::new("rustfmt")
        .arg(&dest_path)
        .output();
    println!("cargo:rerun-if-changed=build.rs");
}