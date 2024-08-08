mod macros;
mod vault;
mod handlers;

entrypoint!(crate::handlers::process_instructions);