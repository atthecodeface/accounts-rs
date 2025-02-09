use rust_accounts::lloyds;

//a Constants
const Stuff: &str = r##"Transaction Date,Transaction Type,Sort Code,Account Number,Transaction Description,Debit Amount,Credit Amount,Balance
05/02/2025,FPI,'10-11-14,12345678,BLOB BLOB-SUB 00000009876543210 892313     10 05FEB25 12:56,,332.00,747.17
05/02/2025,FPI,'10-11-14,12345678,BLAH BLAH-SUB RP123456790123456 102340     30 05FEB25 01:14,,21.00,415.17
05/02/2025,FPI,'10-11-14,12345678,FOO FOO-SUB   RP987654321098765 504932     30 05FEB25 01:12,,21.00,394.17"##;

//a Tests
#[test]
fn test_me() -> Result<(), Box<dyn std::error::Error>> {
    lloyds::read_transactions_csv(Stuff.as_bytes())?;
    assert!(false);
    Ok(())
}
