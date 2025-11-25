use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args.get(0).map(|s| s.as_str()).unwrap_or("wc-like");

    if args.len() <= 1 {
        eprintln!("usage: {} [-c] [-w] [-l] <file> [file ...]", prog);
        process::exit(1);
    }

    let mut count_bytes = false;
    let mut count_words = false;
    let mut count_lines = false;
    let mut filenames: Vec<&String> = Vec::new();
    for arg in args.iter().skip(1) {
        if arg == "-c" {
            count_bytes = true;
        } 
        else if arg == "-w" {
            count_words = true;
        }
        else if arg == "-l" {
            count_lines = true;
        }
        else if arg.starts_with('-') {
            eprintln!("{}: option inconnue {}", prog, arg);
            process::exit(1);
        } 
        else {
            filenames.push(arg);
        }
    }

    if filenames.is_empty() {
        eprintln!("usage: {} [-c] [-w] [-l] <file> [file ...]", prog);
        process::exit(1);
    }
    for filename in filenames {
        if count_bytes {
            match fs::read(filename) {
                Ok(bytes) => println!("{} {}", bytes.len(), filename),
                Err(err) => eprintln!("{}: erreur lecture {}: {}", prog, filename, err),
            }
        } 
        else if count_words {
            match fs::read_to_string(filename) {
                Ok(contents) => {
                    let words_count = contents.split_whitespace().count();
                    println!("{} {}", words_count, filename);
                }
                Err(err) => eprintln!("{}: erreur lecture {}: {}", prog, filename, err),
            }
        }
        else if count_lines {
            match fs::read_to_string(filename) {
                Ok(contents) => {
                    let lines_count = contents.lines().count();
                    println!("{} {}", lines_count, filename);
                }
                Err(err) => eprintln!("{}: erreur lecture {}: {}", prog, filename, err),
            }
        }   
        else {
            match fs::read_to_string(filename) {
                Ok(contents) => {
                    let lines_count = contents.lines().count();
                    let words_count = contents.split_whitespace().count();
                    let bytes_count = contents.len();
                    println!("{} {} {} {}", lines_count, words_count, bytes_count, filename);
                }
                Err(err) => eprintln!("{}: erreur lecture {}: {}", prog, filename, err),
            }
        }
    }
}
