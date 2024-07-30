use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
// use pyth_sdk_solana::state::PriceStatus;
// use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};

declare_id!("HB5YUkkQ15LPEqE5sBaF3BsWNjHBqB1HzZbiNiLv7ufK");

#[program]
pub mod neurolov_presale {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        start_time: i64,
        end_time: i64,
        token_amount: u64,
        token_price: u64,
    ) -> Result<()> {
        let presale = &mut ctx.accounts.presale;
        if start_time >= end_time {
            return Err(ErrorCode::InvalidTimeRange.into());
        }

        presale.start_time = start_time;
        presale.end_time = end_time;
        presale.token_mint = ctx.accounts.token_mint.key();
        presale.presale_token_account = ctx.accounts.presale_token_account.key();
        presale.presale_supply = token_amount;
        presale.token_price = token_price; // Price in lamports per NLOV token
        presale.total_contributed = 0;
        presale.is_active = true;
        presale.is_paused = false;
        presale.owner = *ctx.accounts.owner.key;
        presale.public_sale_end_time = end_time + 3 * 7 * 24 * 60 * 60; // 3 weeks after presale end

        // Transfer tokens to the presale account
        let cpi_accounts = Transfer {
            from: ctx.accounts.owner_token_account.to_account_info(),
            to: ctx.accounts.presale_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, token_amount)?;

        emit!(PresaleInitialized {
            start_time,
            end_time,
            token_amount,
            token_price,
        });
        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> Result<()> {
        let presale = &mut ctx.accounts.presale;
        require!(!presale.is_paused, ErrorCode::PresalePaused);

        let now = Clock::get()?.unix_timestamp;
        require!(
            now >= presale.start_time && now <= presale.end_time,
            ErrorCode::PresaleNotActive
        );

        // Simplified price calculation (1 SOL = 25 NLOV tokens)
        let nlov_amount = amount.checked_mul(25).ok_or(ErrorCode::CalculationError)?;

        require!(
            nlov_amount >= 1 && nlov_amount <= presale.presale_supply - presale.total_contributed,
            ErrorCode::InvalidAmount
        );

        presale.total_contributed += nlov_amount;

        // Transfer SOL from user to presale account
        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            ctx.accounts.user.key,
            ctx.accounts.presale_account.key,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.presale_account.to_account_info(),
            ],
        )?;

        // Record user contribution
        let user_info = &mut ctx.accounts.user_info;
        user_info.user = *ctx.accounts.user.key;
        user_info.amount_contributed += nlov_amount;

        emit!(ContributionMade {
            user: *ctx.accounts.user.key,
            sol_amount: amount,
            nlov_amount,
            total_contributed: presale.total_contributed,
        });

        Ok(())
    }
    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        let presale = &ctx.accounts.presale;
        require!(!presale.is_paused, ErrorCode::PresalePaused);

        let user_info = &ctx.accounts.user_info;
        let now = Clock::get()?.unix_timestamp;

        require!(
            now > presale.public_sale_end_time,
            ErrorCode::ClaimingNotAvailable
        );

        let amount_to_claim = user_info.amount_contributed - user_info.amount_claimed;
        require!(amount_to_claim > 0, ErrorCode::NothingToClaim);

        // Transfer tokens from presale account to user
        let cpi_accounts = Transfer {
            from: ctx.accounts.presale_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.presale.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let seeds = &[b"presale".as_ref(), &[ctx.bumps.presale]];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount_to_claim)?;

        // Update user info
        let user_info = &mut ctx.accounts.user_info;
        user_info.amount_claimed += amount_to_claim;

        emit!(TokensClaimed {
            user: *ctx.accounts.user.key,
            amount: amount_to_claim,
        });

        Ok(())
    }

    pub fn pause(ctx: Context<PauseUnpause>) -> Result<()> {
        let presale = &mut ctx.accounts.presale;
        require!(
            presale.owner == *ctx.accounts.owner.key,
            ErrorCode::Unauthorized
        );
        require!(!presale.is_paused, ErrorCode::AlreadyPaused);

        presale.is_paused = true;
        emit!(PresalePaused {});
        Ok(())
    }

    pub fn unpause(ctx: Context<PauseUnpause>) -> Result<()> {
        let presale = &mut ctx.accounts.presale;
        require!(
            presale.owner == *ctx.accounts.owner.key,
            ErrorCode::Unauthorized
        );
        require!(presale.is_paused, ErrorCode::NotPaused);

        presale.is_paused = false;
        emit!(PresaleUnpaused {});
        Ok(())
    }

    pub fn finalize_presale(ctx: Context<FinalizePresale>) -> Result<()> {
        let presale = &mut ctx.accounts.presale;
        require!(
            presale.owner == *ctx.accounts.owner.key,
            ErrorCode::Unauthorized
        );
        require!(presale.is_active, ErrorCode::PresaleAlreadyFinalized);

        let now = Clock::get()?.unix_timestamp;
        require!(now > presale.end_time, ErrorCode::PresaleStillActive);

        presale.is_active = false;
        emit!(PresaleFinalized {
            total_contributed: presale.total_contributed,
            end_time: presale.end_time,
        });
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let presale = &ctx.accounts.presale;
        require!(
            presale.owner == *ctx.accounts.owner.key,
            ErrorCode::Unauthorized
        );
        require!(!presale.is_active, ErrorCode::PresaleStillActive);

        let presale_balance = ctx.accounts.presale_account.lamports();
        require!(amount <= presale_balance, ErrorCode::InsufficientFunds);

        **ctx.accounts.presale_account.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.owner.try_borrow_mut_lamports()? += amount;

        emit!(FundsWithdrawn {
            owner: *ctx.accounts.owner.key,
            amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 256, seeds = [b"presale"], bump)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub presale_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Contribute<'info> {
    #[account(mut, seeds = [b"presale"], bump)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub presale_account: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8,
        seeds = [b"user_info", presale.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut, seeds = [b"presale"], bump)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub presale_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"user_info", presale.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct PauseUnpause<'info> {
    #[account(mut, seeds = [b"presale"], bump)]
    pub presale: Account<'info, Presale>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct FinalizePresale<'info> {
    #[account(mut, seeds = [b"presale"], bump)]
    pub presale: Account<'info, Presale>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(seeds = [b"presale"], bump)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub presale_account: SystemAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Presale {
    pub start_time: i64,
    pub end_time: i64,
    pub public_sale_end_time: i64,
    pub token_mint: Pubkey,
    pub presale_token_account: Pubkey,
    pub presale_supply: u64,
    pub token_price: u64,
    pub total_contributed: u64,
    pub is_active: bool,
    pub is_paused: bool,
    pub owner: Pubkey,
}

#[account]
pub struct UserInfo {
    pub user: Pubkey,
    pub amount_contributed: u64,
    pub amount_claimed: u64,
}

#[event]
pub struct PresaleInitialized {
    pub start_time: i64,
    pub end_time: i64,
    pub token_amount: u64,
    pub token_price: u64,
}

#[event]
pub struct ContributionMade {
    pub user: Pubkey,
    pub sol_amount: u64,
    pub nlov_amount: u64,
    pub total_contributed: u64,
}

#[event]
pub struct TokensClaimed {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct PresalePaused {}

#[event]
pub struct PresaleUnpaused {}

#[event]
pub struct PresaleFinalized {
    pub total_contributed: u64,
    pub end_time: i64,
}

#[event]
pub struct FundsWithdrawn {
    pub owner: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Presale is not active.")]
    PresaleNotActive,
    #[msg("Amount is invalid.")]
    InvalidAmount,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Invalid start and end times.")]
    InvalidTimeRange,
    #[msg("Presale supply exceeded.")]
    ExceedsPresaleSupply,
    #[msg("Claiming is not available yet.")]
    ClaimingNotAvailable,
    #[msg("Nothing to claim.")]
    NothingToClaim,
    #[msg("Error fetching Pyth price")]
    PythError,
    #[msg("Invalid Pyth price")]
    InvalidPythPrice,
    #[msg("Presale is paused")]
    PresalePaused,
    #[msg("Presale is already paused")]
    AlreadyPaused,
    #[msg("Presale is not paused")]
    NotPaused,
    #[msg("Presale is still active")]
    PresaleStillActive,
    #[msg("Presale has already been finalized")]
    PresaleAlreadyFinalized,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Calculation error")]
    CalculationError,
}
