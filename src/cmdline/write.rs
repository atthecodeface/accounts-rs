//a Imports
use clap::{Arg, ArgMatches, Command};

use crate::cmdline::{Subcommand, SubcommandSet};
use crate::{Database, Error};

//a Write
#[derive(Default)]
pub struct Write();

//ip Subcommand for Write
impl Subcommand<Database> for Write {
    //fp create_subcommand
    fn create_subcommand(&self) -> Command {
        Command::new("write")
            .about("Write out the database")
            .arg(
                Arg::new("write_database")
                    .required(true)
                    .help("Filename to write the database to"),
            )
            .arg(
                Arg::new("output_format")
                    .long("ofmt")
                    .help("Format of the database to write"),
            )
    }

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
