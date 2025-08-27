//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use crate::banks;
use crate::cmdline::{CmdArgs, Subcommand, SubcommandSet};
use crate::{Database, Error, Member};

//a Members
fn lloyds_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let filename = &cmd_args.string_args[0];

    println!("Attempt to import Lloyds CSV from file '{}'", filename);
    let csv_data = std::fs::read_to_string(filename)?;
    let acc_transactions = banks::lloyds::read_transactions_csv(csv_data.as_bytes())?;

    for a in acc_transactions {
        eprintln!("{a:?}");
    }

    Ok("".into())
}

pub fn banks_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("banks").about("Import data from banks");

    let mut build = CommandBuilder::new(command);
    let mut lloyds =
        CommandBuilder::with_handler(Command::new("lloyds_csv").about("Used Lloyds"), lloyds_fn);
    CmdArgs::arg_add_positional_string(
        &mut lloyds,
        "csv_filename",
        "CSV filename of transactions to import",
        Some(1),
        None,
    );

    build.add_subcommand(lloyds);

    build
}
