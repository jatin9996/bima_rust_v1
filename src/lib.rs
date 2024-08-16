#[macro_use]
mod macros; // Ensure this module contains the `entrypoint` macro or imports it correctly
mod vault;
mod handlers;
mod dao;
mod core;
mod staking;

entrypoint!(crate::handlers::process_instructions);

