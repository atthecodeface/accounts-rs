// Transaction Date,Transaction Type,Sort Code,Account Number,Transaction Description,Debit Amount,Credit Amount,Balance
// 28/08/2024,FPI,'30-91-74,02344812,NAME REASON,,20.00,12004.61
use crate::{AccTransaction, AccTransactionType, AccountDesc, Amount, Date, Error};

//a CsvTransaction
//tp CsvTransaction
/// A transaction as it appears in a Lloyds CSV export
#[derive(Debug, serde::Deserialize)]
pub struct CsvTransaction {
    /// Date
    ///
    #[serde(rename = "Transaction Date")]
    date: String, // Date,
    /// CSV Transaction Type,
    #[serde(rename = "Transaction Type")]
    ttype: Option<String>,
    /// CSV Sort Code, Account Number
    #[serde(rename = "Sort Code")]
    sort_code: String,
    #[serde(rename = "Account Number")]
    account_number: usize,
    /// Transaction Description,
    #[serde(rename = "Transaction Description")]
    description: String,
    /// CSV Debit Amount,
    #[serde(rename = "Debit Amount")]
    debit: Option<String>,
    /// CSV: Credit Amount
    #[serde(rename = "Credit Amount")]
    credit: Option<String>,
    /// CSV: Balance
    #[serde(rename = "Balance")]
    balance: Option<String>,
}

//ip TryFrom<CsvTransaction> for AccTransaction
impl TryFrom<CsvTransaction> for AccTransaction {
    type Error = Error;
    fn try_from(csv: CsvTransaction) -> Result<AccTransaction, Error> {
        if csv.balance.is_none() {
            return Err(Error::ParseTransaction(format!(
                "CSV transaction had no balance field value {csv:?}"
            )));
        }
        let date = Date::parse(&csv.date, false)?;
        let mut debit = None;
        if let Some(d) = &csv.debit {
            let amount: Amount = d.parse()?;
            debit = Some(amount);
        };
        let debit = debit.unwrap_or_default();
        let mut credit = None;
        if let Some(d) = &csv.credit {
            let amount: Amount = d.parse()?;
            credit = Some(amount);
        };
        let credit = credit.unwrap_or_default();
        let mut balance = None;
        if let Some(d) = &csv.balance {
            let amount: Amount = d.parse()?;
            balance = Some(amount);
        };
        let balance = balance.unwrap_or_default();
        let ttype: Option<&str> = csv.ttype.as_deref();
        let ttype = AccTransactionType::parse(ttype.unwrap_or(""), !debit.is_zero())?;

        let account_desc = {
            if !csv.sort_code.is_empty() {
                AccountDesc::parse_uk(csv.sort_code.trim_start_matches("'"), csv.account_number)?
            } else {
                AccountDesc::default()
            }
        };
        let related_party = None;
        Ok(AccTransaction {
            date,
            ttype,
            account_desc,
            description: csv.description,
            debit,
            credit,
            balance,
            related_party,
        })
    }
}

//a Public functions
//fp read_transactions_csv
/// Read a CSV transactions file and return a Vec<AccTransaction>
///
/// All the transactions must belong to the same AccountDesc
pub fn read_transactions_csv<R: std::io::Read>(reader: R) -> Result<Vec<AccTransaction>, Error> {
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .quoting(false)
        .from_reader(reader);
    let mut result = vec![];
    for record in csv_reader.deserialize() {
        let record: CsvTransaction = record?;
        let transaction = record.try_into()?;
        result.push(transaction);
    }
    let result: Vec<AccTransaction> = result.into_iter().rev().collect();
    if result.len() > 1 {
        let mut balance = result[0].balance;
        for i in 1..result.len() {
            if result[i].account_desc != result[0].account_desc {
                return Err(Error::TransactionLog(format!(
                    "entry {} has different account description {} to the first entry {}",
                    i + 1,
                    result[i].account_desc,
                    result[0].account_desc
                )));
            }
            let new_balance = balance + result[i].balance_delta();
            if new_balance != result[i].balance {
                return Err(Error::TransactionLog(format!("balance before entry {} was {balance} but after it was calculated to be {new_balance}", i+1)));
            }
            balance = new_balance;
        }
    }
    Ok(result)
}
