use anchor_lang::prelude::*;

declare_id!("SoLD0cs11111111111111111111111111111111111111");

#[program]
pub mod soldocs {
    use super::*;

    pub fn register_document(
        ctx: Context<RegisterDocument>,
        ipfs_hash: String,
        expires_at: i64,
    ) -> Result<()> {
        let document = &mut ctx.accounts.document;
        document.owner = *ctx.accounts.owner.key;
        document.ipfs_hash = ipfs_hash;
        document.expires_at = expires_at;
        document.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn grant_access(ctx: Context<GrantAccess>, grantee: Pubkey) -> Result<()> {
        let access = &mut ctx.accounts.access;
        access.doc = ctx.accounts.document.key();
        access.owner = *ctx.accounts.owner.key;
        access.grantee = grantee;
        access.granted_at = Clock::get()?.unix_timestamp;
        access.revoked = false;
        Ok(())
    }

    pub fn revoke_access(ctx: Context<RevokeAccess>) -> Result<()> {
        let access = &mut ctx.accounts.access;
        require!(access.owner == *ctx.accounts.owner.key, SolDocsError::Unauthorized);
        access.revoked = true;
        Ok(())
    }

    pub fn log_access(ctx: Context<LogAccess>) -> Result<()> {
        let log = &mut ctx.accounts.access_log;
        log.doc = ctx.accounts.document.key();
        log.user = *ctx.accounts.user.key;
        log.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterDocument<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 128 + 8 + 8)]
    pub document: Account<'info, Document>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GrantAccess<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 32 + 32 + 8 + 1)]
    pub access: Account<'info, Access>,
    #[account(mut, has_one = owner)]
    pub document: Account<'info, Document>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeAccess<'info> {
    #[account(mut, has_one = owner)]
    pub access: Account<'info, Access>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct LogAccess<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub access_log: Account<'info, AccessLog>,
    #[account(mut)]
    pub document: Account<'info, Document>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Document {
    pub owner: Pubkey,
    pub ipfs_hash: String, // Encrypted file CID
    pub expires_at: i64,
    pub created_at: i64,
}

#[account]
pub struct Access {
    pub doc: Pubkey,
    pub owner: Pubkey,
    pub grantee: Pubkey,
    pub granted_at: i64,
    pub revoked: bool,
}

#[account]
pub struct AccessLog {
    pub doc: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum SolDocsError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
