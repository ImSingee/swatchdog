use std::thread;
use std::time::Duration;
use clap::Parser;
use parse_duration::{parse as parse_duration};
use reqwest::blocking::Client;
use reqwest::Method;
use reqwest::Url;

#[derive(Parser, Debug, Clone)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    url: Url,
    #[arg(long, default_value = "GET")]
    method: Method,
    #[arg(long, default_value = "60s", value_parser = parse_duration)]
    interval: Duration,
    #[arg(long, default_value = "false")]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let client = Client::new();

    println!("swatchdog v{} started!", env!("CARGO_PKG_VERSION"));

    loop {
        let client = client.clone();

        let args = args.clone();

        thread::spawn(move || {
            if args.verbose {
                eprint!("Send request to {} ... ", args.url);
            }

            let result = client.request(args.method, args.url).send().and_then(|res| res.error_for_status());

            if let Err(err) = result {
                eprintln!("Error: {}", err)
            }

            if args.verbose {
                eprintln!("Success");
            }
        });

        thread::sleep(args.interval);
    }
}