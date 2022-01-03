#![allow(unused)]
use log::info;

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

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

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
}

fn mimic_intensive_task() {
    for i in 0..1000 {}
}
