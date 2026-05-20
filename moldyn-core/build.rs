//! Build script for `moldyn_core`

use std::{env, ffi::OsString, process::Command};

fn main() {
    //
    println!("cargo:rustc-check-cfg=cfg(nightly)");

    // https://docs.rs/select-rustc/0.1.2/src/rustc/rustc.rs.html#50
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("Failed to execute rustc to get version");
    let version_str =
        String::from_utf8(output.stdout).expect("Failed to parse rustc version output");

    // https://docs.rs/select-rustc/0.1.2/src/rustc/rustc.rs.html#64
    let last_line = version_str.lines().last().unwrap_or(&version_str);
    let mut words = last_line.trim().split(' ');

    match words.next() {
        Some("rustc") => (),
        _ => return,
    }

    let mut version_channel = words
        .next()
        .expect("Failed to get version channel")
        .split('-');
    let _version = version_channel.next().expect("Failed to get version");
    let channel = version_channel.next();

    // https://users.rust-lang.org/t/add-unstable-feature-only-if-compiled-on-nightly/27886/2
    if channel == Some("nightly") {
        println!("cargo:rustc-cfg=nightly");
    }
}
