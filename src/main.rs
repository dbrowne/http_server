#[macro_use] extern crate rocket;
extern crate chrono;

use std::error::Error;
use std::io;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::{SystemTime};

#[get("/")]
fn index() -> String {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    let  host_name = match hostname::get(){
        Err(er) => panic!("Can't retreive the system hostname: {}",er.description()),
        Ok(host_name) =>host_name
    };

    format!("{:?}: {}:", host_name, datetime.format("%m/%d/%Y: %T \n"))
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}


