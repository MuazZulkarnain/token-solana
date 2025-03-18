use anchor_lang::prelude::*;

declare_id!("Bpu4rTuAdaczTeiwMg9TXbMe5S4gDFNn8DfioWqsXRRQ");

#[program]
pub mod token_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
