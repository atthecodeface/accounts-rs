//a Imports
use clap::{Arg, ArgMatches, Command};

use crate::cmdline::Subcommand;
use crate::{Database, Error};

//a Write
#[derive(Default)]
pub struct Write();

//ip Subcommand for Write
impl Subcommand<Database> for Write {
    //mp create_subcommand
    fn create_subcommand(&mut self) -> Command {
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

    //mp handle
    fn handle(&mut self, db: &mut Database, matches: &ArgMatches) -> Result<(), Error> {
        let write_db = matches.get_one::<String>("write_database").unwrap();
        eprintln!("write_db : {write_db}");
        let mut w = vec![];
        let mut s = serde_yaml::Serializer::new(&mut w);
        db.serialize_as_array(&mut s)?;
        let s = std::str::from_utf8(&w).unwrap();
        // let w = vec![];
        // let mut s = serde_json::Serializer::pretty(w);
        // db.serialize_as_array(&mut s)?;
        // let w = s.into_inner();
        // let s = std::str::from_utf8(&w).unwrap();
        eprintln!("{s}");
        Ok(())
    }
}
