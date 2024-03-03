use {cl_chess::args::Args, clap::Parser};

fn main() {
    let args = Args::parse();

    cl_chess::run(args).unwrap_or_else(|err| {
        println!("Unexpected error: {}", err);
    });
}
