use clap::{Args, Parser, Subcommand};

mod sub;
mod subsub;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add(Add),

    /// Gets files in myapp
    Get(Get),

    Sub(sub::Sub),

    Subsub(subsub::SubSub),
}

#[derive(Args)]
struct Add {
    name: Option<String>,
}

#[derive(Args)]
struct Get {
    key: Option<String>
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add(name) => {
            println!("'myapp add' was used, name is: {:?}", name.name)
        }
        Commands::Get(argv) => {
            println!("'myapp get' was used, key is: {:?}", argv.key)
        }
        Commands::Sub(argv) => {
            if let Err(e) = sub::run(argv) {
                println!("error: {:?}", e);
            }
        }
        Commands::Subsub(argv) => {
            let _ = subsub::run(argv);
        }
    }
}
