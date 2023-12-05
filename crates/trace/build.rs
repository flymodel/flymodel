use std::process::Command;

fn git_cmd(args: &[&str]) -> String {
    let output = Command::new("git").args(args).output().expect("git info");
    String::from_utf8(output.stdout).expect("valid utf-8")
}

fn main() {
    let git_hash = git_cmd(&["rev-parse", "HEAD"]);
    let diff = git_cmd(&["diff"]);
    let clean = diff.trim().is_empty();
    if clean {
        println!("cargo:rustc-env=GIT_STATUS=Clean");
    } else {
        println!("cargo:rustc-env=GIT_STATUS=Dirty");
    }

    let branch = git_cmd(&["rev-parse", "--abbrev-ref", "HEAD"]);
    println!("cargo:rustc-env=GIT_BRANCH={}", branch);

    let hash_suffix = {
        if clean {
            ""
        } else {
            "-dirty"
        }
    };
    println!("cargo:rustc-env=GIT_HASH={}{}", git_hash, hash_suffix);
}
