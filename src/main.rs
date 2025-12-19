mod distro;
mod ollama;
mod executor;

use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let distro_details = distro::get_distro().unwrap_or_else(|_| distro::DistroDetails {
        id: "unknown".to_string(),
        id_like: None,
    });

    println!("AI Assistant Initialized.");
    println!("Distro: {} (like: {:?})", distro_details.id, distro_details.id_like);
    println!("Enter a command, or 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match ollama::get_command_from_prompt(input, &distro_details.id).await {
            Ok(command) => {
                if let Err(e) = executor::run_command(&command) {
                    eprintln!("Failed to execute command: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error getting command from Ollama: {}", e);
            }
        }
    }

    Ok(())
}