//a Imports
use std::rc::Rc;

use thunderclap::json;
use thunderclap::json::JsonValueConvert;
use thunderclap::{CommandArgs, CommandBuilder};

// use rust_accounts::Idx;
use rust_accounts::RelatedPartyQuery;
use rust_accounts::RelatedPartyType;
use rust_accounts::{Amount, Database, Date, DateRange, Error, FileFormat, FileType};
use rust_accounts::{DbAccount, DbBankTransaction, DbFund, DbId, DbItemType, DbRelatedParty};

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
    pub db_id: Vec<DbId>,
    pub rp_id: Option<usize>,
    pub amount: Amount,
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
    pub account: Option<String>,
    pub string_args: Vec<String>,
    pub usize_args: Vec<usize>,
    pub value_args: Vec<Rc<json::Value>>,
    pub string_args_index: usize,
    pub usize_args_index: usize,
    pub value_args_index: usize,
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
        write!(fmt, "amount: {:?}", self.amount)?;
        write!(fmt, "db_id: {:?}", self.db_id)?;
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
        write!(fmt, "account: {:?}", self.account)?;
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
        self.db_id = vec![];
        self.rp_id = None;
        self.rp_type = None;
        self.amount = Amount::default();
        self.start_date = Date::default();
        self.end_date = Date::default();
        self.postcode = None;
        self.house_number = None;
        self.address = None;
        self.email = None;
        self.telephone = None;
        self.tax_name = None;

        self.string_args_index = 0;
        self.usize_args_index = 0;
        self.value_args_index = 0;
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

    //mi set_amount
    fn set_amount(&mut self, amount: &str) -> Result<(), Error> {
        self.amount = amount.parse::<Amount>()?;
        Ok(())
    }

    //mi set_rp_type
    fn set_rp_type(&mut self, rp_type: &str) -> Result<(), Error> {
        self.rp_type = Some(rp_type.parse::<RelatedPartyType>()?);
        Ok(())
    }

    //mi set_db_id
    fn set_db_id(&mut self, db_id: &Rc<serde_json::Value>) -> Result<(), Error> {
        if let Some(db_id) = db_id.as_u64() {
            let db_id = DbId::of_usize(db_id as usize);
            if !self.db.has_db_id(db_id) {
                Err(format!("Database does not contain ID {db_id}").into())
            } else {
                self.db_id.push(db_id);
                Ok(())
            }
        } else if let Some(db_id_array) = db_id.as_array() {
            for db_id in db_id_array {
                let Some(db_id) = db_id.as_u64() else {
                    return Err(
                        format!("db_id was not an array of database ids, got {db_id}").into(),
                    );
                };
                let db_id = DbId::of_usize(db_id as usize);
                if !self.db.has_db_id(db_id) {
                    return Err(format!("Database does not contain ID {db_id}").into());
                }
                self.db_id.push(db_id);
            }
            Ok(())
        } else {
            Err(format!("Value {db_id} must be an array of id or a single id").into())
        }
    }

    //mi set_rp_id
    fn set_rp_id(&mut self, rp_id: &str) -> Result<(), Error> {
        if let Some(db_rp) = self.db.related_parties().get_party_of_str(rp_id) {
            self.rp_id = Some(db_rp.inner().rp_id());
        } else {
            self.rp_id = Some(rp_id.parse::<usize>().map_err(|_| format!("Failed to parse rp_id '{rp_id}' - it could be a related party name, or an RP id integer"))?);
        }
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

    //mi set_account
    fn set_account(&mut self, s: &str) -> Result<(), Error> {
        self.account = Some(s.into());
        Ok(())
    }

    //mi set_related_party
    fn set_related_party(&mut self, s: &str) -> Result<(), Error> {
        let db_rp = self.get_related_party_by_name(s)?;
        self.rp_id = Some(db_rp.inner().rp_id());
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
    pub fn get_related_party(&self) -> Result<DbRelatedParty, Error> {
        let Some(rp_id) = self.rp_id else {
            return Err(format!("No rp_id provided").into());
        };
        if let Some(db_m) = self.db.related_parties().get_rp_id(rp_id) {
            Ok(db_m)
        } else {
            Err(format!("Did not find related party '{rp_id}'").into())
        }
    }

    //ap get_related_party_by_name
    pub fn get_related_party_by_name(&self, name: &str) -> Result<DbRelatedParty, Error> {
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

    //ap get_member_by_name
    pub fn get_member_by_name(&self, name: &str) -> Result<DbRelatedParty, Error> {
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
    pub fn get_account(&self) -> Result<DbAccount, Error> {
        self.get_account_by_name(
            self.account
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_default(),
        )
    }

    //ap get_account_by_name
    pub fn get_account_by_name(&self, name: &str) -> Result<DbAccount, Error> {
        if let Some(db_acc) = self.db.accounts().get_account_by_name(name) {
            Ok(db_acc)
        } else {
            Err(format!("Did not find account '{name}'").into())
        }
    }

    //ap get_bank_transaction
    /// If db_id is specified, then that should be the bank transaction
    pub fn get_bank_transaction(&mut self) -> Result<DbBankTransaction, Error> {
        if !self.db_id.is_empty() {
            if let Some(d_bt) = self.db.get_bank_transaction(self.db_id[0]) {
                return Ok(d_bt);
            }
            return Err(format!("Db id {} is not a bank transaction", self.db_id[0]).into());
        } else {
            let db_acc = self.get_account()?;
            let rp = self.get_related_party()?;
            let opt_nth = Some(self.next_usize_arg()?);
            if let Some((db_id, _n)) = db_acc.inner().get_bank_transaction(
                &self.db,
                self.start_date,
                rp.id(),
                Amount::default(),
                opt_nth,
            ) {
                if let Some(d_bt) = self.db.get_bank_transaction(db_id) {
                    return Ok(d_bt);
                }
            }
            return Err(format!(
                "Failed to find bank transaction for date {} from account '{}' with related party '{}' nth {:?}",
                self.start_date,
                db_acc.inner().name(),
                rp.inner().name(),
                opt_nth
            )
            .into());
        }
    }

    //ap get_bank_transactions
    /// If db_id is specified, then that should be the bank transaction
    pub fn get_bank_transactions(&mut self) -> Result<Vec<DbBankTransaction>, Error> {
        let mut bank_transactions = vec![];
        for db_id in self.db_id.iter() {
            let Some(d_bt) = self.db.get_bank_transaction(*db_id) else {
                return Err(format!("Db id {} is not a bank transaction", db_id).into());
            };
            bank_transactions.push(d_bt);
        }
        Ok(bank_transactions)
    }

    //ap get_fund
    pub fn get_fund(&self, name: &str) -> Result<DbFund, Error> {
        if let Some(db_acc) = self.db.funds().get_fund(name) {
            Ok(db_acc)
        } else {
            Err(format!("Did not find fund '{name}'").into())
        }
    }

    //ap next_string_arg
    pub fn next_string_arg(&mut self) -> Result<String, Error> {
        let n = self.string_args_index;
        self.string_args_index += 1;
        if let Some(arg) = self.string_args.get(n).cloned() {
            Ok(arg)
        } else {
            Err(format!("ran out of string arguments {n}").into())
        }
    }

    //ap remaining_string_args
    pub fn remaining_string_args<'a>(&'a mut self) -> impl Iterator<Item = String> + use<'a> {
        let n = self.string_args_index.min(self.string_args.len());
        self.string_args.drain(..).skip(n)
    }

    //ap next_usize_arg
    pub fn next_usize_arg(&mut self) -> Result<usize, Error> {
        let n = self.usize_args_index;
        self.usize_args_index += 1;
        if let Some(arg) = self.usize_args.get(n).copied() {
            Ok(arg)
        } else {
            Err(format!("ran out of string arguments {n}").into())
        }
    }

    //ap next_value_arg
    pub fn next_value_arg(&mut self) -> Result<Rc<json::Value>, Error> {
        let n = self.value_args_index;
        self.value_args_index += 1;
        if let Some(arg) = self.value_args.get(n).cloned() {
            Ok(arg)
        } else {
            Err(format!("ran out of string arguments {n}").into())
        }
    }

    //z All done
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

    //fp arg_add_option_account
    pub fn arg_add_option_account(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_string(
            "account",
            None,
            "Account",
            required,
            None,
            Self::set_account,
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

    //fp arg_add_option_date
    pub fn arg_add_option_date(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_string(
            "date",
            None,
            "Date of the entry",
            required,
            None,
            Self::set_start_date,
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

    //fp arg_add_option_db_id
    pub fn arg_add_option_db_id(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_value(
            "db_id",
            Some('i'),
            "DB Id - database ids to use",
            required,
            None,
            Self::set_db_id,
        );
    }

    //fp arg_add_option_rp_id
    pub fn arg_add_option_rp_id(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_string(
            "rp_id",
            Some('i'),
            "RP Id - by convention director=1, member number if 100-500, friend if 500-1000, musician if 1000-2000, supplier 2000+",
            required,
            None,
            Self::set_rp_id,
        );
    }

    //fp arg_add_option_amount
    pub fn arg_add_option_amount(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_string("amount", None, "Amount", required, None, Self::set_amount);
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

    //fp arg_add_option_related_party
    pub fn arg_add_option_related_party(builder: &mut CommandBuilder<Self>, required: bool) {
        builder.add_arg_string(
            "rp",
            Some('r'),
            "Related party",
            required,
            None,
            Self::set_related_party,
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

    //fp arg_add_bank_transaction
    pub fn arg_add_bank_transaction(builder: &mut CommandBuilder<Self>) {
        // Need to allow for an option of specifying the DbId
        Self::arg_add_option_date(builder, true);
        Self::arg_add_option_account(builder, true);
        Self::arg_add_option_related_party(builder, true);
        Self::arg_add_option_usize(
            builder,
            "nth",
            None,
            "Which of the transactions on that day with the related party",
            Some("0"),
        ); // Not required as default value is Some
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
