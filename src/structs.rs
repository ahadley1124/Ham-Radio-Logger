use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Response {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Credentials<'a> {
    pub callsign: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl Credentials<'_> {
    pub fn new_default() -> Credentials<'static> {
        Credentials {
            callsign: "",
            email: "",
            password: "",
        }
    }

    pub fn clone(&self) -> Credentials<'_> {
        Credentials {
            callsign: self.callsign,
            email: self.email,
            password: self.password,
        }
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    pub creds: Credentials<'a>,
    pub grid: String,
    pub qth: String,
    pub rig: String,
    pub power: String,
    pub antenna: String,
}

impl Config<'_> {
    pub fn clone(&self) -> Config<'_> {
        Config {
            //clone the credentials
            creds: self.creds.clone(),
            grid: self.grid.clone(),
            qth: self.qth.clone(),
            rig: self.rig.clone(),
            power: self.power.clone(),
            antenna: self.antenna.clone(),
        }
    }
}