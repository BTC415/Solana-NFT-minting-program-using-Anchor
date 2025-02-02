use anchor_lang::prelude::*;

declare_id!("GUPgmjutVzrin2XorvkteHZK1SRhohpTLkAabkh3r7bQ");

#[program]
pub mod solana_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
