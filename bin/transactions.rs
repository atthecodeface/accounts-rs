//a Imports
use clap::Command;
pub use thunderclap::json;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::{Date, DbId, Error, Transaction};

//a Transactions
//mi add_payment_cmd
/// This will be a ToRp, as part of a BankTransaction, for a fund for an amount, with optional notes, on a date
fn add_payment_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("add_payment").about("Add a payment for an expense or invoice"),
        add_payment_fn,
    );
    CmdArgs::arg_add_option_rp_id(&mut cmd, true);
    CmdArgs::arg_add_option_amount(&mut cmd, true);
    cmd
}

//mi add_payment_fn
fn add_payment_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    /*
        let mut transaction = Transaction::new_payment(
            date, amount, from_fund_id, to_id);
        for n in notes {
            transaction.add_note(n);
    }
         */
    let transaction = Transaction::default();
    // let db_id = cmd_args.db.add_transaction(transaction);
    let db_id = DbId::default();
    Ok(json::to_value(db_id).unwrap())
}

//mp transactions_cmd
pub fn transactions_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("transactions").about("Transactions in the database");

    let mut build = CommandBuilder::new(command);
    build.add_subcommand(add_payment_cmd());

    build
}
