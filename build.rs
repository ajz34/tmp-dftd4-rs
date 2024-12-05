use std::{error::Error, path::PathBuf};

/// Generate link search paths from a list of paths.
///
/// This allows paths like `/path/to/lib1:/path/to/lib2` to be split into individual paths.
fn generate_link_search_paths(paths: &[Result<String, impl Error + Clone>]) -> Vec<String> {
    paths
        .iter()
        .map(|path| {
            path.clone()
                .unwrap_or_default()
                .split(":")
                .map(|path| path.to_string())
                .collect::<Vec<_>>()
        })
        .into_iter()
        .flatten()
        .filter(|path| !path.is_empty())
        .collect::<Vec<_>>()
}

/// Check if the library is found in the given paths.
fn check_library_found(
    lib_name: &str,
    lib_paths: &[String],
    lib_extension: &[String],
) -> Option<String> {
    for path in lib_paths {
        for ext in lib_extension {
            let lib_path = PathBuf::from(&path).join(format!("lib{}.{}", lib_name, ext));
            if lib_path.exists() {
                return Some(lib_path.to_string_lossy().to_string());
            }
        }
    }
    return None;
}

fn main() {
    // search dirs
    for key in ["DFTD4_DIR", "REST_EXT_DIR"].iter() {
        println!("cargo:rerun-if-env-changed={}", key);
    }
    let lib_paths = generate_link_search_paths(&[
        std::env::var("DFTD4_DIR"),
        std::env::var("REST_EXT_DIR"),
        std::env::var("LD_LIBRARY_PATH"),
    ]);
    // static linking or anyway
    if cfg!(feature = "static") {
        if let Some(path) = check_library_found("dftd4", &lib_paths, &["a".to_string()]) {
            let path = std::fs::canonicalize(path).unwrap();
            let path = path.parent().unwrap().display();
            println!("cargo:rustc-link-search=native={}", path);
        } else {
            let dst = cmake::Config::new("external_deps").build();
            println!("cargo:rustc-link-search=native={}/lib", dst.display());
        }
        println!("cargo:rustc-link-lib=static=dftd4");
        println!("cargo:rustc-link-lib=static=multicharge");
        println!("cargo:rustc-link-lib=static=mctc-lib");
        println!("cargo:rustc-link-lib=blas");
        println!("cargo:rustc-link-lib=lapack");
        println!("cargo:rustc-link-lib=gomp");
        println!("cargo:rustc-link-lib=gfortran");
    } else {
        if let Some(path) = check_library_found("dftd4", &lib_paths, &["so".to_string()]) {
            let path = std::fs::canonicalize(path).unwrap();
            let path = path.parent().unwrap().display();
            println!("cargo:rustc-link-search=native={}", path);
            println!("cargo:rustc-link-lib=dftd4");
        } else {
            let dst = cmake::Config::new("external_deps")
                .define("BUILD_SHARED_LIBS", "1")
                .build();
            println!("cargo:rustc-link-search=native={}/lib", dst.display());
            println!("cargo:rustc-link-lib=multicharge");
            println!("cargo:rustc-link-lib=mctc-lib");
            println!("cargo:rustc-link-lib=dftd4");
        }
    }
}
