//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use crate::cmdline::CmdArgs;
use crate::Error;

//a Accounts
//a Write
fn list_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    println!("Accounts:");
    for k in cmd_args.db.accounts().ids() {
        let account = cmd_args.db.get(k).unwrap().account().unwrap();
        let account = account.borrow();
        println!("  {k} : {} - {}", account.org(), account.name());
    }
    Ok("".into())
}

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

    build
}
