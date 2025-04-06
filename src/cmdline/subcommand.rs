//a Imports
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::cmdline::CmdData;

//a Subcommand
//ti Subcommand
/// A subcommand in a [SubcommandSet]
pub trait CmdHandlerFn<D: CmdData>:
    Fn(&mut D, &ArgMatches) -> Result<(), D::Error> + 'static
{
}

impl<D: CmdData, T: Fn(&mut D, &ArgMatches) -> Result<(), D::Error> + 'static> CmdHandlerFn<D>
    for T
{
}

struct Subcommand<D: CmdData> {
    name: String,
    handler: Box<dyn CmdHandlerFn<D>>,
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

//a SubcommandSet
//tp SubcommandSet
pub struct SubcommandSet<D: CmdData> {
    cmd: Command,
    cmd_interactive: Option<Command>,
    sub_cmds: Vec<Subcommand<D>>,
    matches: Option<ArgMatches>,
}

//ip Deref for SubcommandSet
impl<D: CmdData> std::ops::Deref for SubcommandSet<D> {
    type Target = Command;
    fn deref(&self) -> &Command {
        &self.cmd
    }
}

//ip DerefMut for SubcommandSet
impl<D: CmdData> std::ops::DerefMut for SubcommandSet<D> {
    fn deref_mut(&mut self) -> &mut Command {
        &mut self.cmd
    }
}

//ip SubcommandSet
impl<D: CmdData> SubcommandSet<D> {
    //cp new
    /// Create a new set of subcommands for a [Command]
    pub fn new(cmd: Command) -> Self {
        Self {
            cmd,
            cmd_interactive: None,
            sub_cmds: vec![],
            matches: None,
        }
    }

    //mp map_cmd
    /// Apply a mapping function to the command
    pub fn map_cmd<F: FnOnce(Command) -> Command>(&mut self, f: F) {
        self.cmd = f(std::mem::take(&mut self.cmd));
    }

    //mp get_matches
    /// Get the matches from the CLI
    pub fn get_matches(&mut self) -> &ArgMatches {
        self.matches = Some(self.cmd.get_matches_mut());
        self.matches.as_ref().unwrap()
    }

    //mp make_interactive
    /// Make the current command interactive
    ///
    /// This takes the *current* command and the subcommands, and adds
    /// an interactive mode of operation
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

    //mp new_subcommand
    /// Add a new subcommand to the command set
    ///
    /// This includes the handler function which handles those
    /// invocations of the subcommand
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

    //mi handle_subcommand_matches
    /// Handle the current matches for the command given that it has
    /// (probably) a subcommand
    ///
    /// This matches the subcommands name with one from the set, and
    /// invokes the handler on the data
    fn handle_subcommand_matches(&mut self, data: &mut D) -> Result<(), D::Error> {
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

    //mp handle_matches
    /// Handle the command matches from the command line, and run
    /// interactively if requested
    ///
    /// This invokes the subcommand handler on the current matches,
    /// and then if 'interactive' is specified it enters a command
    /// line loop where each line is parsed with the interactive
    /// command parser, and those matches are handled.
    pub fn handle_matches(&mut self, mut data: D) {
        if let Err(e) = self.handle_subcommand_matches(&mut data) {
            eprintln!("database : error: {e}");
            std::process::exit(4);
        }

        if *(self
            .matches
            .as_ref()
            .unwrap()
            .get_one::<bool>("interactive")
            .unwrap())
        {
            loop {
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
                        if let Err(e) = self.handle_subcommand_matches(&mut data) {
                            eprintln!("Error: {e}");
                        }
                    }
                }
            }
        }
    }
}
