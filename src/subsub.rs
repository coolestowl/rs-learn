use std::{alloc::LayoutError};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct SubSub {
    #[clap(subcommand)]
    sub_cmd: Commands
}

#[derive(Subcommand)]
enum Commands {
    Cool(CoolArgv)
}

#[derive(Args)]
struct CoolArgv {
    #[clap(short = 'a', long)]
    argv_a: String,

    #[clap(short = 'b', long)]
    argv_b: String,
}

pub fn run(argv: &SubSub) -> Result<(), LayoutError> {
    match &argv.sub_cmd {
        Commands::Cool(argv) => {
            println!("arga: {}, argb: {}", argv.argv_a, argv.argv_b);
        }
    }

    Ok(())
}
