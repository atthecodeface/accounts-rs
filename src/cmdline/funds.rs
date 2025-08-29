//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use crate::cmdline::CmdArgs;
use crate::{Error, Fund};

//a Funds
//fi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    println!("Funds:");
    for k in cmd_args.db.funds().db_ids() {
        let fund = cmd_args.db.get(k).unwrap().fund().unwrap();
        let fund = fund.borrow();
        println!("  {k} : {} - {}", fund.name(), fund.desc());
        for d in fund.aliases() {
            println!("      {d}");
        }
    }
    Ok("".into())
}

//fi add_fn
fn add_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let description = &cmd_args.string_args[1];

    let fund = Fund::new(name, description);
    let db_id = cmd_args.db.add_fund(fund);
    Ok(format!("DbId{db_id}"))
}

//fi add_alias_fn
fn add_alias_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let clear = cmd_args.clear;

    let db_m = cmd_args.get_fund(name)?; // related_party(name, RelatedPartyQuery::Rp(RelatedPartyType::Fund))?;
    cmd_args.db.funds().remove_fund_aliases(&db_m);
    if clear {
        db_m.inner_mut().clear_aliases();
    }
    for i in 1..cmd_args.string_args.len() {
        db_m.inner_mut().add_alias(&cmd_args.string_args[i]);
    }
    cmd_args.db.funds().add_fund_aliases(&db_m);
    Ok("".into())
}

//mi list_cmd
fn list_cmd() -> CommandBuilder<CmdArgs> {
    CommandBuilder::with_handler(Command::new("list").about("List all the funds"), list_fn)
}

//mi add_cmd
fn add_cmd() -> CommandBuilder<CmdArgs> {
    let mut add = CommandBuilder::with_handler(Command::new("add").about("Add a fund"), add_fn);
    CmdArgs::arg_add_positional_string(&mut add, "name", "Fund name", Some(1), None);
    CmdArgs::arg_add_positional_string(&mut add, "description", "Description", Some(1), None);
    add
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

//mp funds_cmd
pub fn funds_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("funds").about("Operate on the funds section of the database");

    let mut build = CommandBuilder::new(command);

    build.add_subcommand(list_cmd());
    build.add_subcommand(add_cmd());
    build.add_subcommand(add_alias_cmd());

    build
}
