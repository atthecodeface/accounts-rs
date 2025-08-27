//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use crate::cmdline::CmdArgs;
use crate::{Error, Member};

//a Members
fn list_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    println!("Members:");
    for k in cmd_args.db.members().iter_member_id() {
        let member = cmd_args.db.members().get_member_id(k).unwrap().borrow();
        println!("  {k} : {} - {}", member.member_id(), member.name());
    }
    Ok("".into())
}

fn add_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let member_id = cmd_args.usize_args[0];

    let member = Member::new(name.into(), member_id);
    let db_id = cmd_args.db.add_member(member);
    Ok(format!("DbId{db_id}"))
}

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

    build.add_subcommand(list);
    build.add_subcommand(add);

    build
}
