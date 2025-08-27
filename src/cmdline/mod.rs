//a Imports

use crate::{Database, Error, FileFormat, FileType};

mod traits;
pub use traits::{CmdData, Subcommand};

mod subcommand;
pub use subcommand::{CommandSet, SubcommandSet};

impl CmdData for Database {
    type Error = Error;
}

//    if let Some(lloyds_csv) = matches.get_one::<String>("lloyds_csv") {
//        let s = std::fs::read_to_string(lloyds_csv)?;
//        let csv = lloyds::read_transactions_csv(s.as_bytes())?;
//        eprintln!("csv : {csv:?}");
//    }

pub mod database;

mod accounts;
mod write;
pub use accounts::Accounts;
pub use write::write_cmd;

use thunderclap::{CommandArgs, CommandBuilder};

#[derive(Default)]
pub struct CmdArgs {
    pub db: Database,
    pub verbose: bool,
    pub write_format: String,
    pub write_args: Vec<String>,
}
impl CommandArgs for CmdArgs {
    type Error = Error;
    type Value = String;
    fn cmd_ok() -> Result<String, Error> {
        Ok("".into())
    }
    fn reset_args(&mut self) {
        self.write_args.clear();
    }
}

impl CmdArgs {
    //mi set_verbose
    pub(crate) fn set_verbose(&mut self, verbose: bool) -> Result<(), Error> {
        self.verbose = verbose;
        Ok(())
    }

    //mi push_string_arg
    fn push_string_arg(&mut self, s: &str) -> Result<(), Error> {
        self.write_args.push(s.into());
        Ok(())
    }

    //mi set_verbose
    fn set_write_format(&mut self, s: &str) -> Result<(), Error> {
        self.write_format = s.into();
        Ok(())
    }

    //mi load_database
    fn load_database(&mut self, filename: &str) -> Result<(), Error> {
        let ftype = FileType::from_filename(filename)?;
        let s = std::fs::read_to_string(filename)?;
        match ftype {
            FileType::Json => {
                let mut deserializer =
                    serde_json::Deserializer::new(serde_json::de::StrRead::new(&s));
                // Deserialize from Vec<DbItem>
                self.db = Database::deserialize(&mut deserializer, FileFormat::Array)?;
                Ok(())
            }
            _ => Err(Error::FileTypeNotSupported(ftype, "database")),
        }
    }

    //fp arg_add_verbose
    pub fn arg_add_verbose(build: &mut CommandBuilder<Self>) {
        build.add_flag(
            "verbose",
            Some('v'),
            "Enable verbose output",
            CmdArgs::set_verbose,
        );
    }

    //fp arg_add_database
    pub fn arg_add_database(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "db",
            Some('d'),
            "Database to read initially",
            false,
            None,
            Self::load_database,
        );
    }

    //fp arg_add_positional_string
    /// count should be None for optional; Some(0) for a single
    /// optional argument, Some(n) for a required number
    pub fn arg_add_positional_string(
        builder: &mut CommandBuilder<Self>,
        tag: &'static str,
        help: &'static str,
        count: Option<usize>,
        default_value: Option<&'static str>,
    ) {
        builder.add_arg_string(
            tag,
            None,
            help,
            (count, true),
            default_value,
            Self::push_string_arg,
        );
    }

    //fp arg_add_write_format
    pub fn arg_add_write_format(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "format",
            Some('f'),
            "Format to write",
            false,
            None,
            Self::set_write_format,
        );
    }
}
