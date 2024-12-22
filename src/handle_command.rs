use crate::cli::Command;
use crate::ulid::ulid;
use crate::uuid::{uuid_v1, uuid_v3, uuid_v4, uuid_v5, uuid_v6, uuid_v7, uuid_v8};

pub fn handle_command(cmd: Command) {
    match cmd {
        Command::V1(args) => uuid_v1(args),
        Command::V3(args) => uuid_v3(args),
        Command::V4(args) => uuid_v4(args),
        Command::V5(args) => uuid_v5(args),
        Command::V6(args) => uuid_v6(args),
        Command::V7(args) => uuid_v7(args),
        Command::V8(args) => uuid_v8(args),
        Command::Ulid => {
            let ulid = ulid();
            print!("{ulid}");
        }
        Command::Version => println!("{}", env!("CARGO_PKG_VERSION")),
    }
}
