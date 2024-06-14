use anchor_lang::{prelude::*};
use anchor_lang::{system_program, ToAccountInfo};
use anchor_spl::token::{self, TokenAccount, Token};
// use spl_associated_token_account;
use std::mem::size_of;

pub mod status;
use crate::status::*;

declare_id!("HQW9FafmgcTLLQTjtMaET7ViNiSe5Bk2fEW5jetNivCv");

#[program]
pub mod solana_bridge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("initialize!");
        ctx.accounts.my_storage.owner = ctx.accounts.signer.key();

        Ok(())
    }

    pub fn delete_mystorage(ctx: Context<Delete>) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.signer.key(),
            ctx.accounts.my_storage.owner,
            BridgeError::NotOwner
        );
        msg!("close mystorage!");
        Ok(())
    }

    pub fn modify_owner(ctx: Context<ModifyOwner>, new_owner: Pubkey) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.signer.key(),
            ctx.accounts.my_storage.owner,
            BridgeError::NotOwner
        );

        let my_storage = &mut ctx.accounts.my_storage;
        my_storage.owner = new_owner;
    
        Ok(())
    }

    pub fn register_token(ctx: Context<RegisterToken>, pda_key: String, token_key: String) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.authority.key(),
            ctx.accounts.my_storage.owner,
            BridgeError::NotOwner
        );

        let to_ata = anchor_spl::associated_token::get_associated_token_address(ctx.accounts.pda.key, ctx.accounts.mint.key);
        require_keys_eq!(
            ctx.accounts.to_ata.key(),
            to_ata,
            BridgeError::WrongATA
        );
        
        if ctx.accounts.to_ata.to_account_info().data.borrow().as_ref().len() == 0 {
            msg!{"ATA is not exist, start create"};
            // let ix = spl_associated_token_account::instruction::create_associated_token_account(
            //     ctx.accounts.to_ata.key,
            //     ctx.accounts.pda.key,
            //     ctx.accounts.mint.key,
            //     ctx.accounts.token_program.key,
            // );
            
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: ctx.accounts.authority.to_account_info(),
                    associated_token: ctx.accounts.to_ata.to_account_info(),
                    authority: ctx.accounts.pda.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info()
                }
            );
            anchor_spl::associated_token::create(cpi_ctx)?;

            // token::initialize_account(CpiContext::new(
            //     ctx.accounts.token_program.to_account_info(),
            //     token::InitializeAccount {
            //         account: ctx.accounts.to_ata.to_account_info(),
            //         mint: ctx.accounts.mint.to_account_info(),
            //         authority: ctx.accounts.authority.to_account_info(),
            //         rent: ctx.accounts.authority.to_account_info(),
            //     },
            // ))?;
        
        };
    
        let token = &mut ctx.accounts.token;
        token.token_mint = ctx.accounts.mint.key();
        token.token_ata = to_ata;
        token.amount = 0;

        Ok(())
    }

    pub fn deposit_native(ctx: Context<DepositNativeAccount>, amount :u64, target_chain : String, target_addr: String) -> Result<()>{
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            system_program::Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
            }
        );

        let pda_balance_before = ctx.accounts.to.lamports();               
        system_program::transfer(cpi_context, amount)?;
        let pda_balance_after = ctx.accounts.to.lamports();
        require_eq!(pda_balance_after, pda_balance_before + amount, BridgeError::DepositNE);
        let clock: Clock = Clock::get()?;
        emit!(DepositNative{
            from: *ctx.accounts.from.key, 
            to: *ctx.accounts.to.key, 
            value: amount, 
            chain:target_chain, 
            addr: target_addr,
            time: clock.unix_timestamp});
        Ok(())
    }
 
    pub fn deposit_ft(ctx: Context<DepositTokenAccount>, amount :u64, target_chain : String, target_addr: String) -> Result<()>{
        // 校验ata的地址，确保ATA是由程序的PDA生成
        let to_ata = anchor_spl::associated_token::get_associated_token_address(ctx.accounts.pda.key, ctx.accounts.mint.key);
        // TODO 可能遭受，假地址攻击
        // 这里应该不用处理了，因为会对to_ata会校验，而to_ata是由我们程序pda生成的代币接受地址，这个地址由外部生成后，还会在程序中进行验证。所以就算是假地址，代币还是我们控制的
        // 只需要后端在扫链监听event的时候，根据mint字段来判断，是我们支持的代币，就进行处理，不支持的，我们就不处理即可
        require_keys_eq!(
            to_ata,
            ctx.accounts.to_ata.key(),
            BridgeError::WrongATA
        );
        // 确保ATA地址的Owner是程序PDA地址
        require_keys_eq!(
            ctx.accounts.to_ata.owner.key(),
            ctx.accounts.pda.key(),
            BridgeError::WrongATAOwner
        );
        let pda_balance = ctx.accounts.from_ata.amount;
        require!(pda_balance >=  amount, BridgeError::DepositNE);

        // 构建交易，从用户的ATA转移到程序PDA地址持有的ATA地址
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_context, amount)?;
        let clock: Clock = Clock::get()?;
        emit!(DepositFt{
            from: *ctx.accounts.from.key, 
            to: *ctx.accounts.to.key, 
            mint: ctx.accounts.mint.key(), 
            value: amount, 
            chain:target_chain, 
            addr: target_addr,
            time: clock.unix_timestamp});
        Ok(())
    }

    pub fn withdraw_native(ctx: Context<WithdrawNative>, key: String, amount: u64) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.signer.key(),
            ctx.accounts.my_storage.owner,
            BridgeError::NotOwner
        );

        let (bridge_key, _bump) = Pubkey::find_program_address(&[key.as_bytes().as_ref()], ctx.program_id);
        msg!("find pda address is {}-{}", bridge_key, _bump);
        require_keys_eq!(bridge_key, *ctx.accounts.pda.key, BridgeError::WrongPDA);

        msg!("withdraw native.");
        let pda_balance = ctx.accounts.pda.lamports();
        require!(pda_balance >= amount, BridgeError::WithdrawNE);       

        let seeds: &[&[u8]] = &[key.as_bytes().as_ref(), &[_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 

            system_program::Transfer {
                from: ctx.accounts.pda.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            }
        ).with_signer(signer_seeds);

        let pda_balance_before = ctx.accounts.pda.lamports();               
        system_program::transfer(cpi_context, amount)?;
        let pda_balance_after = ctx.accounts.pda.lamports();
        require_eq!(pda_balance_after, pda_balance_before - amount);

        Ok(())
    }

    pub fn withdraw_ft(ctx: Context<WithdrawFt>, key: String, amount :u64) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.authority.key(),
            ctx.accounts.my_storage.owner,
            BridgeError::NotOwner
        );

        msg!("withdraw ft.");
        // let bridge_key = Pubkey::create_program_address(&[key.as_bytes().as_ref()], ctx.program_id).unwrap();
        let (bridge_key, _bump) = Pubkey::find_program_address(&[key.as_bytes().as_ref()], ctx.program_id);
        msg!("find ft-pda address is {}", bridge_key);
        require_keys_eq!(bridge_key, *ctx.accounts.pda.key, BridgeError::WrongPDA);

        let pda_balance = ctx.accounts.from_ata.amount;
        require!(pda_balance >= amount, BridgeError::WithdrawNE);       

        let seeds: &[&[u8]] = &[key.as_bytes().as_ref(), &[_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = token::Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.pda.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);
        token::transfer(cpi_context, amount)?;

        Ok(())
    }
}

