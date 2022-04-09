use clap::{Parser};

#[derive(Parser)]
struct Args {
    filename: String,

    outfile: String,

    #[clap(short, long)]
    platform: String,
}

fn main() {
    let args = Args::parse();

    hammer::bundle(&args.filename, &args.outfile, &args.platform);
}
