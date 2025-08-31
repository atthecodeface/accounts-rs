//a Imports
use clap::Command;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::{Account, AccountDesc, Date, DbTransactions, Error};

//a Transactions
//mi transactions_fn
fn transactions_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let mut transactions = DbTransactions::default();

    if cmd_args.name.is_some() {
        transactions = transactions.with_name(cmd_args.name.as_ref().unwrap());
    }
    if cmd_args.item_type.is_some() {
        transactions = transactions.with_item_type(cmd_args.item_type);
    }
    if cmd_args.rp_type.is_some() {
        transactions = transactions.with_rp_type(cmd_args.rp_type);
    }
    if cmd_args.id.is_some() {
        transactions = transactions.with_id(cmd_args.id);
    }
    if cmd_args.desc.is_some() {
        transactions = transactions.with_desc(cmd_args.desc.as_ref().unwrap());
    }

    let db_transactions = cmd_args.db.transactions(transactions);
    for x in db_transactions {
        eprintln!("{} : {}", x, cmd_args.db.get(x).unwrap());
    }

    CmdArgs::cmd_ok()
}

//mp transactions_cmd
pub fn transactions_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("transactions").about("Transactions the database"),
        transactions_fn,
    );

    CmdArgs::arg_add_option_search_name(&mut cmd);
    CmdArgs::arg_add_option_search_id(&mut cmd);
    CmdArgs::arg_add_option_search_desc(&mut cmd);
    CmdArgs::arg_add_option_rp_type(&mut cmd, false);
    CmdArgs::arg_add_option_item_type(&mut cmd, false);

    cmd
}
