use rust_accounts::{Account, AccountDesc, DbAccounts, DbId};

//a Tests
#[test]
fn test_me() -> Result<(), Box<dyn std::error::Error>> {
    let desc = AccountDesc::uk(12345, 871645);
    let a = Account::new("TSB".into(), "Savings".into(), desc);

    let mut accs = DbAccounts::new();
    let db_id = DbId::default();
    accs.add_account((db_id, a).into());

    let j = serde_json::to_string(&accs)?;

    eprintln!("{j}");
    assert!(false);
    Ok(())
}
