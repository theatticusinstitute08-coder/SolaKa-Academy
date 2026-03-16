use anchor_lang::prelude::*;

// This is a placeholder ID. You will get a real one during deployment.
declare_id!("11111111111111111111111111111111");

#[program]
pub mod solaka {
    use super::*;

    // Initializes a new learner profile on the blockchain
    pub fn initialize_learner(ctx: Context<Initialize>) -> Result<()> {
        let learner_progress = &mut ctx.accounts.learner_progress;
        learner_progress.owner = *ctx.accounts.user.key;
        learner_progress.completed_modules_mask = 0;
        learner_progress.verification_hash = 0;
        Ok(())
    }

    // Records the completion of a specific module using XOR-fold verification
    pub fn complete_module(ctx: Context<CompleteModule>, module_id: u8, task_hash: u64) -> Result<()> {
        let learner_progress = &mut ctx.accounts.learner_progress;
        
        // Logical XOR-fold: ensures a unique sequence of completed tasks
        learner_progress.verification_hash ^= task_hash;
        
        // Bitwise OR: marks the specific module_id as completed (0-7)
        learner_progress.completed_modules_mask |= 1 << module_id;

        msg!("Module {} marked as complete. Progress updated.", module_id);
        Ok(())
    }
}

#[account]
pub struct LearnerProgress {
    pub owner: Pubkey,               // 32 bytes
    pub completed_modules_mask: u8,  // 1 byte (up to 8 modules)
    pub verification_hash: u64,      // 8 bytes (XOR-fold result)
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + 32 + 1 + 8, 
        seeds = [b"progress", user.key().as_ref()], 
        bump
    )]
    pub learner_progress: Account<'info, LearnerProgress>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteModule<'info> {
    #[account(
        mut, 
        has_one = owner,
        seeds = [b"progress", owner.key().as_ref()], 
        bump
    )]
    pub learner_progress: Account<'info, LearnerProgress>,
    pub owner: Signer<'info>,
}
