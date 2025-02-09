use rust_accounts::lloyds;

//a Tests
#[test]
fn test_me() -> Result<(), Box<dyn std::error::Error>> {
    lloyds::read_transactions_csv(Stuff.as_bytes())?;
    assert!(false);
    Ok(())
}
