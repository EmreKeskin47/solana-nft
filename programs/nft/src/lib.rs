use {
    anchor_lang::{prelude::*, solana_program::program::invoke, system_program},
    anchor_spl::{associated_token, token},
    mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID},
};

declare_id!("62eUAMqPKSQvG4M7CfyfNqygpcfMVAob4XRRf9DbYk6J");

#[program]
pub mod nft {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        msg!("Mint account: {}", &ctx.accounts.mint.key());
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000,
            82,
            &ctx.accounts.token_program.key(),
        )?;

        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;

        msg!(
            "Token Account Address: {}",
            &ctx.accounts.token_account.key()
        );
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.mint_authority.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ))?;

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!(
            "Metadata account: {}",
            &ctx.accounts.metadata.to_account_info().key()
        );
        invoke(
            &token_instruction::create_metadata_accounts_v3(
                TOKEN_METADATA_ID,
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                metadata_title,
                metadata_symbol,
                metadata_uri,
                None,
                1,
                true,
                false,
                None,
                None,
                None,
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!(
            "Master edition metadata account: {}",
            &ctx.accounts.master_edition.to_account_info().key()
        );
        invoke(
            &token_instruction::create_master_edition_v3(
                TOKEN_METADATA_ID,
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint_authority.key(),
                Some(0),
            ),
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        Ok(())
    }
    pub fn sell(ctx: Context<SellNft>, sale_lamports: u64) -> Result<()> {
        msg!("Initiating transfer of {} lamports...", sale_lamports);
        msg!(
            "Purchaser (sending lamports): {}",
            &ctx.accounts.buyer_authority.key()
        );
        msg!(
            "Seller (receiving lamports): {}",
            &ctx.accounts.owner_authority.key()
        );
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer_authority.to_account_info(),
                    to: ctx.accounts.owner_authority.to_account_info(),
                },
            ),
            sale_lamports,
        )?;

        msg!("Lamports transferred successfully.");

        msg!("Creating buyer token account...");
        msg!(
            "Buyer Token Address: {}",
            &ctx.accounts.buyer_token_account.key()
        );
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ))?;

        msg!("Transferring NFT...");
        msg!(
            "Owner Token Address: {}",
            &ctx.accounts.owner_token_account.key()
        );
        msg!(
            "Buyer Token Address: {}",
            &ctx.accounts.buyer_token_account.key()
        );
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.owner_token_account.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.owner_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("NFT transferred successfully.");

        msg!("Sale completed successfully!");

        Ok(())
    }

    pub fn transfer_nft(ctx: Context<TransferNft>) -> Result<()> {
        msg!("Initiating NFT transfer...");
        msg!("Sender: {}", &ctx.accounts.owner_authority.key());
        msg!("Receiver: {}", &ctx.accounts.buyer_authority.key());

        msg!("Creating receiver's token account...");
        msg!(
            "Receiver Token Address: {}",
            &ctx.accounts.buyer_token_account.key()
        );
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.buyer_authority.to_account_info(),
                associated_token: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.buyer_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ))?;

        msg!("Transferring NFT...");
        msg!(
            "Sender Token Address: {}",
            &ctx.accounts.owner_token_account.key()
        );
        msg!(
            "Receiver Token Address: {}",
            &ctx.accounts.buyer_token_account.key()
        );
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.owner_token_account.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.owner_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("NFT transferred successfully.");

        Ok(())
    }

    pub fn burn_nft(ctx: Context<BurnNft>) -> Result<()> {
        msg!("Initiating NFT burn...");
        msg!("Owner: {}", &ctx.accounts.owner_authority.key());
        msg!("Token Address: {}", &ctx.accounts.owner_token_account.key());

        msg!("Burning NFT...");
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.owner_authority.to_account_info(),
                    from: ctx.accounts.owner_token_account.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("NFT burned successfully.");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: will be created with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: will be created with anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: will be created with Metaplex
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct SellNft<'info> {
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub owner_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub owner_authority: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub buyer_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

#[derive(Accounts)]
pub struct BurnNft<'info> {
    /// CHECK: will be created with anchor
    #[account(signer)]
    pub owner_authority: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    #[account(mut)]
    pub owner_token_account: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    pub mint: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferNft<'info> {
    /// CHECK: will be created with anchor

    #[account(signer)]
    pub owner_authority: AccountInfo<'info>,
    /// CHECK: will be created with anchor

    #[account(mut)]
    pub owner_token_account: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    #[account(signer)]
    pub buyer_authority: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    #[account(mut)]
    pub buyer_token_account: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    pub mint: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    pub system_program: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    pub token_program: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: will be created with anchor
    pub rent: AccountInfo<'info>,
}
