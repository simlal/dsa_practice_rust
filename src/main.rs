use std::env;
use std::fs;
use std::path::Path;

// modules for each problem
mod remove_duplicates_sorted_array;

fn list_challenges(main_dir: &Path) {
    match fs::read_dir(main_dir) {
        Ok(entries) => {
            println!("Possible challenges are:");
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if entry.file_name() == "main.rs" {
                            continue;
                        }
                        if let Some(file_stem) = entry.path().file_stem() {
                            println!("{}", file_stem.to_string_lossy());
                        }
                    },
                    Err(e) => println!("Error: {}", e)
                }
            }
        },
        Err(e) => println!("Error: {}", e)
    }
}

struct HelpMessage<'a> {
    base_message: &'a str,
    program_name: &'a str
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_to_run = args.get(1);

    let help_msg = HelpMessage {
        base_message: "Usage: {} <challenge_name>\n\
        If challenge exists, make sure to:\n\
        - Create a new <challenge_name.rs> file\n\
        - Write pub fn run() in it\n\
        - Implement the challenge logic",
        program_name: &args[0],
    };
    let source_dir = Path::new(file!()).parent().unwrap();

    match problem_to_run {
        Some(challenge) => match challenge.as_str() {
            "--help" => println!(
                "{}", help_msg.base_message.replace("{}", help_msg.program_name)
            ),
            "challenges" => list_challenges(&source_dir),
            "remove_duplicates_sorted_array" => {
                println!("Running: <{}>...", challenge);
                remove_duplicates_sorted_array::run();
            }
            other => {
                println!("Unknown challenge <{}>", other);
                list_challenges(&source_dir);
            }
        },
        None => println!(
            "{}", help_msg.base_message.replace("{}", help_msg.program_name)
        ),
    }
}
