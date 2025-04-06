//a Imports
use clap::Command;

use rust_accounts::*;

pub fn main() {
    let cmd = Command::new("database")
        .about("Accounts database tool")
        .version("0.1.0");

    let mut subcmds = cmdline::CommandSet::new(cmd);
    subcmds.new_subcommand(cmdline::Write::default());
    subcmds.make_interactive();

    subcmds.map_cmd(cmdline::database::add_args);

    let matches = subcmds.get_matches();

    let result = cmdline::database::new(matches);
    if let Err(e) = result {
        eprintln!("database : error: {e}");
        std::process::exit(4);
    }
    let db = result.unwrap();
    subcmds.handle_matches(db);
}
