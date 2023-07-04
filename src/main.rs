use std::path;
use std::fs::File;
use std::io::BufReader;
use rss::Channel;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path : path::PathBuf = args.path;
    let rss_file = File::open(&path).expect("File not found!!");
    let channel = Channel::read_from(BufReader::new(rss_file)).unwrap();
    

    println!("File Path: {}", &path.display());
    println!("Channel: {}", &channel.to_string());
}
