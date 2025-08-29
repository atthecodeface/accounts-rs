//a Imports
use clap::Command;
use thunderclap::CommandBuilder;

use rust_accounts::cmdline::CmdArgs;

pub fn main() -> Result<(), rust_accounts::Error> {
    let command = Command::new("database")
        .about("Accounts database tool")
        .version("0.1.0");

    let mut build = CommandBuilder::<CmdArgs>::new(command);

    CmdArgs::arg_add_verbose(&mut build);
    CmdArgs::arg_add_database(&mut build);

    build.add_subcommand(rust_accounts::cmdline::accounts_cmd());
    build.add_subcommand(rust_accounts::cmdline::banks_cmd());
    build.add_subcommand(rust_accounts::cmdline::funds_cmd());
    build.add_subcommand(rust_accounts::cmdline::members_cmd());
    build.add_subcommand(rust_accounts::cmdline::write_cmd());

    let mut cmd_args = CmdArgs::default();
    let mut command = build.main(true, true);
    command.execute_env(&mut cmd_args)?;
    Ok(())
}
