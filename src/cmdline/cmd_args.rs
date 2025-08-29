//a Imports
use thunderclap::{CommandArgs, CommandBuilder};

use crate::RelatedPartyQuery;
use crate::{Database, Date, Error, FileFormat, FileType};
use crate::{DbAccount, DbFund, DbRelatedParty};

//a CmdArgs
//tp CmdArgs
#[derive(Default)]
pub struct CmdArgs {
    pub db: Database,
    pub verbose: bool,
    pub clear: bool,
    pub file_format: FileFormat,

    pub write_filename: String,
    pub rp_id: Option<usize>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub postcode: Option<String>,
    pub house_number: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub tax_name: Option<String>,
    pub string_args: Vec<String>,
    pub usize_args: Vec<usize>,
}

//ip CommandArgs for CmdArgs
impl CommandArgs for CmdArgs {
    type Error = Error;
    type Value = String;
    fn cmd_ok() -> Result<String, Error> {
        Ok("".into())
    }
    fn reset_args(&mut self) {
        self.string_args.clear();
        self.usize_args.clear();

        self.clear = false;

        self.rp_id = None;
        self.start_date = None;
        self.end_date = None;
        self.postcode = None;
        self.house_number = None;
        self.address = None;
        self.email = None;
        self.telephone = None;
        self.tax_name = None;
    }
}

//ip CmdArgs - setters
impl CmdArgs {
    //mi set_verbose
    pub(crate) fn set_verbose(&mut self, verbose: bool) -> Result<(), Error> {
        self.verbose = verbose;
        Ok(())
    }

    //mi set_clear
    pub(crate) fn set_clear(&mut self, clear: bool) -> Result<(), Error> {
        self.clear = clear;
        Ok(())
    }

    //mi set_rp_id
    fn set_rp_id(&mut self, rp_id: usize) -> Result<(), Error> {
        self.rp_id = Some(rp_id);
        Ok(())
    }

    //mi push_usize_arg
    fn push_usize_arg(&mut self, s: usize) -> Result<(), Error> {
        self.usize_args.push(s);
        Ok(())
    }

    //mi push_string_arg
    fn push_string_arg(&mut self, s: &str) -> Result<(), Error> {
        self.string_args.push(s.into());
        Ok(())
    }

    //mi set_write_filename
    fn set_write_filename(&mut self, s: &str) -> Result<(), Error> {
        self.write_filename = s.into();
        Ok(())
    }

    //mi set_file_format
    fn set_file_format(&mut self, s: &str) -> Result<(), Error> {
        self.file_format = s.parse::<FileFormat>()?;
        Ok(())
    }

    //mi set_postcode
    fn set_postcode(&mut self, s: &str) -> Result<(), Error> {
        self.postcode = Some(s.into());
        Ok(())
    }

    //mi set_house_number
    fn set_house_number(&mut self, s: &str) -> Result<(), Error> {
        self.house_number = Some(s.into());
        Ok(())
    }

    //mi set_tax_name
    fn set_tax_name(&mut self, s: &str) -> Result<(), Error> {
        self.tax_name = Some(s.into());
        Ok(())
    }

    //mi set_address
    fn set_address(&mut self, s: &str) -> Result<(), Error> {
        self.address = Some(s.into());
        Ok(())
    }

    //mi set_email
    fn set_email(&mut self, s: &str) -> Result<(), Error> {
        self.email = Some(s.into());
        Ok(())
    }

    //mi set_telephone
    fn set_telephone(&mut self, s: &str) -> Result<(), Error> {
        self.telephone = Some(s.into());
        Ok(())
    }

    //mi set_start_date
    fn set_start_date(&mut self, s: &str) -> Result<(), Error> {
        let date = Date::parse(s, false)?;
        self.start_date = Some(date);
        Ok(())
    }

    //mi set_end_date
    fn set_end_date(&mut self, s: &str) -> Result<(), Error> {
        let date = Date::parse(s, false)?;
        self.end_date = Some(date);
        Ok(())
    }

