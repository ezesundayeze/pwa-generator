use clap::Parser;
mod pwa;
use pwa::{ init_pwa, Init};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Eze Sunday")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

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


