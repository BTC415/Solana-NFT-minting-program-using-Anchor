use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::{
    create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3, CreateMedatadataAccountsV3 ,Metadata
};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::state::{Collection, Creator, DataV2};

declare_id!("GUPgmjutVzrin2XorvkteHZK1SRhohpTLkAabkh3r7bQ");

#[program]
pub mod solana_nft {
    use super::*;


}

#[derive(Accounts)]
pub struct CreateNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[acocunt(init, payer=payer, mint::decimals=0, mint::authority=authority, mint::freeze_authority=authority, seeds = ["mint".as_bytes(), id.to_le_bytes().as_ref()], bump)]
    pub mint: Account<'info, Mint>,
    #[account(init_if_needed, payer=payer, associated_token::mint=mint, associated_token::authority=authority)]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    #[account(mut, seeds=[b"metadata".as_ref(), metadata_program.key().as_ref(), mint.key().as_ref(), b"edition".as_ref()], bump, seeds::program=metadata_program.key())]
    pub master_edition_account:UncheckedAccount<'info>,
    #[acocunt(mut, seeds=[b"metadata".as_ref(), metadata_program.key().as_ref(), mint.key().as_ref()], bump, seeds::program=metadata_program.key())]
    pub nft_metadata:UncheckedAccount<'info>,
}