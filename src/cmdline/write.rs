//a Imports
use crate::cmdline::CmdArgs;
use crate::Error;
use clap::Command;
use thunderclap::CommandBuilder;

//a Write
fn write_fn(cmd_args: &mut CmdArgs) -> Result<String, Error> {
    let mut w = vec![];
    let mut s = serde_yaml::Serializer::new(&mut w);
    cmd_args.db.serialize_as_array(&mut s)?;
    let s = std::str::from_utf8(&w).unwrap();

    // let w = vec![];
    // let mut s = serde_json::Serializer::pretty(w);
    // db.serialize_as_array(&mut s)?;
    // let w = s.into_inner();
    // let s = std::str::from_utf8(&w).unwrap();
    eprintln!("{s}");
    Ok("".into())
}

//a write command
//fp write_cmd
pub fn write_cmd() -> CommandBuilder<CmdArgs> {
    let command = Command::new("write").about("Write out the database");

    let mut build = CommandBuilder::with_handler(command, write_fn);
    CmdArgs::arg_add_positional_string(
        &mut build,
        "output_filename",
        "File to write the database out to",
        Some(1), // Required, single positional argument
        None,
    );
    CmdArgs::arg_add_write_format(&mut build);
    build
}
