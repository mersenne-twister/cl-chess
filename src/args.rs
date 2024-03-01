use {clap::Parser, std::io};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

/// Command line chess, with artificial intelligence
pub struct Args {}
