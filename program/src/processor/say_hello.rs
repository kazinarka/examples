use solana_program::entrypoint::ProgramResult;
use solana_program::msg;

pub fn say_hello() -> ProgramResult {
    msg!("Hello");

    Ok(())
}
