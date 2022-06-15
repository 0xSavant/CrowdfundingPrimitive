use anchor_lang::prelude::*;
use anchor_lang::prelude::ErrorCode;
use anchor_lang::solana_program::program::{invoke};
use anchor_lang::solana_program::{clock, program_option::COption, sysvar};
use anchor_lang::solana_program::program_error::ProgramError;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::pubkey::Pubkey;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod CreateFundraiser {
    use super::*;
    pub fn initialize(ctx: Context<InitializeFundraiser>, update_account_bump: u8) -> Result<()> {
        
        let update_account = &mut ctx.accounts.update_account;
        let authority = &mut ctx.accounts.authority;
         
        //Initialize Update Account and required fields
        update_account.bump = update_account_bump;
        update_account.authority = *authority.key; 
        update_account.fundraiser_data = Vec::new();
        update_account.contribution_received = Vec::new();
        update_account.count = 0;
        
        Ok(())
    }

    pub fn createfundraiser(ctx: Context<Create>, name: String, description: String, image: String, update_account_bump: u8) -> Result<()> {

        let update_account = &mut ctx.accounts.update_account;
        let authority = &mut ctx.accounts.authority;

        //Create PDA
        let (pda, bump ) = Pubkey::find_program_address
        (
            &[b"ILikeTurtles", 
            &*authority.key().as_ref()], 
            &self::ID
        );

        // Validate PDA authority is passed in
        if pda != update_account.key()  {                       
            return Err(ProgramError::InvalidAuthority)
        };

        if bump != update_account_bump {
            return Err(ProgramError::Custom(1))
        };

        //Name length can only be 30 characters max, or throws an error
        if name.len()  > 30 {
            return Err(ErrorCode::NameTooLong.into())
        };

        //Description length can only be 150 characters max, throws an error
        if description.len() > 150 {
            return Err(ErrorCode::DescriptionTooLong.into())
        };

        //Define relevant fundraiser data
        let fundraiser_data = FundraiserData {
            owner: *authority.key,
            name: name.to_string(),
            description: description.to_string(),
            image: image.to_string(),
        };

        update_account.fundraiser_state.push(fundraiser_data);
    }


    pub fn withdraw(ctx: Context<Withdraw>, amount: u8, update_account_bump: u8) -> Result<()> {
        let update_account = &mut ctx.accounts.update_account;
        let authority = &mut ctx.accounts.authority;

        let (pda, bump ) = Pubkey::find_program_address
        (
            &[b"ILikeTurtles", 
            &*authority.key().as_ref()], 
            &self::ID
        );

        // Validate PDA authority is passed in
        if pda != update_account.key()  {                       
            return Err(ProgramError::InvalidAuthority)
        };

        if bump != update_account_bump {
            return Err(ProgramError::Custom(1))
        };

        **update_account.to_account.info().try_borrow_mut_lamports()? -= amount;
        **authority.to_account.info().try_borrow_mut_lamports()? -= amount;

        let withdraw_data = WithdrawOrder {
            withdraw_amount: amount,
            beneficiary: *authority.to_account_info().key,
        };

        update_account.withdraw_order.push(withdraw_data);

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(update_account_bump: u8)]
pub struct InitializeFundraiser<'info> {
    #[account(
        init, 
        payer = authority, 
        seeds = [b"ILikeTurtles".as_ref(), authority.key().as_ref()],
        bump,
        space = 8 + 512
        )]
    pub update_account: Account<'info, FundraiserState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut, has_one = authority)]
    pub update_account: Account<'info, FundraiserState>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = authority)]
    pub update_account: Account<'info, FundraiserState>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct FundraiserData {
    pub authority: Pubkey,
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub raised: u64,
    pub bump: u8,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawOrder {
    pub withdraw_amount: u64,
    pub authority: Pubkey,
}

#[account]
pub struct FundraiserState {
    pub fundraiser_data: Vec<FundraiserData>,
    pub raised: u64,
    pub bump: u8,
    pub authority: Pubkey,
    pub withdraw_order: Vec<WithdrawOrder>,
}

//Error Codes
#[error_code]
pub enum Errors {
    #[msg("Update Authority Invalid.")]
        InvalidAuthority,
    #[msg("Name has a limit of 30 characters.")]
        NameTooLong,
    #[msg("Description has a limit of 150 characters.")]
        DescriptionTooLong,
}