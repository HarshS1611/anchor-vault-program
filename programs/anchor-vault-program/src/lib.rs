use anchor_lang::prelude::*;

declare_id!("H39GF6qRfe9yeg2mADACwBn5EqyaJFfh62nFVa7mNM3z");

#[program]
pub mod anchor_vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