    //mi load_database
    fn load_database(&mut self, filename: &str) -> Result<(), Error> {
        let ftype = FileType::from_filename(filename)?;
        let s = std::fs::read_to_string(filename)?;
        match ftype {
            FileType::Json => {
                let mut deserializer =
                    serde_json::Deserializer::new(serde_json::de::StrRead::new(&s));
                // Deserialize from Vec<DbItem>
                self.db = Database::deserialize(&mut deserializer, FileFormat::Array)?;
                Ok(())
            }
            FileType::Yaml => {
                let deserializer = serde_yaml::Deserializer::from_str(&s);
                // Deserialize from Vec<DbItem>
                self.db = Database::deserialize(deserializer, FileFormat::Array)?;
                Ok(())
            }
            _ => Err(Error::FileTypeNotSupported(ftype, "database")),
        }
    }
}

//ip CmdArgs - operations
impl CmdArgs {
    pub fn write_database(&self) -> Result<(), Error> {
        let ftype = FileType::from_filename(&self.write_filename)?;

        // use self.file_format

        let mut w = vec![];
        match ftype {
            FileType::Json => {
                let mut s = serde_json::Serializer::pretty(w);
                self.db.serialize_as_array(&mut s)?;
                w = s.into_inner();
            }
            FileType::Yaml => {
                let mut s = serde_yaml::Serializer::new(&mut w);
                self.db.serialize_as_array(&mut s)?;
            }
            _ => return Err("Cannot write database out as CSV fle".to_string().into()),
        }
        use std::io::Write;
        let s = std::str::from_utf8(&w).unwrap();
        let mut f = std::fs::File::create(&self.write_filename)?;
        f.write(s.as_bytes())?;
        Ok(())
    }
}

//ip CmdArgs - getters
impl CmdArgs {
    //ap get_date
    pub fn get_date(&self) -> Result<Date, Error> {
        if let Some(date) = self.start_date {
            Ok(date)
        } else {
            Err("No date supplied".to_string().into())
        }
    }

    //ap get_date_range
    pub fn get_date_range(&self) -> Option<(Date, Date)> {
        if let Some(start_date) = self.start_date {
            if let Some(end_date) = self.end_date {
                Some((start_date, end_date))
            } else {
                Some((start_date, Date::default()))
            }
        } else {
            None
        }
    }

    //ap get_member
    pub fn get_member(&self, name: &str) -> Result<DbRelatedParty, Error> {
        if let Some(db_m) = self
            .db
            .related_parties()
            .get_party(name, RelatedPartyQuery::Any)
        {
            Ok(db_m)
        } else {
            Err(format!("Did not find member '{}'", name).into())
        }
    }

    //ap get_account
    pub fn get_account(&self, name: &str) -> Result<DbAccount, Error> {
        if let Some(db_acc) = self.db.accounts().get_account_by_name(name) {
            Ok(db_acc)
        } else {
            Err(format!("Did not find account '{}'", name).into())
        }
    }

    //ap get_fund
    pub fn get_fund(&self, name: &str) -> Result<DbFund, Error> {
        if let Some(db_acc) = self.db.funds().get_fund(name) {
            Ok(db_acc)
        } else {
            Err(format!("Did not find fund '{}'", name).into())
        }
    }
}

//ip CmdArgs - arg adders
impl CmdArgs {
    //fp arg_add_verbose
    pub fn arg_add_verbose(build: &mut CommandBuilder<Self>) {
        build.add_flag(
            "verbose",
            Some('v'),
            "Enable verbose output",
            CmdArgs::set_verbose,
        );
    }

    //fp arg_add_clear
    pub fn arg_add_clear(build: &mut CommandBuilder<Self>) {
        build.add_flag("clear", None, "Clear data first", CmdArgs::set_clear);
    }

