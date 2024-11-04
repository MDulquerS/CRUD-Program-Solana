use anchor_lang::prelude::*;

declare_id!("Mx2KcqEtge1DbkVTNC3oyM1Bmbvd8eXfADUWxegdvGy");

#[program]
pub mod crud {
    use super::*;

    pub fn create_journal_state(ctx: Context<CreateEntry>, title:String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key.as_ref()],
        bump,
         space = 8 + JournalEntryState::INIT_SPACE,
         payer = owner,
    )]
    pub journal_entry : Account<'info, JournalEntryState>,
    #[account(mut)]
    pub owner:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState{
    pub owner: PubKey,
    #[max_len(50)]
    pub title: String,
    #[max_len(1000)]
    pub message: String,

}
