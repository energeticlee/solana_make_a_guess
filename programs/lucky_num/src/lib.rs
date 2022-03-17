use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction::{transfer, create_account};
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use std::convert::TryInto;
// use rand::Rng;
// solana address -k target/deploy/myepicproject-keypair.json <------
declare_id!("8BQunCRwyBZ3EaPP4bmCraTWhVvazrajNR3CtTM7P5rT");

#[program]
pub mod lucky_num {
    use anchor_lang::solana_program::instruction::Instruction;

    use super::*;
    pub fn initialize(ctx: Context<Initialize>, stake: u64, max_participants: u8, lucky_num: u8, vault_bump: u8) -> ProgramResult {
        ctx.accounts.game_info.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts.game_info.stake = stake;
        ctx.accounts.game_info.max_participants = max_participants;
        ctx.accounts.vault.amount += stake;
        ctx.accounts.vault.bump = vault_bump;

        // let (vault_account, _) =
        // Pubkey::find_program_address(&[b"pubkey"], ctx.program_id);
        
        let ix = transfer(&ctx.accounts.initializer.key,&ctx.accounts.vault.key(), stake);
        invoke(
            &ix,
            &[
                ctx.accounts.initializer.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;
    
        // Create participants account
        let participant = Participant {
            participant_address: *ctx.accounts.initializer.key,
            lucky_num
        };

        ctx.accounts.game_info.participant_list.push(participant);
        ctx.accounts.game_info.vault_address = ctx.accounts.vault.key();
        
        Ok(())
    }
    
    pub fn participate(ctx: Context<Participate>, stake: u64, lucky_num: u8) -> ProgramResult {
        let ix = transfer(&ctx.accounts.participant_account.key,&ctx.accounts.vault.key(), stake);
        invoke(
            &ix,
            &[
                ctx.accounts.participant_account.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        // Create participants account
        let participant = Participant {
            participant_address: *ctx.accounts.participant_account.key,
            lucky_num
        };
        let game_info = &mut ctx.accounts.game_info;

        game_info.participant_list.push(participant);
        Ok(())
    }
    pub fn exchange(ctx: Context<Exchange>) -> ProgramResult {
        // Get all participant account where vault_address == vault_account
        if ctx.accounts.game_info.participant_list.len() == ctx.accounts.game_info.max_participants.try_into().unwrap() {
            // Roll die
            // let mut rng = rand::thread_rng();
            // let lucky_number: u8 = rng.gen_range(1..7);
            let lucky_number: u8 = 5;
            
            // Check how many correct guess
            let winners_pubkey: Vec<Pubkey> = ctx.accounts.game_info.participant_list.iter().fold([].to_vec(), |mut acc, val| {if val.lucky_num == lucky_number {acc.push(val.participant_address)}; acc});
            
            // Check if accountInfo is winners
            let player_accounts = &[&ctx.accounts.player_one.to_account_info(), &ctx.accounts.player_two.to_account_info(), &ctx.accounts.player_three.to_account_info()];
            
            let winning_accounts = player_accounts.iter().fold([].to_vec(), |mut arr, acc_info| {if winners_pubkey.iter().any(|&i| i==*acc_info.key) {arr.push(*acc_info)}; arr});

            // Divide PDA lamports by number of correct guess
            let share: u64 = &ctx.accounts.vault.amount / winners_pubkey.len() as u64;
            
            // For loop to transfer fund to winner
            // Require account ID

            for i in winning_accounts {
                let ix = transfer(&ctx.accounts.vault.key(),&i.key, share);
                invoke_signed(
                &ix,
                    &[
                        ctx.accounts.vault.to_account_info().clone(),
                        i.clone(),
                    ],&[&[b"pubkey", &[ctx.accounts.vault.bump]]]
                )?;
            };
            // Stack contain all winning transaction
            
            // Close and empty PDA
            // Close and empty game_info account
            // Close and empty all participant account
        }
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 9000)]
    pub game_info: Account<'info, Game>,
    #[account(init, payer = initializer, space = 9000, seeds = [b"pubkey"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        constraint = initializer.lamports() >= game_info.stake
    )]
    pub initializer: Signer<'info>,
    pub system_program: Program <'info, System>
}

#[derive(Accounts)]
#[instruction(stake: u64)]
pub struct Participate<'info> {
    #[account(
        mut,
        constraint = participant_account.lamports() >= stake
    )]
    pub participant_account: Signer<'info>,
    #[account(
        mut,
        seeds = [b"pubkey"], bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        constraint = game_info.max_participants > game_info.participant_list.len().try_into().unwrap())]
    pub game_info: Account<'info, Game>,
    pub system_program: Program <'info, System>
}
// What does ctx.remaining_accounts do?
//https://discord.com/channels/889577356681945098/889702325231427584/954007004714786826
//https://book.anchor-lang.com/chapter_3/the_program_module.html?highlight=remaining_accounts#context

#[derive(Accounts)]
pub struct Exchange<'info> {
    pub player_one: AccountInfo<'info>,
    pub player_two: AccountInfo<'info>,
    pub player_three: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"pubkey"], bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        constraint = game_info.max_participants == game_info.participant_list.len().try_into().unwrap())]
    pub game_info: Account<'info, Game>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Participant {
    pub participant_address: Pubkey,
    pub lucky_num: u8
}

#[account]
pub struct Game {
    pub initializer_key: Pubkey,
    pub vault_address: Pubkey,
    pub stake: u64,
    pub max_participants: u8,
    pub participant_list: Vec<Participant>
}

#[account]
pub struct Vault {
    pub amount: u64,
    pub bump: u8
}