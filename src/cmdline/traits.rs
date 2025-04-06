//a Imports
use clap::{ArgMatches, Command};

//a CmdData and Subcommand
//tt CmdData
pub trait CmdData {
    type Error: std::fmt::Display;
}
//tt Subcommand
pub trait Subcommand<D: CmdData> {
    fn create_subcommand(&self) -> Command;
    fn handle(&self, data: &mut D, matches: &ArgMatches) -> Result<(), D::Error>;
}
