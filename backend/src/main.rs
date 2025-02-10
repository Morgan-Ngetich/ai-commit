mod ai_model;

use std::process::Command;
use std::io::{self, Write};
use std::time::Duration;
// use std::path::Path;
// use std::env;
// use std::fs;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use ai_model::{generate_commit_message, get_selected_model};



fn main() {

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} Fetching staged changes...")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(120));

    // Get staged files
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()
        .expect("Failed to execute git command");
    
    pb.finish_and_clear();
    let changed_files = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if changed_files.is_empty() {
        println!("{}", "🚫 No staged changes found.".red());
        return;
    }

    // Get file diffs
    let diff_output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .expect("Failed to get git diff");
    let file_diffs = String::from_utf8_lossy(&diff_output.stdout).trim().to_string();

    // Get the selected AI model
    let model = get_selected_model();

    // Call AI model
    match generate_commit_message(&changed_files, &file_diffs, &model) {
        Ok(commit_message) => {
            println!("\n{}: {}\n", "✨ Suggested Commit Message".cyan().bold(), commit_message);

            // Ask for confirmation
            print!("Do you want to use this message? (Y/n): ");
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();
            let user_input = user_input.trim().to_lowercase();

            if user_input == "n" {
                println!("{}", "Commit message not applied. You can enter your own.".yellow());
            } else {
                let status = Command::new("git")
                    .args(["commit", "-m", &commit_message])
                    .status()
                    .expect("Failed to execute git commit");

                if status.success() {
                    println!("{}", "✅ Commit successful!".green().bold());
                } else {
                    println!("{}", "❌ Commit failed!".red().bold());
                }
            }
        }
        Err(err) => {
            println!("{}", format!("❌ Error generating commit message: {}", err).red());
        }
    }
}
