#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("izdLJcDdkWjMzx9sRbbmbpB8tvbk1gn9EnkcFjAQg6D");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;
        poll.candidate_amount = 0;
        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        candidate_name: String,
        _poll_id: u64,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate;
        candidate.candidate_name = candidate_name;
        candidate.candidate_votes = 0;
        Ok(())
    }

    pub fn initialize_voter(ctx: Context<InitializeVoter>) -> Result<()> {
        let voter = &mut ctx.accounts.voter;
        voter.has_voted = false; // Default state is that the voter has not voted
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _candidate_name: String, _poll_id: u64) -> Result<()> {
        let voter = &mut ctx.accounts.voter;

        if voter.has_voted {
            return Err(VotingError::AlreadyVoted.into());
        }

        voter.has_voted = true;

        let candidate = &mut ctx.accounts.candidate;
        candidate.candidate_votes += 1;

        msg!("Voted for candidate: {}", candidate.candidate_name);
        msg!("Votes: {}", candidate.candidate_votes);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
      )]
    pub poll: Account<'info, Poll>,

    #[account(
      mut,
      seeds = [poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
      bump
    )]
    pub voter: Account<'info, Voter>,

    #[account(
      mut,
      seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
      bump
    )]
    pub candidate: Account<'info, Candidate>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(candidate_name: String, poll_id: u64)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
      )]
    pub poll: Account<'info, Poll>,

    #[account(
      init,
      payer = signer,
      space = 8 + Candidate::INIT_SPACE,
      seeds = [poll_id.to_le_bytes().as_ref(), candidate_name.as_ref()],
      bump
    )]
    pub candidate: Account<'info, Candidate>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      init,
      payer = signer,
      space = 8 + Poll::INIT_SPACE,
      seeds = [poll_id.to_le_bytes().as_ref()],
      bump
    )]
    pub poll: Account<'info, Poll>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Voter {
    pub has_voted: bool,
}

impl Voter {
    pub const INIT_SPACE: usize = 8; 
}

#[error_code]
pub enum VotingError {
    #[msg("The signer has already voted in this poll.")]
    AlreadyVoted,
}

#[account]
#[derive(InitSpace)]
pub struct Poll {
    pub poll_id: u64,
    #[max_len(200)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializeVoter<'info> {  // Fixing typo here
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
    init,
    space = 8 + Voter::INIT_SPACE,
    payer = signer,
    seeds = [poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub voter: Account<'info, Voter>,

    pub system_program: Program<'info, System>,
}
