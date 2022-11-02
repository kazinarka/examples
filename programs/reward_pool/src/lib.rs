pub mod consts;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("5wJ919qPv4V9GXm7aFJtPMFYnDZhz3GuTkHKk4yHuTnp");

pub type Timestamp = u64;
