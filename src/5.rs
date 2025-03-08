use std::process::Command;

fn main() {
    let output = Command::new("my_program")
        .arg("--help")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    println!("{}", stdout);
    println!("{}", stderr);
}
