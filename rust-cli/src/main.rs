#![allow(unused)]
use log::info;
use std::{thread, time::Duration};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

use indicatif::ProgressBar;

fn main() {
    env_logger::init();
    info!("starting up");

    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    let result = std::fs::read_to_string("src/test.txt");
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    };
    println!("file content: {}", content);

    let pb = ProgressBar::new(100);
    for i in 0..100 {
        mimic_intensive_task();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    thread::sleep(Duration::from_secs(5));
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

fn mimic_intensive_task() {
    for i in 0..1000 {}
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
