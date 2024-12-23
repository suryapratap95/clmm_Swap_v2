use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

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
    pub price_impact: u64,
    pub sqrt_price_limit: u128,
}

#[event]
pub struct LiquidityAddedEvent {
    pub pool_id: Pubkey,
    pub liquidity_added: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub amount_0: u64,
    pub amount_1: u64,
}

#[event]
pub struct LiquidityRemovedEvent {
    pub pool_id: Pubkey,
    pub liquidity_removed: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub amount_0: u64,
    pub amount_1: u64,
}

#[event]
pub struct PoolUpdateEvent {
    pub pool_id: Pubkey,
    pub sqrt_price: u128,
    pub tick_index: i32,
    pub liquidity: u128,
    pub fee_growth_global_0: u128,
    pub fee_growth_global_1: u128,
}

#[event]
pub struct PositionUpdateEvent {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tokens_owed_0: u64,
    pub tokens_owed_1: u64,
    pub update_type: String,
}

// Helper functions
pub fn calculate_price_impact(
    amount_in: u64,
    amount_out_min: u64,
    current_sqrt_price: u128,
) -> Result<u64> {
    // Price impact calculation using square root price
    let amount_in_u128 = amount_in as u128;
    let amount_out_u128 = amount_out_min as u128;

    let ideal_out = amount_in_u128
        .checked_mul(current_sqrt_price)
        .ok_or(ErrorCode::MathOverflow)?;

    let actual_out = amount_out_u128
        .checked_mul(current_sqrt_price)
        .ok_or(ErrorCode::MathOverflow)?;

    let impact = ideal_out
        .checked_sub(actual_out)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_mul(10000)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(ideal_out)
        .ok_or(ErrorCode::MathOverflow)? as u64;

    Ok(impact)
}

pub fn validate_tick_range(lower: i32, upper: i32, tick_spacing: i32) -> Result<()> {
    require!(lower < upper, ErrorCode::InvalidTickRange);
    require!(lower % tick_spacing == 0, ErrorCode::InvalidTickRange);
    require!(upper % tick_spacing == 0, ErrorCode::InvalidTickRange);
    Ok(())
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

// Parameters for liquidity creation and swap operations
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateLiquidityParams {
    pub liquidity_delta: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SwapV2Params {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

// Context for CreateLiquidity and SwapV2 operations
#[derive(Accounts)]
#[instruction(params: CreateLiquidityParams)]
pub struct CreateLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    #[account(
        mut,
        constraint = user_token_0_account.owner == user.key(),
        constraint = user_token_0_account.mint == pool_state.token_mint_0
    )]
    pub user_token_0_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_1_account.owner == user.key(),
        constraint = user_token_1_account.mint == pool_state.token_mint_1
    )]
    pub user_token_1_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pool_token_0_vault.owner == pool_state.authority,
        constraint = pool_token_0_vault.mint == pool_state.token_mint_0
    )]
    pub pool_token_0_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pool_token_1_vault.owner == pool_state.authority,
        constraint = pool_token_1_vault.mint == pool_state.token_mint_1
    )]
    pub pool_token_1_vault: Account<'info, TokenAccount>,

    /// CHECK: Verified through CPI
    #[account(address = raydium::ID)]
    pub raydium_program: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
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

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        tick_spacing: i32,
        initial_sqrt_price: u128,
    ) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;

        require!(tick_spacing > 0, ErrorCode::InvalidTickSpacing);
        require!(initial_sqrt_price > 0, ErrorCode::InvalidSqrtPrice);

        pool_state.authority = ctx.accounts.authority.key();
        pool_state.token_mint_0 = ctx.accounts.token_mint_0.key();
        pool_state.token_mint_1 = ctx.accounts.token_mint_1.key();
        pool_state.tick_spacing = tick_spacing;
        pool_state.current_sqrt_price = initial_sqrt_price;
        pool_state.token_vault_0 = ctx.accounts.token_vault_0.key();
        pool_state.token_vault_1 = ctx.accounts.token_vault_1.key();
        pool_state.is_paused = false;
        pool_state.pool_id = pool_state.key();
        Ok(())
    }

    pub fn create_liquidity(
        ctx: Context<CreateLiquidity>,
        params: CreateLiquidityParams,
    ) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        require!(!pool_state.is_paused, ErrorCode::PoolIsPaused);

        // Perform validations
        validate_tick_range(
            params.tick_lower_index,
            params.tick_upper_index,
            pool_state.tick_spacing,
        )?;

        // Emit event
        emit!(LiquidityAddedEvent {
            pool_id: pool_state.key(),
            liquidity_added: params.liquidity_delta,
            tick_lower_index: params.tick_lower_index,
            tick_upper_index: params.tick_upper_index,
            amount_0: params.amount_0_max,
            amount_1: params.amount_1_max,
        });

        Ok(())
    }

    pub fn swap_v2(ctx: Context<SwapV2>, params: SwapV2Params) -> Result<()> {
        require!(!ctx.accounts.pool_state.is_paused, ErrorCode::PoolPaused);
        require!(params.amount > 0, ErrorCode::InsufficientInput);

        // Validate slippage and price impact
        let price_impact = calculate_price_impact(
            params.amount,
            params.other_amount_threshold,
            ctx.accounts.pool_state.current_sqrt_price,
        )?;
        require!(
            price_impact <= MAX_PRICE_IMPACT,
            ErrorCode::ExcessivePriceImpact
        );

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
            price_impact,
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
