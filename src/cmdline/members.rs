//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use crate::cmdline::CmdArgs;
use crate::{Error, RelatedParty};

//a Members
//fi list_fn
fn list_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    println!("Members:");
    for k in cmd_args.db.related_parties().db_ids() {
        let member = cmd_args.db.get(k).unwrap().related_party().unwrap();
        let member = member.borrow();
        println!("  {k} : {} - {}", member.rp_id(), member.name());
        for d in member.account_descrs() {
            println!("      {d}");
        }
    }
    Ok("".into())
}

//fi add_fn
fn add_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let member_id = cmd_args.rp_id.unwrap();

    let member = RelatedParty::new(name.into(), member_id);
    let db_id = cmd_args.db.add_related_party(member);
    Ok(format!("DbId{db_id}"))
}

//fi add_alias_fn
fn add_alias_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let clear = cmd_args.clear;

    let db_m = cmd_args.get_member(name)?; // related_party(name, RelatedPartyQuery::Rp(RelatedPartyType::Member))?;
    cmd_args
        .db
        .related_parties()
        .remove_related_party_aliases(&db_m);
    if clear {
        db_m.inner_mut().clear_aliases();
    }
    for i in 1..cmd_args.string_args.len() {
        db_m.inner_mut().add_alias(&cmd_args.string_args[i]);
    }
    cmd_args
        .db
        .related_parties()
        .add_related_party_aliases(&db_m);
    Ok("".into())
}

//fi add_account_descr_fn
fn add_account_descr_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let clear = cmd_args.clear;

    let db_m = cmd_args.get_member(name)?;
    if clear {
        db_m.inner_mut().clear_account_descr();
    }
    for i in 1..cmd_args.string_args.len() {
        db_m.inner_mut().add_account_descr(&cmd_args.string_args[i]);
    }
    Ok("".into())
}

//fi change_address_fn
fn change_address_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let name = &cmd_args.string_args[0];
    let clear = cmd_args.clear;

    let db_m = cmd_args.get_member(name)?;
    if clear {
        db_m.inner_mut().clear_address_info();
    }
    if let Some(s) = cmd_args.postcode.as_ref() {
        db_m.inner_mut().change_postcode(s);
    }
    if let Some(s) = cmd_args.telephone.as_ref() {
        db_m.inner_mut().change_telephone(s);
    }
    if let Some(s) = cmd_args.email.as_ref() {
        db_m.inner_mut().change_email(s);
    }
    if let Some(s) = cmd_args.address.as_ref() {
        db_m.inner_mut().change_address(s);
    }
    if let Some(s) = cmd_args.house_number.as_ref() {
        db_m.inner_mut().change_house_number(s);
    }
    if let Some(s) = cmd_args.tax_name.as_ref() {
        db_m.inner_mut().change_tax_name(s);
    }
    Ok("".into())
}

//mi list_cmd
fn list_cmd() -> CommandBuilder<CmdArgs> {
    CommandBuilder::with_handler(Command::new("list").about("List all the members"), list_fn)
}

//mi add_cmd
fn add_cmd() -> CommandBuilder<CmdArgs> {
    let mut add = CommandBuilder::with_handler(Command::new("add").about("Add an member"), add_fn);
    CmdArgs::arg_add_option_string(&mut add, "name", None, "Member name", None);
    CmdArgs::arg_add_option_rp_id(&mut add, true);
    add
}

//mi add_alias_cmd
fn add_alias_cmd() -> CommandBuilder<CmdArgs> {
    let mut cmd = CommandBuilder::with_handler(
        Command::new("add_alias").about("Add alias(es) for a member"),
        add_alias_fn,
    );
    CmdArgs::arg_add_clear(&mut cmd);
    CmdArgs::arg_add_member_positional(&mut cmd);
    CmdArgs::arg_add_positional_string(&mut cmd, "alias", "Alias to add", None, None);
    cmd
}

//mi add_account_descr_cmd
fn add_account_descr_cmd() -> CommandBuilder<CmdArgs> {
    let mut add_account_desc = CommandBuilder::with_handler(
        Command::new("add_account_desc").about("Add and account descriptor for a member"),
        add_account_descr_fn,
    );
    CmdArgs::arg_add_clear(&mut add_account_desc);
    CmdArgs::arg_add_member_positional(&mut add_account_desc);
    CmdArgs::arg_add_positional_string(
        &mut add_account_desc,
        "description",
        "Account description",
        Some(0),
        None,
    );
    add_account_desc
}

//mi change_address_cmd
fn change_address_cmd() -> CommandBuilder<CmdArgs> {
    let mut change_address = CommandBuilder::with_handler(
        Command::new("change_address").about("Change some address info"),
        change_address_fn,
    );
    CmdArgs::arg_add_clear(&mut change_address);
    CmdArgs::arg_add_member_positional(&mut change_address);
    CmdArgs::arg_add_option_address(&mut change_address);
    CmdArgs::arg_add_option_house_number(&mut change_address);
    CmdArgs::arg_add_option_email(&mut change_address);
    CmdArgs::arg_add_option_telephone(&mut change_address);
    CmdArgs::arg_add_option_postcode(&mut change_address);
    CmdArgs::arg_add_option_tax_name(&mut change_address);
    change_address
}

//mp members_cmd
pub fn members_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("members").about("Operate on the members section of the database");

    let mut build = CommandBuilder::new(command);

    build.add_subcommand(list_cmd());
    build.add_subcommand(add_cmd());
    build.add_subcommand(add_alias_cmd());
    build.add_subcommand(add_account_descr_cmd());
    build.add_subcommand(change_address_cmd());

    build
}
