//a Imports
use clap::Command;
pub use thunderclap::json;
use thunderclap::CommandBuilder;

use crate::CmdArgs;
use rust_accounts::{DbItemType, DbQuery, Error, Transaction};

//a Transactions
//mi list_cmd
/// List transactions that match criteria
fn list_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd =
        CommandBuilder::with_handler(Command::new("list").about("List transactions"), list_fn);

    CmdArgs::arg_add_option_start_date(&mut cmd);
    CmdArgs::arg_add_option_end_date(&mut cmd);
    cmd
}

//mi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let query = DbQuery::default()
        .with_item_type(Some(DbItemType::Transaction))
        .with_date_range(cmd_args.get_date_range());
    println!("{query}");
    let db_query: Vec<_> = cmd_args.db.query(query).collect();
    for x in db_query.iter() {
        println!("{} : {}", x, cmd_args.db.get(*x).unwrap());
    }
    Ok(json::to_value(db_query).unwrap())
}

//mi add_payment_cmd
/// This will be a ToRp, as part of a BankTransaction, for a fund for an amount, with optional notes, on a date
fn add_payment_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("add_payment").about("Add a payment for an expense or invoice"),
        add_payment_fn,
    );
    CmdArgs::arg_add_option_amount(&mut cmd, true);
    CmdArgs::arg_add_option_date(&mut cmd, true);
    CmdArgs::arg_add_fund_positional(&mut cmd);
    CmdArgs::arg_add_related_party_positional(&mut cmd);
    CmdArgs::arg_add_positional_string(&mut cmd, "notes", "Notes for the transaction", None, None);
    cmd
}

//mi add_payment_fn
fn add_payment_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let date = cmd_args.get_date()?;
    let amount = cmd_args.amount;
    let fund_name = cmd_args.next_string_arg()?;
    let from_fund = cmd_args.get_fund(&fund_name)?;
    let from_fund_id = from_fund.id();
    let to_rp = cmd_args.get_related_party_by_name(&cmd_args.string_args[1])?;
    let to_id = to_rp.id();

    let mut transaction = Transaction::new_payment(date, amount, from_fund_id, to_id);
    for n in cmd_args.string_args.iter().skip(2) {
        transaction.add_note(n);
    }
    let (db_id, okay) = cmd_args.db.add_transaction(transaction);
    if !okay {
        Err(format!("Added transaction {db_id} but database is not consistent").into())
    } else {
        Ok(json::to_value(db_id).unwrap())
    }
}

//mi add_income_cmd
/// This will be a FromRp, as part of a BankTransaction, for a fund for an amount, with optional notes, on a date
///
/// The account should have a BankTransaction associated with the related party
fn add_income_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("add_income").about("Add income from subs, ticket, donation, etc"),
        add_income_fn,
    );
    CmdArgs::arg_add_bank_transaction(&mut cmd);

    CmdArgs::arg_add_option_amount(&mut cmd, true);
    CmdArgs::arg_add_fund_positional(&mut cmd);
    CmdArgs::arg_add_positional_string(&mut cmd, "notes", "Notes for the transaction", None, None);
    cmd
}

//mi add_income_fn
fn add_income_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let amount = cmd_args.amount;
    let fund_name = cmd_args.next_string_arg()?;
    let to_fund = cmd_args.get_fund(&fund_name)?;
    let to_fund_id = to_fund.id();
    let bank_transaction = cmd_args.get_bank_transaction()?;
    let notes: Vec<_> = cmd_args.remaining_string_args().collect();

    let date = bank_transaction.inner().date();
    let from_id = bank_transaction.inner().related_party();
    let mut transaction = Transaction::new_income(date, amount, from_id, to_fund_id);
    for n in notes.iter() {
        transaction.add_note(n);
    }
    let (db_id, okay) = cmd_args.db.add_transaction(transaction);
    if !okay {
        Err(format!("Added transaction {db_id} but database is not consistent").into())
    } else {
        Ok(json::to_value(db_id).unwrap())
    }
}

//mi add_income_bt_db_cmd
/// This will be *many* income transactions, one for each db_id provided
///
/// The account should have a BankTransaction associated with the related party
fn add_income_bt_db_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("add_income_bt_db").about(
            "Add income for one or more bank transactions, from subs, ticket, donation, etc; if amount is supplied, only bank transactions of that amount are added",
        ),
        add_income_bt_db_fn,
    );
    CmdArgs::arg_add_option_db_id(&mut cmd, true);
    CmdArgs::arg_add_option_amount(&mut cmd, false);
    CmdArgs::arg_add_fund_positional(&mut cmd);
    CmdArgs::arg_add_positional_string(&mut cmd, "notes", "Notes for the transactions", None, None);
    cmd
}

//mi add_income_bt_db_fn
fn add_income_bt_db_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let amount = cmd_args.amount;
    let fund_name = cmd_args.next_string_arg()?;
    let to_fund = cmd_args.get_fund(&fund_name)?;
    let to_fund_id = to_fund.id();
    let notes: Vec<_> = cmd_args.remaining_string_args().collect();

    let mut db_ids = vec![];
    for bank_transaction in cmd_args.get_bank_transactions()? {
        db_ids.push(bank_transaction.id());
        let date = bank_transaction.inner().date();
        let from_id = bank_transaction.inner().related_party();
        let bt_amount = bank_transaction.inner().credit();
        if !amount.is_zero() && (bt_amount != amount) {
            continue;
        }
        let mut transaction = Transaction::new_income(date, bt_amount, from_id, to_fund_id);
        for n in notes.iter() {
            transaction.add_note(n);
        }
        let (db_id, okay) = cmd_args.db.add_transaction(transaction);
        if !okay {
            return Err(format!("Added transaction {db_id} but database is not consistent").into());
        }
    }
    Ok(json::to_value(db_ids).unwrap())
}

//mp transactions_cmd
pub fn transactions_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("transactions").about("Transactions in the database");

    let mut build = CommandBuilder::new(command);
    build.add_subcommand(add_payment_cmd());
    build.add_subcommand(add_income_cmd());
    build.add_subcommand(add_income_bt_db_cmd());
    build.add_subcommand(list_cmd());

    build
}
