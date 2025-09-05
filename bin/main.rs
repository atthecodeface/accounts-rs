//a Imports
/// # Accounts
///
/// database
///
///  query - returns an array of DbId
///
///  list - shows a list DbID
///
///  get - get a list of database entries
///
/// accounts
///
///  add - add a new account
///
///  list - list the accounts
///
///  validate - validate an account; tracking starting/ending balance through al bank transactions
///
///  transactions -
///
/// banks
///
///  lloyds - import CSV data for a Lloyds bank account
///
/// funds
///
///  add
///
///  add_alias
///
///  list
///
/// members
///
/// query
///
/// related_parties
///
/// transactions
///
/// write
///
use clap::Command;
use thunderclap::CommandBuilder;

mod cmd_args;
pub use cmd_args::CmdArgs;

mod accounts;
mod banks;
mod database;
mod funds;
mod members;
mod related_parties;
mod write;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::new("database")
        .about("Accounts database tool")
        .version("0.1.0");

    let mut build = CommandBuilder::<CmdArgs>::new(command);

    CmdArgs::arg_add_verbose(&mut build);
    CmdArgs::arg_add_database(&mut build);

    build.add_subcommand(accounts::accounts_cmd());
    build.add_subcommand(banks::banks_cmd());
    build.add_subcommand(funds::funds_cmd());
    build.add_subcommand(members::members_cmd());
    build.add_subcommand(related_parties::related_parties_cmd());
    build.add_subcommand(write::write_cmd());
    build.add_subcommand(database::database_cmd());

    let mut cmd_args = CmdArgs::default();
    let mut command_set = thunderclap_httpd::CommandSetHttpd::build(build, true, true);
    command_set.execute_env(&mut cmd_args)?;
    // thunderclap_httpd::execute_env(&mut command, &mut cmd_args)?;
    // command.execute_env(&mut cmd_args)?;
    Ok(())
}
