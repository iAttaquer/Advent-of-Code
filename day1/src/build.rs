use std::env;
use std::fs;
use std::path::Path;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let src_path = Path::new("test.txt");
  let dest_path = Path::new(&out_dir).join("test.txt");

  fs::copy(src_path, dest_path).expect("Unable to copy file");
}