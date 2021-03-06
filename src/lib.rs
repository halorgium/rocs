#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prettytable;
extern crate dirs;
extern crate reqwest;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
extern crate spinners;
extern crate time;
extern crate url;
extern crate uuid;

pub mod apis;
pub mod cli;
pub mod models;
pub mod store;
