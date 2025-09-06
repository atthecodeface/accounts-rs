//a Imports
use clap::Command;
use thunderclap::json;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::{Error, Fund};

//a Funds
//mi list_cmd
fn list_cmd() -> CommandBuilder<CmdArgs> {
    CommandBuilder::with_handler(Command::new("list").about("List all the funds"), list_fn)
}

//fi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    println!("Funds:");
    let funds = cmd_args.db.funds().db_ids();
    for k in funds.iter() {
        let fund = cmd_args.db.get(*k).unwrap().fund().unwrap();
        let fund = fund.borrow();
        println!("  {k} : {} - {}", fund.name(), fund.desc());
        for d in fund.aliases() {
            println!("      {d}");
        }
    }
    Ok(json::to_value(funds).unwrap())
}

//mi show_cmd
fn show_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("show").about("Show details of a fund"),
        show_fn,
    );
    CmdArgs::arg_add_fund_positional(&mut cmd);
    cmd
}

//fi show_fn
fn show_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let name = cmd_args.next_string_arg()?;
    let db_f = cmd_args.get_fund(&name)?;
    db_f.inner().show(&cmd_args.db, db_f.id());

    CmdArgs::cmd_ok()
}

//mi add_cmd
fn add_cmd() -> CommandBuilder<CmdArgs> {
    let mut add = CommandBuilder::with_handler(Command::new("add").about("Add a fund"), add_fn);
    CmdArgs::arg_add_positional_string(&mut add, "name", "Fund name", Some(1), None);
    CmdArgs::arg_add_positional_string(&mut add, "description", "Description", Some(1), None);
    add
}

//fi add_fn
fn add_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let name = cmd_args.next_string_arg()?;
    let description = cmd_args.next_string_arg()?;

    let fund = Fund::new(&name, &description);
    let db_id = cmd_args.db.add_fund(fund);
    Ok(json::to_value(db_id).unwrap())
}

//mi add_alias_cmd
fn add_alias_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("add_alias").about("Add alias(es) for a fund"),
        add_alias_fn,
    );
    CmdArgs::arg_add_clear(&mut cmd);
    CmdArgs::arg_add_fund_positional(&mut cmd);
    CmdArgs::arg_add_positional_string(&mut cmd, "alias", "Alias to add", None, None);
    cmd
}

//fi add_alias_fn
fn add_alias_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let name = cmd_args.next_string_arg()?;
    let aliases: Vec<_> = cmd_args.remaining_string_args().collect();
    let clear = cmd_args.clear;

    let db_m = cmd_args.get_fund(&name)?;
    let db_id = db_m.id();
    cmd_args.db.funds().remove_fund_aliases(&db_m);
    if clear {
        db_m.inner_mut().clear_aliases();
    }
    for a in aliases.iter() {
        db_m.inner_mut().add_alias(a);
    }
    cmd_args.db.funds().add_fund_aliases(&db_m);
    Ok(json::to_value(db_id).unwrap())
}

//mp funds_cmd
pub fn funds_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("funds").about("Operate on the funds section of the database");

    let mut build = CommandBuilder::new(command);

    build.add_subcommand(list_cmd());
    build.add_subcommand(show_cmd());
    build.add_subcommand(add_cmd());
    build.add_subcommand(add_alias_cmd());

    build
}
