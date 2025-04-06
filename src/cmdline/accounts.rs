//a Imports
use clap::{value_parser, Arg, ArgMatches, Command};

use crate::cmdline::{Subcommand, SubcommandSet};
use crate::{Database, Error};

//a Accounts
#[derive(Default)]
struct AccountsList();

//ip Subcommand for AccountsList
impl Subcommand<Database> for AccountsList {
    //mp create_subcommand
    fn create_subcommand(&mut self) -> Command {
        Command::new("list").about("List the accounts")
    }

    //mp handle
    fn handle(&mut self, db: &mut Database, matches: &ArgMatches) -> Result<(), Error> {
        println!("Accounts:");
        for k in db.accounts().descs() {
            let account = db.accounts().get_account(k).unwrap().borrow();
            println!("  {k} : {} - {}", account.org(), account.name());
        }
        Ok(())
    }
}

#[derive(Default)]
struct AccountsAdd();

//ip Subcommand for AccountsAdd
impl Subcommand<Database> for AccountsAdd {
    //mp create_subcommand
    fn create_subcommand(&mut self) -> Command {
        Command::new("add")
            .about("Add an account")
            .arg(
                Arg::new("sort_code")
                    .required(true)
                    .help("Sort code (xx-xx-xx)"),
            )
            .arg(
                Arg::new("account_number")
                    .required(true)
                    .value_parser(value_parser!(usize))
                    .help("Account number"),
            )
            .arg(Arg::new("bank").required(true).help("Bank name"))
            .arg(Arg::new("name").required(true).help("Account name"))
    }

    //mp handle
    fn handle(&mut self, db: &mut Database, matches: &ArgMatches) -> Result<(), Error> {
        let sort_code = matches.get_one::<String>("sort_code").unwrap();
        let account_number = matches.get_one::<usize>("account_number").unwrap();
        let bank = matches.get_one::<String>("bank").unwrap();
        let name = matches.get_one::<String>("name").unwrap();
        let desc = crate::AccountDesc::parse_uk(sort_code, *account_number)?;
        let account = crate::Account::new(bank.to_owned(), name.to_owned(), desc);
        db.add_account(account);
        Ok(())
    }
}

#[derive(Default)]
pub struct Accounts(SubcommandSet<Database>);

//ip Subcommand for Accounts
impl Subcommand<Database> for Accounts {
    //mp create_subcommand
    fn create_subcommand(&mut self) -> Command {
        Command::new("accounts")
            .about("Perform operations on accounts")
            .subcommand(self.0.new_subcommand(AccountsList::default()))
            .subcommand(self.0.new_subcommand(AccountsAdd::default()))
    }

    //mp handle
    fn handle(&mut self, db: &mut Database, matches: &ArgMatches) -> Result<(), Error> {
        self.0.handle_matches(db, matches)
    }
}
