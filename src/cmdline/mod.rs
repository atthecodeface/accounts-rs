//a Imports
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{Database, Error, FileFormat, FileType};

pub trait CmdData {
    type Error: std::fmt::Display;
}

pub trait CmdHandler<D: CmdData> {
    fn handle(&self, cmd_data: &mut D, matches: &ArgMatches) -> Result<(), <D as CmdData>::Error>;
}

pub struct SubcommandSet<D: CmdData> {
    cmd: Command,
    cmd_interactive: Option<Command>,
    sub_cmds: Vec<Subcommand<D>>,
    matches: Option<ArgMatches>,
}

impl<D: CmdData> std::ops::Deref for SubcommandSet<D> {
    type Target = Command;
    fn deref(&self) -> &Command {
        &self.cmd
    }
}

impl<D: CmdData> std::ops::DerefMut for SubcommandSet<D> {
    fn deref_mut(&mut self) -> &mut Command {
        &mut self.cmd
    }
}

impl<D: CmdData> SubcommandSet<D> {
    pub fn new(cmd: Command) -> Self {
        Self {
            cmd,
            cmd_interactive: None,
            sub_cmds: vec![],
            matches: None,
        }
    }

    pub fn map_cmd<F: FnOnce(Command) -> Command>(&mut self, f: F) {
        self.cmd = f(std::mem::take(&mut self.cmd));
    }

    pub fn get_matches(&mut self) -> &ArgMatches {
        self.matches = Some(self.cmd.get_matches_mut());
        self.matches.as_ref().unwrap()
    }

    pub fn make_interactive(&mut self) {
        self.cmd_interactive = Some(
            self.cmd
                .clone()
                .no_binary_name(true)
                .subcommand_required(true),
        );
        self.map_cmd(|c| {
            c.arg(
                Arg::new("interactive")
                    .short('i')
                    .long("interactive")
                    .action(ArgAction::SetTrue)
                    .help("Run interactively after executing any command line arguments"),
            )
        });
    }

    pub fn new_subcommand<F: Fn(&mut D, &ArgMatches) -> Result<(), D::Error> + 'static>(
        &mut self,
        cmd: Command,
        handler: F,
    ) {
        let name = cmd.get_name().into();
        let handler = Box::new(handler);
        let sub_cmd = Subcommand { name, handler };
        self.sub_cmds.push(sub_cmd);
        self.map_cmd(move |c| c.subcommand(cmd));
    }

    pub fn handle_subcommand_matches(&self, data: &mut D) -> Result<(), D::Error> {
        let matches = self.matches.as_ref().unwrap();
        if let Some((name, submatches)) = matches.subcommand() {
            for s in self.sub_cmds.iter() {
                if name == s.name {
                    return (s.handler)(data, submatches);
                }
            }
        }
        Ok(())
    }

    pub fn handle_matches(&mut self, mut data: D) {
        if let Err(e) = self.handle_subcommand_matches(&mut data) {
            eprintln!("database : error: {e}");
            std::process::exit(4);
        }

        let interactive = self
            .matches
            .as_ref()
            .unwrap()
            .get_one::<bool>("interactive")
            .unwrap();
        let interactive = *interactive;
        while interactive {
            use std::io::Write;
            let mut line = String::new();
            print!("database> ");
            std::io::stdout().flush().unwrap();
            let Ok(n) = std::io::stdin().read_line(&mut line) else {
                break;
            };
            if n == 0 {
                break;
            }
            let split = line.trim().split(' ');
            match self
                .cmd_interactive
                .as_mut()
                .unwrap()
                .try_get_matches_from_mut(split)
            {
                Err(e) => {
                    let _ = e.print();
                }
                Ok(matches) => {
                    self.matches = Some(matches);
                    match self.handle_subcommand_matches(&mut data) {
                        Err(e) => {
                            eprintln!("Error: {e}");
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

pub struct Subcommand<D: CmdData> {
    name: String,
    handler: Box<dyn Fn(&mut D, &ArgMatches) -> Result<(), D::Error> + 'static>,
}

//ip Debug for Subcommand
impl<D> std::fmt::Debug for Subcommand<D>
where
    D: CmdData,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.name.fmt(fmt)
    }
}

impl CmdData for Database {
    type Error = Error;
}

//    if let Some(lloyds_csv) = matches.get_one::<String>("lloyds_csv") {
//        let s = std::fs::read_to_string(lloyds_csv)?;
//        let csv = lloyds::read_transactions_csv(s.as_bytes())?;
//        eprintln!("csv : {csv:?}");
//    }

pub mod database;
pub mod write;
