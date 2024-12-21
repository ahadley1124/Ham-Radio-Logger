use std::fmt::Display;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Record;
use surrealdb::Surreal;
use surrealdb::opt::auth::Root;

use crate::structs::{Response, Credentials};

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token: {}", self.token)
    }
}

pub async fn signup(creds: Credentials<'_>) -> Result<String, surrealdb::Error> {
    let db = Surreal::new::<Ws>("10.0.0.128:8000").await?;
    println!("Connected to database");
    println!("Surreal Client: {:?}", db);
    let jwt = db.signup(Record {
        namespace: "radio-logger",
        database: "radio-logger",
        access: "account",
        params: Root {
            username: creds.email,
            password: creds.password,
        },
    }).await?;
    println!("User signed up successfully");
    let token = jwt.as_insecure_token().to_string();
    println!("Token: {}", token);
    Ok(token)
}

pub async fn signin(creds: Credentials<'_>) -> Result<String, surrealdb::Error> {
    let db = Surreal::new::<Ws>("10.0.0.128:8000").await?;
    println!("Connected to database");
    println!("Surreal Client: {:?}", db);
    // default login to sign a user up is "account_signup", password is "account_signup"
    db.signin(Root {
        username: "account_signup",
        password: "account_signup",
    }).await?;
    println!("Signed in as account_signup");
    db.use_ns("radio-logger").use_db("radio-logger").await?;
    println!("Using namespace radio-logger and database radio-logger");
    //get the current user's token
    let jwt = db.signin(Record {
        namespace: "radio-logger",
        database: "radio-logger",
        access: "account",
        params: Root {
            username: creds.email,
            password: creds.password,
        },
    }).await?;
    println!("User signed in successfully");
    let token = jwt.as_insecure_token().to_string();
    println!("Token: {}", token);
    Ok(token)
}

