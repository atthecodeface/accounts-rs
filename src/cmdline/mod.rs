//a Imports
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{Database, Error, FileFormat, FileType};

mod traits;
pub use traits::CmdData;

mod subcommand;
pub use subcommand::SubcommandSet;

impl CmdData for Database {
    type Error = Error;
}

//    if let Some(lloyds_csv) = matches.get_one::<String>("lloyds_csv") {
//        let s = std::fs::read_to_string(lloyds_csv)?;
//        let csv = lloyds::read_transactions_csv(s.as_bytes())?;
//        eprintln!("csv : {csv:?}");
//    }

pub mod database;
pub mod write;
