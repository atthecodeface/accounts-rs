//a Imports
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::cmdline::SubcommandSet;
use crate::{Database, Error, FileFormat, FileType};

//fp subcommand
pub fn subcommand(sub_cmds: &mut SubcommandSet<Database>) {
    let subcmd = Command::new("accounts")
        .about("Operate on the accounts in the database")
    sub_cmds.new_subcommand(write_subcmd, handle_command)
}

//fp handle_command
pub fn handle_command(db: &mut Database, matches: &ArgMatches) -> Result<(), Error> {
    let write_db = matches.get_one::<String>("write_database").unwrap();
    eprintln!("write_db : {write_db}");
    let mut w = vec![];
    let mut s = serde_json::Serializer::pretty(w);
    db.serialize_as_array(&mut s)?;
    let w = s.into_inner();
    let s = std::str::from_utf8(&w).unwrap();
    eprintln!("{s}");
    Ok(())
}
