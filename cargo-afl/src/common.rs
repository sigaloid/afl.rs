#![deny(clippy::disallowed_macros, clippy::expect_used, clippy::unwrap_used)]

use std::env;
use std::ffi::OsStr;
use std::io::{Error, Result};
use std::path::{Path, PathBuf};

fn xdg_dir() -> Result<xdg::BaseDirectories> {
    let afl_rustc_version = afl_rustc_version()?;
    let prefix = Path::new("afl.rs")
        .join(afl_rustc_version)
        .join(pkg_version());
    xdg::BaseDirectories::with_prefix(prefix).map_err(Error::other)
}

fn data_dir(dir_name: &str) -> Result<PathBuf> {
    // For docs.rs builds, use OUT_DIR.
    // For other cases, use a XDG data directory.
    // It is necessary to use OUT_DIR for docs.rs builds,
    // as that is the only place where we can write to.
    // The Cargo documentation recommends that build scripts
    // place their generated files at OUT_DIR too, but we
    // don't change that for now for normal builds.
    // smoelius: AFL++ is no longer built on docs.rs.
    let xdg_dir = xdg_dir()?;
    println!("{:?}", xdg_dir);
    xdg_dir.create_data_directory(dir_name)
}

const SHORT_COMMIT_HASH_LEN: usize = 7;

pub fn afl_rustc_version() -> Result<String> {
    let version_meta = rustc_version::version_meta().map_err(Error::other)?;
    let mut ret = String::from("rustc-");
    ret.push_str(&version_meta.semver.to_string());
    if let Some(commit_hash) = version_meta.commit_hash {
        ret.push('-');
        ret.push_str(&commit_hash[..SHORT_COMMIT_HASH_LEN]);
    }
    Ok(ret)
}

#[allow(clippy::disallowed_macros)]
fn pkg_version() -> String {
    let mut ret = String::from("afl.rs-");

    let version = env!("CARGO_PKG_VERSION");
    assert!(!version.is_empty());

    ret.push_str(version);
    ret
}

#[allow(dead_code)]
pub fn afl_dir() -> Result<PathBuf> {
    data_dir("afl")
}

#[allow(dead_code)]
pub fn afl_llvm_dir() -> Result<PathBuf> {
    data_dir("afl-llvm")
}

#[allow(dead_code)]
pub fn object_file_path() -> Result<PathBuf> {
    afl_llvm_dir().map(|path| path.join("libafl-llvm-rt.o"))
}

#[allow(dead_code)]
pub fn archive_file_path() -> Result<PathBuf> {
    afl_llvm_dir().map(|path| path.join("libafl-llvm-rt.a"))
}

#[allow(dead_code)]
pub fn plugins_available() -> Result<bool> {
    let afl_llvm_dir = afl_llvm_dir()?;
    for result in afl_llvm_dir.read_dir()? {
        let entry = result?;
        let file_name = entry.file_name();
        if Path::new(&file_name).extension() == Some(OsStr::new("so")) {
            return Ok(true);
        }
    }
    Ok(false)
}
