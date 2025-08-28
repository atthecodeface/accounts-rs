//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use crate::cmdline::CmdArgs;
use crate::{Error, Member};

//a Members
//fi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    println!("Members:");
    for k in cmd_args.db.members().db_ids() {
        let member = cmd_args.db.get(k).unwrap().member().unwrap();
        let member = member.borrow();
        println!("  {k} : {} - {}", member.member_id(), member.name());
        for d in member.account_descrs() {
            println!("      {d}");
        }
    }
    Ok("".into())
}

//fi add_fn
fn add_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let member_id = cmd_args.usize_args[0];

    let member = Member::new(name.into(), member_id);
    let db_id = cmd_args.db.add_member(member);
    Ok(format!("DbId{db_id}"))
}

//fi add_account_descr_fn
fn add_account_descr_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let account_descr = &cmd_args.string_args[1];

    if let Some(db_m) = cmd_args.db.members().get_member(name) {
        let db_id = db_m.id();
        db_m.inner_mut().add_account_descr(account_descr);
        Ok(format!("DbId{db_id}"))
    } else {
        Err(format!("Did not find member '{}'", name).into())
    }
}

//mp members_cmd
pub fn members_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("members").about("Operate on the members section of the database");

    let mut build = CommandBuilder::new(command);
    let list =
        CommandBuilder::with_handler(Command::new("list").about("List all the members"), list_fn);
    let mut add = CommandBuilder::with_handler(Command::new("add").about("Add an member"), add_fn);
    CmdArgs::arg_add_option_string(&mut add, "name", None, "Member name", None);
    CmdArgs::arg_add_option_usize(
        &mut add,
        "member_number",
        None,
        "Member number - a positive integer",
        None,
    );

    let mut add_account_desc = CommandBuilder::with_handler(
        Command::new("add_account_desc").about("Add and account descriptor for a member"),
        add_account_descr_fn,
    );
    CmdArgs::arg_add_positional_string(
        &mut add_account_desc,
        "name",
        "Member identification",
        Some(1),
        None,
    );
    CmdArgs::arg_add_positional_string(
        &mut add_account_desc,
        "description",
        "Account description",
        Some(1),
        None,
    );

    build.add_subcommand(list);
    build.add_subcommand(add);
    build.add_subcommand(add_account_desc);

    build
}
