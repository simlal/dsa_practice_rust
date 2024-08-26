use std::env;
use std::fs;
use std::path::Path;

// modules for each problem
mod remove_duplicates_sorted_array;
mod remove_element;
mod get_concatenation;


fn get_challenges(main_dir: &Path, print_stdout: bool) -> Vec<String> {
    // Use filestems for all non-'main.rs' files inside /src as challenges
    let mut challenges: Vec<String> = Vec::new();
    
    match fs::read_dir(main_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if entry.file_name() == "main.rs" {
                            continue;
                        }
                        if let Some(file_stem) = entry.path().file_stem() {
                            if print_stdout {
                                println!("{}", file_stem.to_string_lossy());
                            }
                            challenges.push(file_stem.to_string_lossy().to_string());
                        }
                    },
                    Err(e) => println!("Error: {}", e)
                }
            }
        },
        Err(e) => println!("Error: {}", e)
    }
    challenges
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
        - Add it in the match expression in `main`\n\
        - Implement the challenge logic",
        program_name: &args[0],
    };
    let source_dir = Path::new(file!()).parent().unwrap();
    let challenges = get_challenges(source_dir, false);

    match problem_to_run {
        Some(problem_to_run) => {
            if problem_to_run == "help" {
                println!(
                    "{}", help_msg.base_message.replace("{}", help_msg.program_name)
                );
            } else if problem_to_run == "list-challenges" {
                get_challenges(source_dir, true);
            } else if challenges.contains(problem_to_run) {
                println!("Running the <{}> challenge...", problem_to_run);
                match problem_to_run.as_str() {
                    "remove_duplicates_sorted_array" => remove_duplicates_sorted_array::run(),
                    "remove_element" => remove_element::run(),
                    "get_concatenation" => get_concatenation::run(),
                    _ => {
                        println!("Unknown challenge <{}>", problem_to_run);
                    }
                }
            } else {
                println!("Unknown challenge <{}>", problem_to_run);
                println!("Make sure the problem to run is one of:");
                for challenge in challenges {
                    println!("\t{}", challenge);
                }
            }
        }
        None => println!(
            "{}", help_msg.base_message.replace("{}", help_msg.program_name)
        ),
    }
}