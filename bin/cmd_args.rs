//a Imports
use std::rc::Rc;

use thunderclap::json;
use thunderclap::json::JsonValueConvert;
use thunderclap::{CommandArgs, CommandBuilder};

use rust_accounts::RelatedPartyQuery;
use rust_accounts::RelatedPartyType;
use rust_accounts::{Database, Date, DateRange, Error, FileFormat, FileType};
use rust_accounts::{DbAccount, DbFund, DbItemType, DbRelatedParty};

//a CmdArgs
//tp CmdArgs
#[derive(Default)]
pub struct CmdArgs {
    pub db: Database,
    pub verbose: bool,
    pub clear: bool,
    pub file_format: FileFormat,

    pub write_filename: String,
    pub item_type: Option<DbItemType>,
    pub rp_id: Option<usize>,
    pub rp_type: Option<RelatedPartyType>,
    pub start_date: Date,
    pub end_date: Date,
    pub postcode: Option<String>,
    pub house_number: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub tax_name: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub id: Option<usize>,
    pub string_args: Vec<String>,
    pub usize_args: Vec<usize>,
    pub value_args: Vec<Rc<json::Value>>,
}

//ip Debug for CmdArgs
impl std::fmt::Debug for CmdArgs {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "CmdArgs {{")?;
        write!(fmt, "verbose: {:?}", self.verbose)?;
        write!(fmt, "clear: {:?}", self.clear)?;
        write!(fmt, "file_format: {:?}", self.file_format)?;
        write!(fmt, "write_filename: {:?}", self.write_filename)?;
        write!(fmt, "item_type: {:?}", self.item_type)?;
        write!(fmt, "rp_id: {:?}", self.rp_id)?;
        write!(fmt, "rp_type: {:?}", self.rp_type)?;
        write!(fmt, "start_date: {:?}", self.start_date)?;
        write!(fmt, "end_date: {:?}", self.end_date)?;
        write!(fmt, "postcode: {:?}", self.postcode)?;
        write!(fmt, "house_number: {:?}", self.house_number)?;
        write!(fmt, "address: {:?}", self.address)?;
        write!(fmt, "email: {:?}", self.email)?;
        write!(fmt, "telephone: {:?}", self.telephone)?;
        write!(fmt, "tax_name: {:?}", self.tax_name)?;
        write!(fmt, "name: {:?}", self.name)?;
        write!(fmt, "desc: {:?}", self.desc)?;
        write!(fmt, "id: {:?}", self.id)?;
        write!(fmt, "}}")
    }
}

//ip CommandArgs for CmdArgs
impl CommandArgs for CmdArgs {
    type Error = Error;

    type Value = json::Value;
    fn value_from_str(s: &str) -> Result<Self::Value, Self::Error> {
        Ok(<json::Value as JsonValueConvert>::value_from_str(s)?)
    }

    fn reset_args(&mut self) {
        self.string_args.clear();
        self.usize_args.clear();
        self.value_args.clear();

        self.clear = false;

        self.id = None;
        self.name = None;
        self.rp_id = None;
        self.rp_type = None;
        self.start_date = Date::default();
        self.end_date = Date::default();
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

    //mi set_item_type
    fn set_item_type(&mut self, item_type: &str) -> Result<(), Error> {
        self.item_type = Some(item_type.parse::<DbItemType>()?);
        Ok(())
    }

    //mi set_rp_type
    fn set_rp_type(&mut self, rp_type: &str) -> Result<(), Error> {
        self.rp_type = Some(rp_type.parse::<RelatedPartyType>()?);
        Ok(())
    }

    //mi set_rp_id
    fn set_rp_id(&mut self, rp_id: usize) -> Result<(), Error> {
        self.rp_id = Some(rp_id);
        Ok(())
    }

    //mi set_id
    fn set_id(&mut self, id: usize) -> Result<(), Error> {
        self.id = Some(id);
        Ok(())
    }

    //mi set_name
    fn set_name(&mut self, s: &str) -> Result<(), Error> {
        self.name = Some(s.into());
        Ok(())
    }

    //mi set_desc
    fn set_desc(&mut self, s: &str) -> Result<(), Error> {
        self.desc = Some(s.into());
        Ok(())
    }

    //mi push_value_arg
    fn push_value_arg(&mut self, s: &Rc<json::Value>) -> Result<(), Error> {
        self.value_args.push(s.clone());
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
        self.start_date = Date::parse(s)?;
        Ok(())
    }

    //mi set_end_date
    fn set_end_date(&mut self, s: &str) -> Result<(), Error> {
        self.end_date = Date::parse(s)?;
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
        f.write_all(s.as_bytes())?;
        Ok(())
    }
}

//ip CmdArgs - getters
impl CmdArgs {
    //ap get_date
    pub fn get_date(&self) -> Result<Date, Error> {
        if self.start_date.is_none() {
            Err("No date supplied".to_string().into())
        } else {
            Ok(self.start_date)
        }
    }

