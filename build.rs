use std::{env, ffi::OsStr, fs, path::PathBuf};

fn main() {
  let target = env::var("TARGET").unwrap();
  if !target.contains("windows") { return }

  let is_debug = env::var("DEBUG").unwrap() == "true";

  let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let lib_dir = root_dir.join("lib");
  println!("cargo:rustc-link-search=all={}", lib_dir.display());

  let target_dir = root_dir
    .join("target")
    .join(if is_debug { "debug" } else { "release" });
  let target_dir = &target_dir;

  fs::create_dir_all(target_dir).unwrap();
  for file in fs::read_dir(lib_dir).unwrap().map(|e|e.unwrap().path()) {
    if file.extension() == Some(OsStr::new("dll")) {
      let new_path = target_dir.join(file.file_name().unwrap().to_str().unwrap());
      fs::copy(file, new_path).unwrap();
    }
  }
}