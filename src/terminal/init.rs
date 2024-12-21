use crate::structs;
use std::io::Error;
use std::io::{self, Write};

pub fn startup() -> Result<structs::Credentials<'static>, Error> {
    let mut creds = structs::Credentials::new_default();
    let mut input = String::new();

    print!("Enter your callsign: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    creds.callsign = Box::leak(input.trim().to_string().into_boxed_str());
    input.clear();

    print!("Enter your email: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    creds.email = Box::leak(input.trim().to_string().into_boxed_str());
    input.clear();

    print!("Enter your password: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    creds.password = Box::leak(input.trim().to_string().into_boxed_str());
    Ok(creds)
}