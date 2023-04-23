#[macro_use]
extern crate log;

pub mod api;
pub mod client;
pub mod message;
pub mod update;

pub use api::*;
pub use client::Client;
pub use message::*;

pub fn init_logger() {
    // We use try_init here so it can by run by tests.
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init();
    debug!("Logger initialized.");
}
