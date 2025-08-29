//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use crate::cmdline::CmdArgs;
use crate::{Date, Error};

//a Accounts
//a Write
//fi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    println!("Accounts:");
    for k in cmd_args.db.accounts().ids() {
        let account = cmd_args.db.get(k).unwrap().account().unwrap();
        let account = account.borrow();
        println!("  {k} : {} - {}", account.org(), account.name());
    }
    Ok("".into())
}

//mi add_fn
fn add_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let bank = &cmd_args.string_args[0];
    let name = &cmd_args.string_args[1];
    let sort_code = &cmd_args.string_args[2];
    let account_number = cmd_args.usize_args[0];

    let desc = crate::AccountDesc::parse_uk(sort_code, account_number)?;
    let account = crate::Account::new(bank.to_owned(), name.to_owned(), desc);

    let db_id = cmd_args.db.add_account(account);
    Ok(format!("DbId{db_id}"))
}

//mi validate_fn
fn validate_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let db_acc = cmd_args.get_account(name)?;

    let errors = db_acc.inner().validate_transactions(&cmd_args.db);
    if !errors.is_empty() {
        for (db_id, e) in errors.into_iter() {
            eprintln!("{db_id} {e}");
        }
    }
    Ok("".into())
}

//mi transactions_fn
fn transactions_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let (start, end) = cmd_args
        .get_date_range()
        .unwrap_or((Date::of_dmy(1, 1, 2020), Date::of_dmy(1, 1, 2099)));
    let db_acc = cmd_args.get_account(name)?;

    let transactions = db_acc.inner().transactions_between_dates(start, end);
    for db_id in transactions.into_iter() {
        let bt = cmd_args.db.get(db_id).unwrap().bank_transaction().unwrap();
        let bt = bt.inner();
        let date = bt.date();
        let desc = bt.description();
        let balance_delta = bt.balance_delta();
        let end_balance = bt.balance();
        let start_balance = end_balance - balance_delta;
        println!("{date} {desc:100} {start_balance:12} {balance_delta:12} {end_balance:12}");
    }
    Ok("".into())
}

//mi validate_cmd
fn validate_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("validate").about("Validate the account transactions"),
        validate_fn,
    );
    CmdArgs::arg_add_account_positional(&mut cmd);
    cmd
}

//mi transactions_cmd
fn transactions_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("transactions").about("Show account transactions"),
        transactions_fn,
    );

    CmdArgs::arg_add_account_positional(&mut cmd);
    CmdArgs::arg_add_option_start_date(&mut cmd);
    CmdArgs::arg_add_option_end_date(&mut cmd);
    cmd
}

//mp accounts_cmd
pub fn accounts_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("accounts").about("Operate on the accounts section of the database");

    let mut build = CommandBuilder::new(command);
    let list =
        CommandBuilder::with_handler(Command::new("list").about("List all the accounts"), list_fn);
    let mut add = CommandBuilder::with_handler(Command::new("add").about("Add an account"), add_fn);
    CmdArgs::arg_add_option_string(&mut add, "bank", None, "Bank name", None);
    CmdArgs::arg_add_option_string(&mut add, "name", None, "Account name", None);
    CmdArgs::arg_add_option_string(&mut add, "sort_code", None, "Sort code (xx-xx-xx)", None);
    CmdArgs::arg_add_option_usize(
        &mut add,
        "account_number",
        None,
        "Account number - a positive integer",
        None,
    );

    build.add_subcommand(list);
    build.add_subcommand(add);
    build.add_subcommand(validate_cmd());
    build.add_subcommand(transactions_cmd());

    build
}
