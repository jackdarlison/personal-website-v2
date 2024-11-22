use std::process::Command;


fn main() {

    println!("Running build Script");

    let output = Command::new("pnpm")
        .arg("dlx")
        .arg("tailwind")
        .arg("-i")
        .arg("styles/tailwind.css")
        .arg("-o")
        .arg("static/main.css")
        .output()
        .expect("Failed to create css");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    println!("cargo:rerun-if-changed=build.rs");
}