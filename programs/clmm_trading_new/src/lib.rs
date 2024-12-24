use anchor_lang::prelude::*;
use anchor_spl::{token::{self, Mint, Token, TokenAccount}, token_2022::Token2022};

declare_id!("E39ZYh2CjA6ht8nNe5tRUKEWvBQMin8wB9Zi3iyrU8nG");

// Constants and Program IDs
pub mod raydium {
    use anchor_lang::prelude::*;
    declare_id!("devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH");
}

pub const MAX_ROUTE_HOPS: usize = 4;
pub const DEFAULT_SLIPPAGE_TOLERANCE: u64 = 100; // 1% = 100 basis points
pub const MAX_PRICE_IMPACT: u64 = 1000; // 10% = 1000 basis points
pub const MINIMUM_LIQUIDITY: u128 = 1000; // Minimum liquidity requirement

#[event]
pub struct SwapEvent {
    pub pool_id: Pubkey,
    pub amount_in: u64,
    pub amount_out_min: u64,
    pub sqrt_price_limit: u128,
}


// PoolState Account
#[account]
#[derive(Default)]
pub struct PoolState {
    pub authority: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub tick_spacing: i32,
    pub tick_spacing_seed: u16,
    pub fee_rate: u32,
    pub liquidity: u128,
    pub current_sqrt_price: u128,
    pub current_tick_index: i32,
    pub fee_growth_global_0: u128,
    pub fee_growth_global_1: u128,
    pub fee_protocol_token_0: u64,
    pub fee_protocol_token_1: u64,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_key: Pubkey,
    pub pool_id: Pubkey,
    pub is_paused: bool,
    pub last_updated: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SwapV2Params {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}


#[derive(Accounts)]
#[instruction(params: SwapV2Params)]
pub struct SwapV2<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Verified through CPI
    pub amm_config: AccountInfo<'info>,

    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    #[account(mut)]
    pub input_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub output_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub input_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub output_vault: Account<'info, TokenAccount>,

    /// CHECK: Verified through CPI
    pub observation_state: AccountInfo<'info>,

    /// CHECK: Verified through CPI
    pub token_program: Program<'info, Token>,

    /// CHECK: Verified through CPI
    pub token_program_2022: Program<'info, Token>,

    /// CHECK: Verified through CPI
    pub memo_program: AccountInfo<'info>,

    /// CHECK: Verified through CPI
    pub input_vault_mint: AccountInfo<'info>,

    /// CHECK: Verified through CPI
    pub output_vault_mint: AccountInfo<'info>,
}

// Program implementation
#[program]
pub mod clmm_trading_new {
    use super::*;

    pub fn swap_v2(ctx: Context<SwapV2>, params: SwapV2Params) -> Result<()> {

        let accounts = vec![
            AccountMeta::new_readonly(ctx.accounts.payer.key(), true),
            AccountMeta::new_readonly(ctx.accounts.amm_config.key(), false),
            AccountMeta::new(ctx.accounts.pool_state.key(), false),
            AccountMeta::new(ctx.accounts.input_token_account.key(), false),
            AccountMeta::new(ctx.accounts.output_token_account.key(), false),
            AccountMeta::new(ctx.accounts.input_vault.key(), false),
            AccountMeta::new(ctx.accounts.output_vault.key(), false),
            AccountMeta::new(ctx.accounts.observation_state.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_program_2022.key(), false),
            AccountMeta::new_readonly(ctx.accounts.memo_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.input_vault_mint.key(), false),
            AccountMeta::new_readonly(ctx.accounts.output_vault_mint.key(), false),
        ];

        let swap_data = SwapV2Params {
            amount: params.amount,
            other_amount_threshold: params.other_amount_threshold,
            sqrt_price_limit_x64: params.sqrt_price_limit_x64,
            is_base_input: params.is_base_input,
        };

        let mut instruction_data = Vec::new();
        swap_data.serialize(&mut instruction_data)?;

        let swap_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: raydium::ID,
            accounts,
            data: instruction_data,
        };

