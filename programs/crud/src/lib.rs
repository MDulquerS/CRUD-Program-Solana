use anchor_lang::prelude::*;

declare_id!("Mx2KcqEtge1DbkVTNC3oyM1Bmbvd8eXfADUWxegdvGy");

#[program]
pub mod crud {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
