use clap::{Parser};
use std::time::Instant;


#[derive(Parser)]
struct Args {
    filename: String,

    outfile: String,

    #[clap(short, long)]
    platform: String,
}

fn main() {
    let start = Instant::now();
    let args = Args::parse();

    hammer::bundle(&args.filename, &args.outfile, &args.platform);
    let duration = start.elapsed();
    println!("Done in {}ms", duration.as_millis());
}
