use std::io::Result;
use std::path::PathBuf;

fn delinkify(path: &PathBuf) -> PathBuf {
    let mut delinked = PathBuf::new();
    for comp in path.components() {
        delinked.push(comp);

        /* make sure that `delinked` isn't a link */
        while let Ok(target) = std::fs::read_link(&delinked) {
            delinked = target;
        }
    }

    delinked
}

fn path() -> Result<String> {
    let current = delinkify(&std::env::current_dir()?)
        .to_string_lossy()
        .to_string();
    let home = delinkify(&dirs::home_dir().unwrap())
        .to_string_lossy()
        .to_string();

    let result = if current.starts_with(&home) {
        current.replacen(&home, "~", 1)
    } else {
        current
    };

    Ok(result)
}

fn emoji(username: &str) -> &'static str {
    if username == "root" {
        "🔥"
    } else {
        "🚀"
    }
}

fn print_prompt_fallible() -> Result<()> {
    use colored::*;

    let username = whoami::username();
    let hostname = whoami::hostname();
    let path = path()?;
    let emoji = emoji(&username);

    control::set_override(true);
    println!(
        "{user}{at}{host}{colon}{path} {emoji} ",
        user  = username.bright_green().bold(),
        at    = "@".bright_green().bold(),
        host  = hostname.bright_green().bold(),
        colon = ":".white().bold(),
        path  = path.cyan().bold(),
        emoji = emoji,
    );

    Ok(())
}

fn print_prompt_infallible() {
    println!("[ps1 failed] $");
}

fn main() {
    if let Err(_) = print_prompt_fallible() {
        print_prompt_infallible();
    }
}
