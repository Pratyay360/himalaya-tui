use pimalaya_toolbox::build::{features_env, target_envs};

fn main() {
    features_env(include_str!("./Cargo.toml"));
    target_envs();

    // Set git envs manually until first commit exists
    println!("cargo::rustc-env=GIT_DESCRIBE=v0.1.0");
    println!("cargo::rustc-env=GIT_REV=dev");
}
