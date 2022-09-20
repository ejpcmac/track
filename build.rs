use std::{io, process::Command};

fn main() {
    define_version_with_git();
}

/// Defines a variable containing the version with the Git revision.
///
/// `VERSION_WITH_GIT` contains at least the cargo version, even when Git is not
/// available. When Git is available, if the current Git tag and dirty state
/// does not match *exactly* the cargo version prefixed by `v`, then the current
/// Git revision and dirty state is added to the version as a tag.
///
/// For instance:
///
/// * Cargo version 1.0.0 on tag v1.0.0, clean state => `1.0.0`
/// * Cargo version 1.0.0 on tag v1.0.0, dirty state => `1.0.0+abcd1234-modified`
/// * Cargo version 1.1.0-dev on any commit, clean state => `1.1.0-dev+abcd1234`
fn define_version_with_git() {
    let cargo_version = env!("CARGO_PKG_VERSION");
    let version = version_with_git(cargo_version)
        .unwrap_or_else(|_| String::from(cargo_version));

    println!("cargo:rustc-env=VERSION_WITH_GIT={version}");
}

fn version_with_git(cargo_version: &str) -> io::Result<String> {
    if git_describe()? != format!("v{cargo_version}") {
        let revision = git_revision_and_state()?;
        Ok(format!("{cargo_version}+{revision}"))
    } else {
        Ok(String::from(cargo_version))
    }
}

fn git_describe() -> io::Result<String> {
    let output = Command::new("git")
        .args(&["describe", "--always", "--dirty=-modified"])
        .output()?;
    Ok(String::from_utf8(output.stdout).unwrap().trim().to_owned())
}

fn git_revision_and_state() -> io::Result<String> {
    let revision = git_revision()?;
    if git_is_dirty()? {
        Ok(format!("{revision}-modified"))
    } else {
        Ok(revision)
    }
}

fn git_revision() -> io::Result<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()?;
    Ok(String::from_utf8(output.stdout).unwrap().trim().to_owned())
}

fn git_is_dirty() -> io::Result<bool> {
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()?;
    Ok(!output.stdout.is_empty())
}
