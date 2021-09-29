extern crate bindgen;

use std::env;
use std::path::PathBuf;
use bindgen::EnumVariation;


// Gets the library folder name appropriate to the current build configuration
// (e.g. arm, x86, x86_64, etc.)
fn find_libs_path(libs_path: PathBuf) -> Option<PathBuf> {
    let search_paths =
        if cfg!(target_arch = "x86") { vec!["x86"] }
        else if cfg!(target_arch = "x86_64") { vec!["x86_64", "x64"] }
        else { panic!("Unsupported architecture") };

    search_paths.iter().map(|p| libs_path.join(p)).filter(|p| p.exists()).next()
}


fn main() {
    let base_path = PathBuf::from(std::option_env!("FMOD_PATH").unwrap_or("vendor/fmod/api"));
    assert!(base_path.exists());

    let api_paths = vec!["core", "fsbank", "studio"]
        .iter()
        .map(|p| base_path.join(p))
        .collect::<Vec<_>>();

    let include_paths = api_paths
        .iter()
        .map(|p| p.join("inc"))
        .collect::<Vec<_>>();

    let lib_paths = api_paths
        .iter()
        .map(|p| find_libs_path(p.join("lib")).unwrap())
        .collect::<Vec<_>>();

    // XXX: Only works in windows
    let libs = vec!["fmod_vc", "fsbank_vc", "fmodstudio_vc"];

    for lib_path in lib_paths.iter() {
        println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    }
    for lib in libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    println!("cargo:rerun-if-changed=wrapper.h");


    let bindings = bindgen::Builder::default()
        .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
        .clang_args(include_paths.iter().map(|p| format!("-I{}", p.to_str().unwrap())))
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Copy dlls
    for path in lib_paths.iter().flat_map(|p| std::fs::read_dir(p).unwrap())
        .flat_map(|p| p.ok())
        .map(|p| p.path())

    {
        if let Some(ext) = path.extension() {
            if ext.to_str().unwrap() == "dll" {
                let out_path = out_path.join(&path.file_name().unwrap());
                std::fs::copy(&path, out_path.clone()).expect("Failed to copy dll");
            }
        }
    }
}