    //fp arg_add_database
    pub fn arg_add_database(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "db",
            Some('d'),
            "Database to read initially",
            false,
            None,
            Self::load_database,
        );
    }

    //fp arg_add_option_postcode
    pub fn arg_add_option_postcode(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "postcode",
            None,
            "Postcode to use",
            false,
            None,
            Self::set_postcode,
        );
    }

    //fp arg_add_option_house_number
    pub fn arg_add_option_house_number(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "house_number",
            None,
            "House_Number to use",
            false,
            None,
            Self::set_house_number,
        );
    }

    //fp arg_add_option_tax_name
    pub fn arg_add_option_tax_name(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "tax_name",
            None,
            "Tax_Name to use",
            false,
            None,
            Self::set_tax_name,
        );
    }

    //fp arg_add_option_address
    pub fn arg_add_option_address(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "address",
            None,
            "Address to use",
            false,
            None,
            Self::set_address,
        );
    }

    //fp arg_add_option_email
    pub fn arg_add_option_email(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string("email", None, "Email to use", false, None, Self::set_email);
    }

    //fp arg_add_option_telephone
    pub fn arg_add_option_telephone(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "telephone",
            None,
            "Telephone to use",
            false,
            None,
            Self::set_telephone,
        );
    }

    //fp arg_add_option_start_date
    pub fn arg_add_option_start_date(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "start_date",
            None,
            "Start date; if no end date is supplied, then only this date",
            false,
            None,
            Self::set_start_date,
        );
    }

    //fp arg_add_option_end_date
    pub fn arg_add_option_end_date(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "end_date",
            None,
            "End date",
            false,
            None,
            Self::set_end_date,
        );
    }

    //fp arg_add_option_rp_id
    pub fn arg_add_option_rp_id(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_usize(
            "rp_id",
            Some('i'),
            "RP Id - by convention director=1, member number if 100-500, friend if 500-1000, musician if 1000-2000, supplier 2000+",
            required,
            None,
            Self::set_rp_id,
        );
    }

    //fp arg_add_member_positional
    pub fn arg_add_member_positional(builder: &mut CommandBuilder<Self>) {
        Self::arg_add_positional_string(builder, "name", "Member identification", Some(1), None);
    }

    //fp arg_add_account_positional
    pub fn arg_add_account_positional(builder: &mut CommandBuilder<Self>) {
        Self::arg_add_positional_string(
            builder,
            "account",
            "Account identification",
            Some(1),
            None,
        );
    }

    //fp arg_add_fund_positional
    pub fn arg_add_fund_positional(builder: &mut CommandBuilder<Self>) {
        Self::arg_add_positional_string(builder, "fund", "Fund identification", Some(1), None);
    }

    //fp arg_add_positional_string
    /// count should be None for optional; Some(0) for a single
    /// optional argument, Some(n) for a required number
    pub fn arg_add_positional_string(
        builder: &mut CommandBuilder<Self>,
        tag: &'static str,
        help: &'static str,
        count: Option<usize>,
        default_value: Option<&'static str>,
    ) {
        builder.add_arg_string(
            tag,
            None,
            help,
            (count, true),
            default_value,
            Self::push_string_arg,
        );
    }

    //fp arg_add_option_usize
    /// This is added as the *next* usize_arg in the Vec
    ///
    /// As such it is *required* or, optional, must have a *default* value
    pub fn arg_add_option_usize(
        builder: &mut CommandBuilder<Self>,
        tag: &'static str,
        short: Option<char>,
        help: &'static str,
        default_value: Option<&'static str>,
    ) {
        let required = default_value.is_none();
        builder.add_arg_usize(
            tag,
            short,
            help,
            required,
            default_value,
            Self::push_usize_arg,
        );
    }

    //fp arg_add_option_string
    /// This is added as the *next* string_arg in the Vec
    ///
    /// As such it is *required* or, optional, must have a *default* value
    pub fn arg_add_option_string(
        builder: &mut CommandBuilder<Self>,
        tag: &'static str,
        short: Option<char>,
        help: &'static str,
        default_value: Option<&'static str>,
    ) {
        let required = default_value.is_none();
        builder.add_arg_string(
            tag,
            short,
            help,
            required,
            default_value,
            Self::push_string_arg,
        );
    }

    //fp arg_add_file_format
    pub fn arg_add_file_format(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "format",
            Some('f'),
            "Format to write",
            false,
            None,
            Self::set_file_format,
        );
    }

    //fp arg_add_write_filename
    pub fn arg_add_write_filename(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "output_filename",
            Some('w'),
            "Output filename",
            false,
            None,
            Self::set_write_filename,
        );
    }
}
