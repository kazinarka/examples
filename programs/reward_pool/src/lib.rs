pub mod consts;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("7CL7BEKMd5BuJqzVbwXUAtwjRCFN9EjrQXcfgLJedpjm");

pub type Timestamp = u64;
