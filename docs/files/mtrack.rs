use chrono::{DateTime, Local};
use shellexpand;
use std::env;
use std::{fs, fs::File, io, io::Write};

const HELP_MENU: &str = "\nThis is a tracking program.\n\nUse `tracker` with no arguments or with the `-h` argument to find this menu.\n\nTo make a log, write `tracker xxx message`, and it will record it in a `~/.config/mtrack/log.txt` file (which needs to be provided).\n\n - The `xxx` can be any list of 0-3 characters to represent up or down changes in mood.\n\n - The `message` should be a message explaining the change.\n";

fn main() {
    let log_path: &str = &format!("{}{}", shellexpand::tilde("~/.config/mtrack/"), "log.txt");

    // gets the datetime and formats it
    let datetime: DateTime<Local> = Local::now();
    let formatted_datetime = datetime.format("%d%m%y:%H%M%S");

    let separator = " │ ";
    let separator = separator.as_bytes();
    // makes a single byte vec to add to the file at the beginning of each entry
    let date_and_bar = [formatted_datetime.to_string().as_bytes(), &separator].concat();

    // the entry starts with the datetime and a separator
    let mut entry = date_and_bar;
    let mut index = 0;
    let mut valid = true;

    // prints help menu if no arguments are given
    if env::args().len() == 1 {
        println!("{}", HELP_MENU);
    } else {
        for argument in env::args() {
            // the zeroth argument will always be the tracker's path, so it just ignores it
            if index == 0 {
                index += 1;
                continue;
            } else if index == 1 {
                // prints the help menu if the first argument is `-h`
                if argument.trim() == "-h" {
                    println!("{}", HELP_MENU);
                    valid = false;
                }
            }
            // checks if argument can be parsed to a String
            let _i = match argument.parse::<String>() {
                Ok(_i) => {
                    // adds the argument to the entry
                    entry.extend(_i.as_bytes().to_vec());
                    // if its the first argument, it also formats it to be 3 characters long, filling in with spaces, and sets valid to false if its more than 3 characters
                    if index == 1 {
                        index += 1;
                        let entry_length = _i.as_bytes().len();
                        if entry_length <= 3 {
                            for _j in 0..3 - entry_length {
                                entry.extend(" ".as_bytes().to_vec());
                            }
                        } else {
                            println!("too many characters in the second argument!");
                            valid = false;
                        }
                        // it also add a separator between the mood change section and the log text
                        entry.extend(" │".as_bytes().to_vec());
                    }
                }
                // just for safety
                Err(_e) => println!("{}: Not a valid String!", argument),
            };
            // this is to add spaces between subsequent arguments (allows to pass log text without "")
            entry.extend(" ".as_bytes().to_vec());
        }
        // if the entry is still valid (mood is under 3 characters and it wasn't `-h`, it adds a newline to the entry and prepends it to the log file)
        if valid {
            entry.extend("\n".as_bytes().to_vec());
            prepend_file(&entry, log_path);
            println!("logged")
        }
    }
}

// prepends a slice to a file
fn prepend_file(data: &[u8], file_path: &str) {
    let temp_path: &str = &format!("{}{}", shellexpand::tilde("~/.config/mtrack/"), "temp.txt");

    let mut tmp = File::create(temp_path).expect("could not create temp file");
    let mut src = File::open(&file_path).expect("could not open given file");
    tmp.write_all(&data)
        .expect("could not write data to temp file");
    io::copy(&mut src, &mut tmp).expect("could not copy source to temp");
    fs::remove_file(&file_path).expect("could not remove file");
    fs::rename(temp_path, &file_path).expect("could not rename temp file");
}
