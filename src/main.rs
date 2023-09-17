use std::{env, fs, process::ExitCode};

// much more elegant solution than the monstrosity I had gotten to
// https://stackoverflow.com/a/58113997
fn bin_name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

fn main() -> ExitCode {
    let file_path = env::args().skip(1).next().unwrap_or("--help".to_owned());

    if file_path == "--help" {
        let bin = match bin_name() {
            Some(name) => name,
            None => "read-dotenv".to_owned(),
        };

        // These packages look great but we don't need KBs of
        // dependencies for three lines of ANSI codes here. :]
        // https://lib.rs/crates/colored
        // https://lib.rs/crates/termcolor
        let red = "\x1b[38;5;203m";
        let reset = "\x1B[0m";
        let grey = "\x1b[38;5;247m";

        eprintln!("‼️  {}Provide path to a .env file!{}", red, reset);
        eprintln!("{}{} ~/example.env{}", grey, bin, reset);

        return ExitCode::FAILURE;
    }

    let content = fs::read_to_string(file_path).expect("could not read .env file");

    let help = content
        .split("\n")
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
        .map(|line| {
            if line.starts_with("export") {
                return line.to_owned();
            } else {
                return format!("export {}", line);
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", help);
    return ExitCode::SUCCESS;
}
