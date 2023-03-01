use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a version 1 UUID
    V1(UuidArguments),
    /// Create a version 3 UUID
    V3(UuidArguments),
    /// Create a version 4 UUID
    V4(UuidArguments),
    /// Create a version 5 UUID
    V5(UuidArguments),
    /// Create a version 6 UUID
    V6(UuidArguments),
    /// Create a version 7 UUID
    V7(UuidArguments),
    /// Create a version 8 UUID
    V8(UuidArguments),

    /// Get the current version
    Version,
}

#[derive(Debug, Args)]
pub struct UuidArguments {
    /// The number of UUIDs you want generated
    #[arg(default_value = "1")]
    pub amount: u32,

    /// Display the UUID in the `simple` format
    #[arg(short, long, default_value = "false", conflicts_with_all = &["urn", "hyphenated", "braced"])]
    pub simple: bool,

    /// Display the UUID in the `urn` format
    #[arg(short, long, default_value = "false", conflicts_with_all = &["simple", "hyphenated", "braced"])]
    pub urn: bool,

    /// Display the UUID in the `hyphenated` format
    #[arg(short = 'p', long, default_value = "true", conflicts_with_all = &["urn", "simple", "braced"])]
    pub hyphenated: bool,

    /// Display the UUID in the `braced` format
    #[arg(short, long, default_value = "false", conflicts_with_all = &["urn", "hyphenated", "simple"])]
    pub braced: bool,
}
