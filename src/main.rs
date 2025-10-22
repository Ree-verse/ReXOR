use clearscreen::clear;
use colored::Colorize;
use rexor::{decode, encode};
use rfd::FileDialog;
use rpassword::prompt_password;
use std::io::{Write, stdin, stdout};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let ascii_art = r#"
 ██████╗ ███████╗██╗  ██╗ ██████╗ ██████╗
 ██╔══██╗██╔════╝╚██╗██╔╝██╔═══██╗██╔══██╗
 ██████╔╝█████╗   ╚███╔╝ ██║   ██║██████╔╝
 ██╔══██╗██╔══╝   ██╔██╗ ██║   ██║██╔══██╗
 ██║  ██║███████╗██╔╝ ██╗╚██████╔╝██║  ██║
 ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝
                    by ree-verse, v0.1.0
"#; // Ascii art generated from: https://patorjk.com/software/taag
    let bar = "-".repeat(50);

    loop {
        clear().unwrap();
        print!(
            "{}{}\nWelcome to ReXOR! What would you like to do?\n\
             >> 1) Encode a file\n\
             >> 2) Decode a file\n\
             >> 3) Exit\n{}\n\
             Please enter the number \"1\", \"2\", or \"3\": ",
            ascii_art.red(),
            bar,
            bar
        );
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => {
                if let Some(input_path) = FileDialog::new()
                    .set_title("Select a file to encode")
                    .pick_file()
                {
                    let password = prompt_password("Enter the password: ").unwrap();

                    if password.is_empty() {
                        println!(
                            "{} The password cannot be empty",
                            "Error encoding file:".red()
                        ); // Necessary to avoid showing output dialog when the password is empty (this is required even though the library already checks it)
                        sleep(Duration::from_secs(2));
                        continue;
                    }

                    let file_name = input_path.file_name().unwrap().to_str().unwrap();
                    let encoded_name = format!("{}.rxor", file_name);

                    if let Some(output_path) = FileDialog::new()
                        .set_title("Select output file location")
                        .set_file_name(encoded_name)
                        .save_file()
                    {
                        match encode(
                            input_path.to_str().unwrap(),
                            &password,
                            Some(output_path.to_str().unwrap()),
                        ) {
                            Ok(encoded_path) => {
                                println!(
                                    "{} {}",
                                    "File encoded successfully! Output:".green(),
                                    encoded_path
                                );
                            }
                            Err(e) => println!("{} {}", "Error encoding file:".red(), e),
                        }
                    } else {
                        println!("{}", "No output file selected.".red());
                    }
                } else {
                    println!("{}", "No input file selected.".red());
                }
                sleep(Duration::from_secs(2));
                continue;
            }

            "2" => {
                if let Some(input_path) = FileDialog::new()
                    .set_title("Select a file to decode")
                    .pick_file()
                {
                    let password = prompt_password("Enter the password: ").unwrap();

                    if password.is_empty() {
                        println!(
                            "{} The password cannot be empty",
                            "Error decoding file:".red()
                        ); // Same as above
                        sleep(Duration::from_secs(2));
                        continue;
                    }

                    let file_name = input_path.file_name().unwrap().to_str().unwrap();
                    let decoded_name = file_name.strip_suffix(".rxor").unwrap_or(file_name);

                    if let Some(output_path) = FileDialog::new()
                        .set_title("Select output file location")
                        .set_file_name(decoded_name)
                        .save_file()
                    {
                        match decode(
                            input_path.to_str().unwrap(),
                            &password,
                            Some(output_path.to_str().unwrap()),
                        ) {
                            Ok(decoded_path) => {
                                println!(
                                    "{} {}",
                                    "File decoded successfully! Output:".green(),
                                    decoded_path
                                );
                            }
                            Err(e) => println!("{} {}", "Error decoding file:".red(), e),
                        }
                    } else {
                        println!("{}", "No output file selected.".red());
                    }
                } else {
                    println!("{}", "No input file selected.".red());
                }
                sleep(Duration::from_secs(2));
                continue;
            }

            "3" => {
                clear().unwrap();
                println!("{}", "Thanks for using ReXOR! See you later!".blue());
                sleep(Duration::from_secs(1));
                exit(0);
            }

            _ => {
                println!("{}", "Invalid response".red());
                sleep(Duration::from_secs(1));
            }
        }
    }
}
