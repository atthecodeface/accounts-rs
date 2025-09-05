//a Imports
use clap::Command;
pub use thunderclap::json;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::{Date, DbId, Error, Invoice};

//a Invoices
//fi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    println!("Invoices:");

    let invoices = cmd_args.db.invoices().ids();
    for k in &invoices {
        let invoice = cmd_args.db.get(*k).unwrap().invoice().unwrap();
        let invoice = invoice.borrow();
        println!("  {k} : {} - {}", invoice.reason(), invoice.filename());
    }
    let summaries: Vec<DbId> = invoices.iter().copied().collect();
    Ok(json::to_value(summaries).unwrap())
}

//mi add_cmd
fn add_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(Command::new("add").about("Add an invoice"), add_fn);
    CmdArgs::arg_add_option_string(
        &mut cmd,
        "reason",
        None,
        "Reason for the invoice (unique in the invoices)",
        None,
    );
    CmdArgs::arg_add_option_string(&mut cmd, "filename", None, "Filename of the invoice", None);
    CmdArgs::arg_add_option_rp_id(&mut cmd, true);
    CmdArgs::arg_add_option_amount(&mut cmd, true);
    cmd
}

//mi add_fn
fn add_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let reason = &cmd_args.string_args[0];
    let filename = &cmd_args.string_args[1];
    let supplier_id = DbId::of_usize(cmd_args.rp_id.unwrap());
    let amount = cmd_args.amount.unwrap();
    let invoice = Invoice::new(
        supplier_id,
        reason.to_string(),
        filename.to_string(),
        amount,
    );
    let db_id = cmd_args.db.add_invoice(invoice);
    Ok(json::to_value(db_id).unwrap())
}

//mi validate_fn
fn validate_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    CmdArgs::cmd_ok()
}

//mi validate_cmd
fn validate_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("validate").about("Validate the invoice transactions"),
        validate_fn,
    );
    // CmdArgs::arg_add_invoice_positional(&mut cmd);
    cmd
}

//mp invoices_cmd
pub fn invoices_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("invoices").about("Operate on the invoices section of the database");

    let mut build = CommandBuilder::new(command);
    let list =
        CommandBuilder::with_handler(Command::new("list").about("List all the invoices"), list_fn);

    build.add_subcommand(add_cmd());
    build.add_subcommand(list);
    build.add_subcommand(validate_cmd());

    build
}
