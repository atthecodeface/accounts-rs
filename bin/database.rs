//a Imports
use clap::Command;
use thunderclap::json;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::{Account, AccountDesc, Date, DbQuery, Error};

//a Query
//mi query_fn
fn query_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    let mut query = DbQuery::default();

    if cmd_args.name.is_some() {
        query = query.with_name(cmd_args.name.as_ref().unwrap());
    }
    if cmd_args.item_type.is_some() {
        query = query.with_item_type(cmd_args.item_type);
    }
    if cmd_args.rp_type.is_some() {
        query = query.with_rp_type(cmd_args.rp_type);
    }
    if cmd_args.id.is_some() {
        query = query.with_id(cmd_args.id);
    }
    if cmd_args.desc.is_some() {
        query = query.with_desc(cmd_args.desc.as_ref().unwrap());
    }
    query = query.with_date_range(cmd_args.get_date_range());

    println!("{query}");
    let db_query: Vec<_> = cmd_args.db.query(query).collect();
    // for x in db_query.iter() {
    // println!("{} : {}", x, cmd_args.db.get(*x).unwrap());
    // }

    Ok(json::to_value(db_query).unwrap())
}

//mp query_cmd
pub fn query_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd =
        CommandBuilder::with_handler(Command::new("query").about("Query the database"), query_fn);

    CmdArgs::arg_add_option_search_name(&mut cmd);
    CmdArgs::arg_add_option_search_id(&mut cmd);
    CmdArgs::arg_add_option_search_desc(&mut cmd);
    CmdArgs::arg_add_option_rp_type(&mut cmd, false);
    CmdArgs::arg_add_option_item_type(&mut cmd, false);
    CmdArgs::arg_add_option_start_date(&mut cmd);
    CmdArgs::arg_add_option_end_date(&mut cmd);

    cmd
}

//mi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    eprintln!("{:?}", cmd_args.value_args[0]);
    let x: &json::Value = &*cmd_args.value_args[0];
    let dbs: Vec<rust_accounts::DbId> = json::from_value(x.clone())?;

    for x in dbs.iter() {
        println!("{} : {}", x, cmd_args.db.get(*x).unwrap());
    }
    CmdArgs::cmd_ok()
}

//mp list_cmd
pub fn list_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("list").about("List entries in the database"),
        list_fn,
    );

    CmdArgs::arg_add_positional_value(&mut cmd, "tag", "help", Some(1), None);

    cmd
}

//mp database_cmd
pub fn database_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd =
        CommandBuilder::new(Command::new("database").about("Operate on the whole database"));

    cmd.add_subcommand(query_cmd());
    cmd.add_subcommand(list_cmd());

    cmd
}
