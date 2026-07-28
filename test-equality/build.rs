use std::process::Command;

fn main() {
    // try to retrieve current commit hash
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output();

    // if successfull, set it to the current environment
    if let Ok(output) = output {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout);
            println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_hash.trim());
        }
    }

    // retrieve the list of enabled features
    let enabled_features = std::env::vars()
        .into_iter()
        .map(|(key, _)| key)
        .filter(|key| key.starts_with("CARGO_FEATURE_"))
        .map(|key| key.trim_start_matches("CARGO_FEATURE_").to_lowercase())
        .collect::<Vec<_>>();
    println!("cargo:rustc-env=ENABLED_FEATURES={}", format!("[{}]", enabled_features.join(",")));
}
