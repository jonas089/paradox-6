#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use crate::constants::RocketCfg;
use rocket::{Config, figment::{
    Figment, providers::{Toml, Env, Format}
}};
use std::env;
use std::path::PathBuf;
mod sync;
mod types;
mod noir;
mod circom;
mod constants;
mod storage;
pub mod node;
use node::router::{proof_receiver, noir_query, circom_query, ping};

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let env_args: Vec<String> = env::args().collect();
    /*
    let peer: Vec<(String, u16)> = vec![(
        "http://127.0.0.1".to_string(),
        env_args[3].parse::<u16>().unwrap(),
    )];*/
    let mut peers: Vec<(String, u16)> = Vec::new();
    for arg in env_args.iter().enumerate().filter(|&(index, _)| index > 2){
        peers.push(("http://127.0.0.1".to_string(), arg.1.parse::<u16>().unwrap()));
    }

    let cfg = RocketCfg {
        path_to_db: PathBuf::from(env_args[1].clone()),
        peers: peers.clone(),
    };
    println!(
        "Starting server with database path: {:?}, port: {:?}, peers: {:?}",
        &env_args[1], &env_args[2], &peers,
    );

    // Create a figment configuration
    let figment = Figment::new()
        .merge(Config::default())
        .merge(Toml::file("./Rocket.toml").nested())  // Merge settings from Rocket.toml
        .merge(Env::prefixed("ROCKET_"));

    // Manually override specific settings if needed
    let port: u16 = env_args[2].parse().expect("Invalid port number");
    let figment = figment.merge(("port", port));

    // Launch the application with the custom configuration
    rocket::custom(figment)
        .mount("/", routes![proof_receiver, noir_query, circom_query, ping])
        .manage(cfg)
        .launch()
        .await?;
    Ok(())
}