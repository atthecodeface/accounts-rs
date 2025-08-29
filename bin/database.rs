//a Imports
use clap::{Arg, ArgMatches, Command};

use crate::{Database, Error, FileFormat, FileType};

//fp add_args
pub fn add_args(cmd: Command) -> Command {
    cmd.arg(Arg::new("database").long("db").help("Database"))
        .arg(
            Arg::new("input_format")
                .long("ifmt")
                .default_value("dict")
                .help("Format of the database to read"),
        )
}

//fp new
pub fn new(matches: &ArgMatches) -> Result<Database, Error> {
    let ifmt = matches
        .get_one::<String>("input_format")
        .unwrap()
        .parse::<FileFormat>()?;

    let Some(read_db) = matches.get_one::<String>("database") else {
        return Ok(Database::default());
    };

    let ftype = FileType::from_filename(read_db)?;
    let s = std::fs::read_to_string(read_db)?;
    match ftype {
        FileType::Json => {
            let mut deserializer = serde_json::Deserializer::new(serde_json::de::StrRead::new(&s));
            Database::deserialize(&mut deserializer, ifmt)
        }
        _ => Err(Error::FileTypeNotSupported(ftype, "database")),
    }
}
