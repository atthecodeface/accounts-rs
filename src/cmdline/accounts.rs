//a Imports
use clap::{Arg, ArgMatches, Command};

use crate::cmdline::Subcommand;
use crate::{Database, Error};

//a Accounts
#[derive(Default)]
pub struct Accounts();

//ip Subcommand for Accounts
impl Subcommand<Database> for Accounts {
    //mp create_subcommand
    fn create_subcommand(&self) -> Command {
        Command::new("accounts").about("Perform operations on accounts")
    }

    //mp handle
    fn handle(&self, db: &mut Database, matches: &ArgMatches) -> Result<(), Error> {
        let write_db = matches.get_one::<String>("write_database").unwrap();
        eprintln!("write_db : {write_db}");
        let w = vec![];
        let mut s = serde_json::Serializer::pretty(w);
        db.serialize_as_array(&mut s)?;
        let w = s.into_inner();
        let s = std::str::from_utf8(&w).unwrap();
        eprintln!("{s}");
        Ok(())
    }
}
