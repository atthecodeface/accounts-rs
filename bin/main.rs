//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

mod cmd_args;
pub use cmd_args::CmdArgs;

mod accounts;
mod banks;
mod funds;
mod members;
mod query;
mod related_parties;
mod write;

pub fn main() -> Result<(), rust_accounts::Error> {
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
    build.add_subcommand(query::query_cmd());

    let mut cmd_args = CmdArgs::default();
    let mut command = build.main(true, true);
    command.execute_env(&mut cmd_args)?;
    Ok(())
}
