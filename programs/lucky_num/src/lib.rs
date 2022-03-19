use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction::transfer;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::clock::Clock;
use std::convert::TryInto;

// use rand::Rng;
// solana address -k target/deploy/myepicproject-keypair.json <------
declare_id!("8BQunCRwyBZ3EaPP4bmCraTWhVvazrajNR3CtTM7P5rT");

#[program]
pub mod lucky_num {

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
        ctx.accounts.vault.amount += stake;

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
            // let r = since_the_epoch.subsec_nanos() as u64 / 1_000_000;
            let r = Clock::get()?.unix_timestamp;
            let lucky_number: u8 = (r % 3 + 1) as u8;
            // let lucky_number: u8 = temp.to_string().chars().find(|x| x.is_digit(10)).unwrap().to_digit(10).unwrap().try_into().unwrap();
            // let lucky_number: u8 = 5;
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
                Exchange::transfer_from_account_we_own(&mut ctx.accounts.vault.to_account_info().clone(),&mut i.clone(), share)?;
            //     let ix = transfer(&ctx.accounts.vault.key(),&i.key, share);
            //     invoke_signed(
            //     &ix,
            //         &[
            //             ctx.accounts.vault.to_account_info().clone(),
            //             i.clone(),
            //         ],&[&[b"pubkey", &[ctx.accounts.vault.bump]]]
            //     )?;
            // };
            // Stack contain all winning transaction
        }
        // Close and empty PDA
        ctx.accounts.vault.amount = 0;
        ctx.accounts.vault.bump = 0;
        Exchange::transfer_from_account_we_own(&mut ctx.accounts.vault.to_account_info().clone(),&mut ctx.accounts.player_one.clone(), ctx.accounts.vault.to_account_info().lamports())?;
        // Close and empty game_info account
        // Close and empty all participant account
        }
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(stake: u64, max_participants: u8, lucky_num: u8, vault_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, 
        // space = Game::LEN + (usize::from(max_participants) * Game::PACTICIPANT_LEN))]
        space = {
            let space = (Game::LEN as u64).wrapping_add((max_participants as u64).wrapping_mul(Game::PACTICIPANT_LEN as u64)).try_into().unwrap();
            space
          })]  
    pub game_info: Account<'info, Game>,
    #[account(init, payer = initializer, space = Vault::LEN, seeds = [b"pubkey"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut, constraint = initializer.lamports() >= game_info.stake)]
    pub initializer: Signer<'info>,
    pub system_program: Program <'info, System>
}

#[derive(Accounts)]
#[instruction(stake: u64, lucky_num: u8)]
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
    #[account(mut)]
    pub player_one: AccountInfo<'info>,
    #[account(mut)]
    pub player_two: AccountInfo<'info>,
    #[account(mut)]
    pub player_three: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"pubkey"], bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        constraint = game_info.max_participants == game_info.participant_list.len().try_into().unwrap())]
    pub game_info: Account<'info, Game>,
    pub system_program: Program<'info, System>
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

// 2. Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STAKE_LENGTH: usize = 8;
const MAX_PARTICIPANT_LENGTH: usize = 1;
const PACTICIPANT_LEN_PREFIX: usize = 4; // Stores the size of the string.
const PARTICIPANT_STRUCT_LEN: usize = 32 + 1; // 50 chars max.

// 3. Add a constant on the Tweet account that provides its total size.
impl Game {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Initializer.
        + STAKE_LENGTH // Stake.
        + MAX_PARTICIPANT_LENGTH; // Number of participants.
    const PACTICIPANT_LEN: usize = DISCRIMINATOR_LENGTH + PACTICIPANT_LEN_PREFIX + PUBLIC_KEY_LENGTH + 1;
}

impl Vault {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + 8 // Amount.
        + 1; // Bump.
}

impl<'info> Exchange<'info> {
    fn transfer_from_account_we_own(
        src: &mut AccountInfo, // we better own this account though
        dst: &mut AccountInfo,
        amount: u64,
    ) -> ProgramResult {
        **src.try_borrow_mut_lamports()? = src
            .lamports()
            .checked_sub(amount)
            .ok_or(ProgramError::InvalidArgument)?;
        **dst.try_borrow_mut_lamports()? = dst
            .lamports()
            .checked_add(amount)
            .ok_or(ProgramError::InvalidArgument)?;
        Ok(())
    }
}