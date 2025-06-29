//! Build script for the Zep Key-Value Store library.
//!
//! This build script extracts the package name from Cargo.toml and makes it
//! available as a compile-time environment variable. This allows the library
//! to create storage directories and registry keys using the actual package name.

use cargo::core::SourceId;
use cargo::sources::path::PathSource;
use cargo::util::context::GlobalContext;
use std::env;

/// Main build function that sets up compile-time environment variables.
///
/// This function:
/// 1. Reads the current package information from Cargo.toml
/// 2. Extracts the package name
/// 3. Sets `ZEP_KVS_APP_NAME` as a compile-time environment variable
///
/// The `ZEP_KVS_APP_NAME` variable is used throughout the library to create
/// platform-appropriate storage paths.
fn main() {
    let pwd = env::current_dir().unwrap();
    let pkg = PathSource::new(
        &pwd,
        SourceId::for_path(&pwd).unwrap(),
        &GlobalContext::default().unwrap(),
    )
    .root_package()
    .unwrap();

    // Make the package name available at compile time for storage path construction
    println!("cargo:rustc-env=ZEP_KVS_APP_NAME={}", pkg.name().as_str());
}
