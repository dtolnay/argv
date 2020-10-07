use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if let Ok(target) = env::var("TARGET") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let out = Path::new(&out_dir).join("target.rs");
        let constant = format!("const TARGET: &str = \"{}\";\n", target);
        fs::write(out, constant).unwrap();
    }
}