pub fn create_ata(ctx: Context<DepositTokenAccount>) -> Result<()> {
    token::initialize_account(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::InitializeAccount {
            account: ctx.accounts.to_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            rent: ctx.accounts.authority.to_account_info(),
        },
    ))?;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
        payer = signer,
        space = 8 + size_of::<MyStorage>(),
        seeds = [b"bridge_storage".as_ref()],
        bump)]
    pub my_storage: Account<'info, MyStorage>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, close = signer)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyOwner<'info> {
    #[account(mut, seeds = [b"bridge_storage".as_ref()], bump)]
    pub my_storage: Account<'info, MyStorage>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(pda_key: String, token_key: String)]
pub struct RegisterToken<'info> {
    #[account(seeds = [b"bridge_storage".as_ref()], bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(
        init,
        payer = authority,
        space = 8 + size_of::<MyToken>(),
        seeds = [&token_key.as_bytes().as_ref()],
        bump
    )]
    pub token: Account<'info, MyToken>,

    #[account(mut,seeds = [&pda_key.as_bytes().as_ref()],bump)]
    pub pda: SystemAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to_ata: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(key: String)]
pub struct WithdrawNative<'info> {
    #[account(seeds = [b"bridge_storage".as_ref()], bump)]
    pub my_storage: Account<'info, MyStorage>,
    #[account(
        mut,
        seeds = [&key.as_bytes().as_ref()],
        bump
    )]
    pub pda: SystemAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(key: String)]
pub struct WithdrawFt<'info>{
    #[account(seeds = [b"bridge_storage".as_ref()], bump)]
    pub my_storage: Account<'info, MyStorage>,
    #[account(
        mut,
        seeds = [&key.as_bytes().as_ref()],
        bump
    )]
    pub pda: SystemAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: AccountInfo<'info>,
    // #[account(mut)]
    #[account(mut, token::mint = mint, token::authority = pda)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub to_ata: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositNativeAccount<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositTokenAccount<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub to: AccountInfo<'info>,
    pub pda: SystemAccount<'info>,
    /// CHECK: This is not dangerous 
    #[account(mut, token::mint = mint)]
    pub from_ata: Account<'info, TokenAccount>, 
    #[account(mut, token::mint = mint, token::authority = pda)]
    pub to_ata: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
