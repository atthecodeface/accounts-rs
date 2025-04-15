//a Imports
use clap::{Arg, ArgAction, ArgMatches, Command};

use std::collections::HashMap;

use crate::cmdline::{CmdData, Subcommand};

//a SubcommandSet
//tp SubcommandSet
pub struct SubcommandSet<D>
where
    D: CmdData,
{
    sub_cmds: HashMap<String, Box<dyn Subcommand<D>>>,
}

//ip Default for SubcommandSet
impl<D> std::default::Default for SubcommandSet<D>
where
    D: CmdData,
{
    fn default() -> Self {
        let sub_cmds = HashMap::new();
        Self { sub_cmds }
    }
}

//ip SubcommandSet
impl<D> SubcommandSet<D>
where
    D: CmdData,
{
    //mp new_subcommand
    /// Add a new subcommand to the command set
    ///
    /// This includes the handler function which handles those
    /// invocations of the subcommand
    pub fn new_subcommand<H: Subcommand<D> + 'static>(&mut self, mut subcommand: H) -> Command {
        let cmd = subcommand.create_subcommand();
        let name = cmd.get_name().into();
        let handler = Box::new(subcommand);
        self.sub_cmds.insert(name, handler);
        cmd
    }

    //mi handle_matches
    /// Handle the current matches for the command given that it has
    /// (probably) a subcommand
    ///
    /// This matches the subcommands name with one from the set, and
    /// invokes the handler on the data
    pub fn handle_matches(&mut self, data: &mut D, matches: &ArgMatches) -> Result<(), D::Error> {
        if let Some((name, submatches)) = matches.subcommand() {
            if let Some(x) = self.sub_cmds.get_mut(name) {
                return x.handle(data, submatches);
            }
        }
        Ok(())
    }
}

//a CommandSet
//tp CommandSet
pub struct CommandSet<D: CmdData> {
    cmd: Command,
    cmd_interactive: Option<Command>,
    sub_cmds: SubcommandSet<D>,
    matches: Option<ArgMatches>,
}

//ip CommandSet
impl<D: CmdData> CommandSet<D> {
    //cp new
    /// Create a new set of subcommands for a [Command]
    pub fn new(cmd: Command) -> Self {
        let sub_cmds = SubcommandSet::default();
        Self {
            cmd,
            cmd_interactive: None,
            sub_cmds,
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
                Arg::new("batch")
                    .short('b')
                    .long("batch")
                    .action(ArgAction::Append)
                    .help("Batch file to run before interactive commands"),
            )
            .arg(
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
    pub fn new_subcommand<H: Subcommand<D> + 'static>(&mut self, subcommand: H) {
        let cmd = self.sub_cmds.new_subcommand(subcommand);
        self.map_cmd(move |c| c.subcommand(cmd));
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
        let matches = self.matches.as_ref().unwrap();
        if let Err(e) = self.sub_cmds.handle_matches(&mut data, matches) {
            eprintln!("database : error: {e}");
            std::process::exit(4);
        }

        let batches: Vec<String> = self
            .matches
            .as_ref()
            .unwrap()
            .get_many::<String>("batch")
            .unwrap()
            .cloned()
            .collect();
        for b in batches {
            let Ok(file) = std::fs::File::open(&b)
                .map_err(|e| eprintln!("Failed to execute batch file {b} - {e}"))
            else {
                return;
            };
            use std::io::BufRead;
            let mut n = 1;
            let lines = std::io::BufReader::new(file).lines();
            for line in lines {
                let Ok(line) = line
                    .map_err(|e| eprintln!("Failed to read line {n} from batch file {b} - {e}"))
                else {
                    return;
                };
                n += 1;
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
                    Ok(submatches) => {
                        if let Err(e) = self.sub_cmds.handle_matches(&mut data, &submatches) {
                            eprintln!("Error: {e}");
                        }
                    }
                }
            }
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
                    Ok(submatches) => {
                        if let Err(e) = self.sub_cmds.handle_matches(&mut data, &submatches) {
                            eprintln!("Error: {e}");
                        }
                    }
                }
            }
        }
    }
}
