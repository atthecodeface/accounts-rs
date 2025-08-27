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

    build.add_subcommand(rust_accounts::cmdline::write_cmd());
    build.add_subcommand(rust_accounts::cmdline::accounts_cmd());
    build.add_subcommand(rust_accounts::cmdline::members_cmd());
    build.add_subcommand(rust_accounts::cmdline::banks_cmd());

    let mut cmd_args = CmdArgs::default();
    let mut command = build.main(true, true);
    command.execute_env(&mut cmd_args)?;
    Ok(())
}

//     let mut subcmds = cmdline::CommandSet::new(cmd);
// /    subcmds.new_subcommand(cmdline::Write::default());
//     subcmds.new_subcommand(cmdline::Accounts::default());
//     subcmds.make_interactive();
//
//     subcmds.map_cmd(cmdline::database::add_args);
//
//     let matches = subcmds.get_matches();
//
//     let result = cmdline::database::new(matches);
//     if let Err(e) = result {
//         eprintln!("database : error: {e}");
//         std::process::exit(4);
//     }
//     let db = result.unwrap();
//     subcmds.handle_matches(db);
// }
