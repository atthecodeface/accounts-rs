//a Imports
use clap::Command;
use thunderclap::json;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::{banks, Error};

//a Members
fn lloyds_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let filename = &cmd_args.string_args[0];

    println!("Attempt to import Lloyds CSV from file '{filename}'");
    let csv_data = std::fs::read_to_string(filename)?;
    let acc_transactions = banks::lloyds::read_transactions_csv(csv_data.as_bytes())?;

    if acc_transactions.is_empty() {
        return Err("Transactions were empty".to_string().into());
    }
    let Some(account) = cmd_args
        .db
        .accounts()
        .get_account(acc_transactions[0].account_desc())
    else {
        return Err(format!(
            "Account {} was not known",
            acc_transactions[0].account_desc()
        )
        .into());
    };

    if let Err(unresolved_transactions) =
        account
            .inner_mut()
            .add_transactions(&cmd_args.db, account.id(), acc_transactions)
    {
        eprintln!("Failed to add transactions {unresolved_transactions:?}");
        return Err(format!(
            "Failed to add {} transactions",
            unresolved_transactions.len()
        )
        .into());
    }
    CmdArgs::cmd_ok()
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
