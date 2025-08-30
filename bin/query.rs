//a Imports
use clap::Command;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::{Account, AccountDesc, Date, DbQuery, Error};

//a Query
//mi query_fn
fn query_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
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

    let db_query = cmd_args.db.query(query);
    for x in db_query {
        eprintln!("{} : {}", x, cmd_args.db.get(x).unwrap());
    }

    CmdArgs::cmd_ok()
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

    cmd
}
