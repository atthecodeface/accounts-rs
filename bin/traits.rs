//a Imports
use clap::{ArgMatches, Command};

//a CmdData and Subcommand
//tt CmdData
pub trait CmdData {
    type Error: std::fmt::Display;
}
//tt Subcommand
pub trait Subcommand<D: CmdData> {
    fn create_subcommand(&mut self) -> Command;
    fn handle(&mut self, data: &mut D, matches: &ArgMatches) -> Result<(), D::Error>;
}
