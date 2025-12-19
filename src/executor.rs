use std::process::{Command, Stdio};

pub fn run_command(command: &str) -> std::io::Result<()> {
    println!("Executing: {}", command);
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
    }

    Ok(())
}
