use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

//This program holds the on-chain logic for making standard & lossless contributions on Mobius Protocol.

#[program]
pub mod Contribution {
    use super::*;
    //Initializing the Contribution Pool and its related accounts.
    pub fn initialize(ctx: Context<Initialize>, nonce: u8) -> Result<()> {

        let contribution_pool = &mut ctx.accounts.contribution_pool;
        
        //Setting Deposit & Contribution counts to zero.
        contribution_pool.total_deposited = 0;
        contribution_pool.total_contributed = 0;

        //Initializing user specific vault accounts, and contribution mints.
        contribution_pool.fundraiser_vault = ctx.accounts.fundraiser_account.key();
        contribution_pool.token_mint = ctx.accounts.token_mint.key();
        contribution_pool.token_vault = ctx.accounts.token_vault.key();
        contribution_pool.user_deposit_count = 0;
        contribution_pool.nonce = nonce;

        Ok(())
    }

    //Creating an account for the contributor.
    pub fn create_contributor(ctx: Context<CreateContributor>, nonce: u8) -> Result<()> {

        let contributor = &mut ctx.accounts.contributor;

        //Initializing contributor specific parameters.
        contributor.contributions = *ctx.accounts.contribution_pool.to_account_info().key;
        contributor.authority = *ctx.accounts.authority.key;
        contributor.balance = 0;
        contributor.nonce = nonce;

        let pool = &mut ctx.accounts.contribution_pool;
        pool.balance = pool.balance.checked_add(amount).unwrap();

        Ok(())
    }

    //Standard Contribution Function: Adds user contribution directly to fundraiser vault to be claimed.
    pub fn standard_contribution(ctx: Context<StdContribution>, amount: u8) -> Result<()> {

        let fundraiser = &mut ctx.accounts.fundraiser_account.key();

            token::transfer(ctx.accounts.transfer_stdcontribute.amount);

        msg!("Successfully Contributed: {}", amount);

        Ok(())
    }

    //Deposit Function: Adds user contributed funds to a token vault, which then deposits into the TulipV1 Lending Market.
    pub fn deposit(ctx: Context<Deposit>, amount: u64, nonce: u8) -> Result<()> {
            
        let contributor = &mut ctx.accounts.contributor;

                //CPI with Token Program to transfer funds to lending market.
                token::transfer(ctx.accounts.transfer_deposit(), amount)?;

            //Print result to console.
            msg!("Successfully Deposited: {}", ctx.accounts.token_vault.amount);
        Ok(())
    }

    //Contribute to Fundraiser Function: Automatically contributes lending returns to selected fundraiser.
    pub fn contribute(ctx: Context<Contribute>, amount: u64, nonce: u8) -> Result<()> {

        let fundraiser = &mut ctx.accounts.fundraiser;

                //CPI with Token Program to transfer funds to fundraiser.
                token::transfer(ctx.accounts.transfer_contribute(), amount)?;
        
            Ok(())

    }

    //Implement Standard Contribution, CPI with Token Program to transfer directly to fundraiser.
    impl<'info> StdContribution<'info> {
        
        fn transfer_deposit(&self) -> CpiContext<'_, '_, '_,'info, Transfer<'info>> {
            CpiContext::new(
                self.token_program.to_account_info(),
                    Transfer {
                        from: self.contributor.to_account_info(),
                        to: self.fundraiser.to_account_info(),
                        authority: self.contributor.to_account_info(),
                    }
            )
        }
    }
    //Implement Deposit Function, CPI with Token Program to transfer to lending market.
    impl<'info> Deposit<'info> {
        
        fn transfer_deposit(&self) -> CpiContext<'_, '_, '_,'info, Transfer<'info>> {
            CpiContext::new(
                self.token_program.to_account_info(),
                    Transfer {
                        from: self.sender_token.to_account_info(),
                        to: self.receiver_token.to_account_info(),
                        authority: self.contributor.to_account_info(),
                    }
            )
        }
    }
     //Implement Contribute to Fundraiser Function, CPI with Token Program to transfer from token vault to fundraiser.
     impl<'info> Contribute<'info> {
            
            fn transfer_contribute(&self) -> CpiContext<'_, '_, '_,'info, Transfer<'info>> {
                CpiContext::new(
                    self.token_program.to_account_info(),
                        Transfer {
                            from: self.token_vault.to_account_info(),
                            to: self.fundraiser.to_account_info(),
                            authority: self.contributor.to_account_info(),
                        }
                )
            }
        }

}

//Derived account validation

#[derive(Accounts)]
pub struct Initialize {
    
    #[account(zero)]
    pub contribution_pool: Account<'info>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut, 
        constraint = token_vault.mint == token_mint.key(),
        constraint = token_vault.owner == signer.key(),
    )]
    pub token_vault: Account<'info, TokenAccount>,

    #[account(mut, 
        constraint = contribution_vault.mint == token_mint.key(),
        constraint = contribution_vault.owner == fundraiser.key(),
    )]
    pub contribution_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            signer.to_account_info().key.as_ref()
        ]
    )]
    pub signer: UncheckedAccount<'info>

}

#[derive(Accounts)]
pub struct StdContribution {
    
    //Input

}

#[derive(Accounts)]
pub struct CreateContributor {

//Input

}


#[derive(Accounts)]
pub struct Deposit {
    
    #[account(init, payer = contributor, space 8 + 16 + 124)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub token_vault: Account<'info>,
    #[account(mut)]
    pub receiver_token: Account<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program <'info, Token>,
    pub system_program: System <'info, System>,
}

#[derive(Accounts)]
pub struct Contribute {
    
    #[account(init, payer = user, space 8+8)]
    pub contributor: Signer<'info>,
    #[account(mut)]
    pub contribution_pool: Box<Account<'info, ContributionPool>>,
    pub fundraiser: Account<'info>,
    pub token_vault: Account<'info>,
    pub sender_token: Account<'info, Token>,
    #[account(mut)]
    pub receiver_token: Account<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program:  Program<'info, System>,
}

//Additional typing and account validation.

#[account]
#[derive(Default)]
pub struct Contributor {
    
    pub contribution_pool: Pubkey,
    pub authority: Pubkey,
    pub contributed: u64,
    pub nonce: u8,
}

#[account]
pub struct ContributionPool {
   
    // User
    pub contributor: Pubkey,
    pub fundraiser: Pubkey,
    pub token_vault: Mint,
    pub receiver_token: Mint,
    // Token Mint
    pub mint: Pubkey,
    // User's contribution count
    pub contributed: u64,
    pub token_program: Pubkey,
}

//Error Codes.
#[error]
pub enum ErrorCode {
    
    #[msg("Insufficient funds to unstake.")]
    InsufficientFundUnstake,
    #[msg("Amount must be greater than zero.")]
    AmountMustBeGreaterThanZero,
}