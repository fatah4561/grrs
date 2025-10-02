use std::{fs::File, io::{self, BufRead, BufReader, Write}, thread::sleep, time::Duration};

use clap::{Parser};
use ansi_term::Color::{Green, Blue};
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // using buff reader
    let text_file = File::open(&args.path)?;
    let reader = BufReader::new(text_file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if line.contains(&args.pattern) {
            println!("{} is on line {}", line, i)
        } 

    }

    // load all ways
    // let content = std::fs::read_to_string(&args.path).
    // with_context(|| {
    //     format!("could not read file `{}`", args.path.display())
    // })?;

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line)
    //     }
    // }

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let colored = Blue.paint("Downloading").to_string();
    let _ = writeln!(handle, "{}", colored);
    let _ = handle.flush();

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        sleep(Duration::new(0, 10));
        pb.println(Green.paint(format!("[+] finished #{}", i)).to_string());
        pb.inc(1);
    }
    pb.finish_with_message("done");

    env_logger::Builder::new().filter_level(args.verbosity.into()).init();
    info!("boom");
    warn!("aw");

    Ok(())
}
