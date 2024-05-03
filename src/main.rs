use {clap::Parser, termchess::args::Args};

fn main() {
    let args = Args::parse();

    termchess::run(args).unwrap_or_else(|err| {
        println!("Unexpected error: {}", err);
    });
}
