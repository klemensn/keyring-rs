extern crate keyring;

use keyring::{Entry, Error, Result};

fn main() -> Result<()> {
    let username = "example-username";
    let service = "example-service";
    let password = "example-password";
    let keyring = Entry::new(service, username);
    if let Err(err) = keyring.set_password(password) {
        panic!("Could not set password: {}", err)
    }
    match keyring.get_password() {
        Ok(stored_password) => assert_eq!(
            password, stored_password,
            "Stored and retrieved passwords don't match"
        ),
        Err(Error::NoEntry) => panic!("Password was set but not found?"),
        Err(err) => panic!("Could not get password: {}", err),
    }
    if let Err(err) = keyring.delete_password() {
        panic!("Could not delete password: {}", err);
    }
    assert!(
        keyring.get_password().is_err(),
        "No error retrieving password after deletion"
    );

    Ok(())
}
