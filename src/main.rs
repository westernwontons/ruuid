mod cli;
mod handle_command;
mod uuid;

use clap::Parser;
use cli::CommandLineArguments;
use handle_command::handle_command;

fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    anyhow::Ok(handle_command(args.command))
}
