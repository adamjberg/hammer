use clap::{Parser};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    filename: String,

    #[clap(short, long)]
    platform: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.platform);

    hammer::bundle(&args.filename, &args.platform);
}