    //ap get_date_range
    pub fn get_date_range(&self) -> DateRange {
        (self.start_date, self.end_date).into()
    }

    //ap get_related_party
    pub fn get_related_party(&self, name: &str) -> Result<DbRelatedParty, Error> {
        if let Some(db_m) = self
            .db
            .related_parties()
            .get_party(name, RelatedPartyQuery::Any)
        {
            Ok(db_m)
        } else {
            Err(format!("Did not find related party '{name}'").into())
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
            Err(format!("Did not find member '{name}'").into())
        }
    }

    //ap get_account
    pub fn get_account(&self, name: &str) -> Result<DbAccount, Error> {
        if let Some(db_acc) = self.db.accounts().get_account_by_name(name) {
            Ok(db_acc)
        } else {
            Err(format!("Did not find account '{name}'").into())
        }
    }

    //ap get_fund
    pub fn get_fund(&self, name: &str) -> Result<DbFund, Error> {
        if let Some(db_acc) = self.db.funds().get_fund(name) {
            Ok(db_acc)
        } else {
            Err(format!("Did not find fund '{name}'").into())
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

    //fp arg_add_option_search_name
    pub fn arg_add_option_search_name(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "name",
            None,
            "Name to search for (possibly a regex)",
            false,
            None,
            Self::set_name,
        );
    }

    //fp arg_add_option_search_desc
    pub fn arg_add_option_search_desc(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_string(
            "desc",
            None,
            "Description to search for (possibly a regex)",
            false,
            None,
            Self::set_desc,
        );
    }

    //fp arg_add_option_search_id
    pub fn arg_add_option_search_id(builder: &mut CommandBuilder<Self>) {
        builder.add_arg_usize("id", None, "Id to look for", false, None, Self::set_id);
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

    //fp arg_add_option_item_type
    pub fn arg_add_option_item_type(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_string(
            "item_type",
            None,
            "Item type: fund/account/transaction/bank_transaction/related_party/invoice",
            required,
            None,
            Self::set_item_type,
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

    //fp arg_add_option_rp_type
    pub fn arg_add_option_rp_type(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_string(
            "rp_type",
            None,
            "RP type: director/member/friend/muicisan/supplier",
            required,
            None,
            Self::set_rp_type,
        );
    }

    //fp arg_add_related_party_positional
    pub fn arg_add_related_party_positional(builder: &mut CommandBuilder<Self>) {
        Self::arg_add_positional_string(
            builder,
            "name",
            "Related party identification",
            Some(1),
            None,
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

    //fp arg_add_positional_value
    /// count should be None for optional; Some(0) for a single
    /// optional argument, Some(n) for a required number
    pub fn arg_add_positional_value(
        builder: &mut CommandBuilder<Self>,
        tag: &'static str,
        help: &'static str,
        count: Option<usize>,
        default_value: Option<&'static str>,
    ) {
        builder.add_arg_value(
            tag,
            None,
            help,
            (count, true),
            default_value,
            Self::push_value_arg,
        );
    }

    //fp arg_add_positional_usize
    /// count should be None for optional; Some(0) for a single
    /// optional argument, Some(n) for a required number
    pub fn arg_add_positional_usize(
        builder: &mut CommandBuilder<Self>,
        tag: &'static str,
        help: &'static str,
        count: Option<usize>,
        default_value: Option<&'static str>,
    ) {
        builder.add_arg_usize(
            tag,
            None,
            help,
            (count, true),
            default_value,
            Self::push_usize_arg,
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
