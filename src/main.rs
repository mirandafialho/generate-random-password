use std::env;

fn help() {
    println!("Usage:
randpass --size=<size> --special-chars={{true|false}}

Options:
    -h, --help          Display this message
    -s, --size          Size of your random password (default=16)
    -S, --special-chars Random password with special chars or only letters and numbers (default=false)");
}

fn generate_password(password: &str) -> String {
    let mut reverse_password = String::new();

    for charac in password.chars().rev() {
        reverse_password.push(charac);
    }

    return reverse_password;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let new_password = String::from("fialho");

    match args.len() {
        1 => {
            print!("New password: {}", generate_password(&new_password));
        },
        _ => {
            help();
        }
    }
}
