//a Imports
use clap::Command;

use rust_accounts::*;

pub fn main() {
    let cmd = Command::new("database")
        .about("Accounts database tool")
        .version("0.1.0");

    let mut subcmds = cmdline::SubcommandSet::new(cmd);
    cmdline::write::subcommand(&mut subcmds);
    subcmds.make_interactive();

    subcmds.map_cmd(|cmd| cmdline::database::add_args(cmd));

    let matches = subcmds.get_matches();

    let result = cmdline::database::new(&matches);
    if let Err(e) = result {
        eprintln!("database : error: {e}");
        std::process::exit(4);
    }
    let db = result.unwrap();
    subcmds.handle_matches(db);
}
