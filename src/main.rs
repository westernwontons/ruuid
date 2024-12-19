mod cli;
mod handle_command;
mod uuid;
mod ulid;

use clap::Parser;
use cli::CommandLineArguments;
use handle_command::handle_command;

fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    handle_command(args.command);
    anyhow::Ok(())
}
