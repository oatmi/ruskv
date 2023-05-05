use clap::{Parser, Subcommand};
use ruskv::{get, del, set, list};

/// Simple program to store or get a value.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// command to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
enum Commands {
    set  { key: String, val: String },
    get  { key: String },
    del  { key: String },
    list { },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::set { key, val } => {
            set(key, val);
            println!("OK")
        }
        Commands::get { key } => {
            get(key);
        }
        Commands::del { key } => {
            del(key);
            println!("OK")
        }
        Commands::list { } => {
            list();
        }
    }
}
