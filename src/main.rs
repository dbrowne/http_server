//! Simple Rocket HTTP server
//! Dwight J. Browne
//! Quick and dirty example
//! -p port #
//! Environment variables are
//! SERVER_PORT
//! Hard coded localhost 127.0.0.1
//! Not creating a dependency on the ROCKET_ environment vars.


#[macro_use] extern crate rocket;
extern crate chrono;

use chrono::{Local, DateTime};
use git_version::git_version;
use std::net::{IpAddr,Ipv4Addr};
use structopt::StructOpt;
use std::env;
use rocket::Config;

const GIT_VERSION: &str = git_version!();
#[get("/")]
fn index() -> String {
    let datetime: DateTime<Local> = Local::now();
    let host_name = match hostname::get() {
        Err(er) => panic!("Can't retrieve the system hostname: {}", er),
        Ok(host_name) => host_name
    };
    format!("HOSTNAME: ->{:?}<-: Time: {} LOCAL:  server version: ->{}<- \n", host_name, datetime.format("%m/%d/%Y: %T"), GIT_VERSION)
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    const K_PORT: &str = "3333";
    const K_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127,0,0,1)); //for future use

    // check for command line args
    #[derive(StructOpt, Debug)]
    // #[derive(Debug)]
    #[structopt(rename_all = "kebab-case")]
    struct Opt {
        #[structopt(default_value = K_PORT, short)]
        port: u16,
    }
    let opt = Opt::from_args();

    let _port: u16;
    // check for environment variables
    let env_port: String = env::var("SERVER_PORT").unwrap_or(K_PORT.to_string());

    // see if we have any input args
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Have input args: Overriding default vars with the following: ENV VARS IGNORED!");
        println!("{:#?}", &opt);
        _port = opt.port;

    }else {
        println!("Using environment vars: with the following values");
        println!("port: SERVER_PORT                        {}", env_port);
        _port = env_port.parse::<u16>().unwrap();
    }

    let config = Config {
        port: _port,
        address: K_ADDR.into(),
        ..Config::default()
    };

    rocket::custom(&config).mount("/", routes![index]).launch().await;

}