        let account_infos = &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.amm_config.to_account_info(),
            ctx.accounts.pool_state.to_account_info(),
            ctx.accounts.input_token_account.to_account_info(),
            ctx.accounts.output_token_account.to_account_info(),
            ctx.accounts.input_vault.to_account_info(),
            ctx.accounts.output_vault.to_account_info(),
            ctx.accounts.observation_state.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.token_program_2022.to_account_info(),
            ctx.accounts.memo_program.to_account_info(),
            ctx.accounts.input_vault_mint.to_account_info(),
            ctx.accounts.output_vault_mint.to_account_info(),
        ];

        anchor_lang::solana_program::program::invoke(&swap_ix, account_infos)?;

        emit!(SwapEvent {
            pool_id: ctx.accounts.pool_state.pool_id,
            amount_in: params.amount,
            amount_out_min: params.other_amount_threshold,
            sqrt_price_limit: params.sqrt_price_limit_x64,
        });

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math operation overflowed")]
    MathOverflow, // Add this line
    #[msg("Invalid pool state")]
    InvalidPoolState,
    #[msg("Pool is paused")]
    PoolPaused,
    #[msg("Invalid tick spacing")]
    InvalidTickSpacing,
    #[msg("Invalid sqrt price")]
    InvalidSqrtPrice,
    #[msg("Invalid tick range")]
    InvalidTickRange,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Liquidity overflow")]
    LiquidityOverflow,
    #[msg("Insufficient input amount")]
    InsufficientInput,
    #[msg("Excessive price impact")]
    ExcessivePriceImpact,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid fee rate")]
    InvalidFeeRate,
    #[msg("Fee calculation overflow")]
    FeeOverflow,
    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Maximum tick index exceeded")]
    MaxTickIndexExceeded,
    #[msg("Minimum tick index exceeded")]
    MinTickIndexExceeded,
    #[msg("Invalid position")]
    InvalidPosition,
    #[msg("Position not found")]
    PositionNotFound,
    #[msg("Position update failed")]
    PositionUpdateFailed,
    #[msg("Observation state invalid")]
    ObservationStateInvalid,
    #[msg("Tick array invalid")]
    TickArrayInvalid,
    #[msg("Price limit reached")]
    PriceLimitReached,
    #[msg("Zero liquidity")]
    ZeroLiquidity,
    #[msg("Token account balance insufficient")]
    InsufficientTokenBalance,
    #[msg("Pool is Paused")]
    PoolIsPaused,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<PoolState>()
    )]
    pub pool_state: Account<'info, PoolState>,

    pub token_mint_0: Account<'info, token::Mint>,
    pub token_mint_1: Account<'info, token::Mint>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint_0,
        token::authority = authority,
    )]
    pub token_vault_0: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint_1,
        token::authority = authority,
    )]
    pub token_vault_1: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PositionInfo {
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub fee_growth_inside_0_last: u128,
    pub fee_growth_inside_1_last: u128,
    pub tokens_owed_0: u64,
    pub tokens_owed_1: u64,
}

#[account]
pub struct UserPosition {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub position_info: PositionInfo,
    pub created_at: i64,
    pub last_updated: i64,
}

impl<'info> SwapV2<'info> {
    // fn validate_accounts(&self) -> Result<()> {
    //     require!(
    //         self.user_source_token.owner == self.payer.key(),
    //         ErrorCode::InvalidTokenAccountOwner
    //     );
    //     require!(
    //         self.user_destination_token.owner == self.payer.key(),
    //         ErrorCode::InvalidTokenAccountOwner
    //     );
    //     Ok(())
    // }

    // fn verify_pool_state(&self) -> Result<()> {
    //     let pool_state = &self.pool_state;
    //     require!(!pool_state.is_paused, ErrorCode::PoolPaused);
    //     require!(pool_state.liquidity > 0, ErrorCode::ZeroLiquidity);
    //     Ok(())
    // }

    // fn transfer_tokens_to_pool(&self, amount: u64) -> Result<()> {
    //     let cpi_accounts = Transfer {
    //         from: self.user_source_token.to_account_info(),
    //         to: self.pool_source_vault.to_account_info(),
    //         authority: self.user.to_account_info(),
    //     };
    //     let cpi_program = self.token_program.to_account_info();
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     token::transfer(cpi_ctx, amount)?;
    //     Ok(())
    // }
}
