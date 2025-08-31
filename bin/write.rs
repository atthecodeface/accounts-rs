//a Imports
use clap::Command;
use thunderclap::json;
use thunderclap::{CommandArgs, CommandBuilder};

use crate::CmdArgs;
use rust_accounts::Error;

//a Write
fn write_fn(cmd_args: &mut CmdArgs) -> Result<json::Value, Error> {
    cmd_args.write_database()?;
    CmdArgs::cmd_ok()
}

//a write command
//fp write_cmd
pub fn write_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("write").about("Write out the database");

    let mut build = CommandBuilder::with_handler(command, write_fn);
    CmdArgs::arg_add_file_format(&mut build);
    CmdArgs::arg_add_write_filename(&mut build);
    build
}
