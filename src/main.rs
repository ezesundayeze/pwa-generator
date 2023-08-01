use clap::Parser;
mod init;
use init::{ init_pwa, Init};

/// Main command
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Eze Sunday")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Subcommands
#[derive(Parser, Debug)]
enum SubCommand {
    Init(Init),
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Init(t) => init_pwa(t),
    }
}


