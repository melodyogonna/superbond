use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{Mint, Token},
};

declare_id!("Afaffb9YSmP65WbZbAu9Vw7k9HgA21rR51Bssu8uJVYG");

const DESCRIMINATOR: usize = 8;

#[program]
pub mod superbond {
    use super::*;

    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        uri: String,
        bond: Pubkey,
    ) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"info", &[ctx.bumps.token_data]]];
        let metadata_cpi_ctx = CpiContext::new(
            ctx.accounts.metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                payer: ctx.accounts.signer.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                mint_authority: ctx.accounts.token_data.to_account_info(),
                update_authority: ctx.accounts.token_data.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        )
        .with_signer(signer_seeds);
        let metadata = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        create_metadata_accounts_v3(metadata_cpi_ctx, metadata, false, true, None)?;
        ctx.accounts.token_data.bond = bond;
        ctx.accounts.token_data.mint = ctx.accounts.mint.key();
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct TokenInfo {
    pub bond: Pubkey,
    pub mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct CreateToken<'info> {
    #[account(init, mint::decimals = 2, payer = signer, mint::authority = token_data.key())]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, seeds = [b"info", signer.key().as_ref(), mint.key().as_ref()], bump, payer = signer, space = DESCRIMINATOR + TokenInfo::INIT_SPACE)]
    pub token_data: Account<'info, TokenInfo>,

    /// CHECK: Validate address by deriving pda
    #[account(mut, seeds=[b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()], bump,
            seeds::program = metadata_program.key())]
    pub metadata: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
