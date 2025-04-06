//a Imports
use clap::{Arg, ArgAction, Command};

//fp add_subcommand
pub fn add_subcommand(cmd: Command) -> Command {
    let write_subcmd = Command::new("write")
        .about("Write out the database")
        .arg(Arg::new("write_database").help("Write the database out to a file"))
        .arg(
            Arg::new("output_format")
                .long("ofmt")
                .help("Format of the database to write"),
        );
    cmd.subcommand(write_subcmd)
}
