use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_tool::account::get_spl_token_balance;
use solana_tool::reader::r_u64;
use solana_tool::reader::r_u128;
use solana_tool::unit::conver_balance;
use tokio::join;

/// raydium liquidity pool v4 data size
const RAYDIUM_LIQUIDITY_POOL_V4_DATA_SIZE: usize = 752;
/// offset relative to the swap field
const SWAP_BASE_IN_AMOUNT_OFFSET: usize = 256;
const SWAP_QUOTE_OUT_AMOUNT_OFFSET: usize = 272;
const SWAP_BASE2_QUOTE_FEE_OFFSET: usize = 288;
const SWAP_QUOTE_IN_AMOUNT_OFFSET: usize = 296;
const SWAP_BASE_OUT_AMOUNT_OFFSET: usize = 312;
const SWAP_QUOTE2_BASE_FEE_OFFSET: usize = 328;

/// raydium liquidity pool v4 raw data
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct RaydiumLiquidityPoolV4 {
    status: u64,                   // 0-7
    nonce: u64,                    // 8-15
    max_order: u64,                // 16-23
    depth: u64,                    // 24-31
    base_decimal: u64,             // 32-39
    quote_decimal: u64,            // 40-47
    state: u64,                    // 48-55
    reset_flag: u64,               // 56-63
    min_size: u64,                 // 64-71
    vol_max_cut_ratio: u64,        // 72-79
    amount_wave_ratio: u64,        // 80-87
    base_lot_size: u64,            // 88-95
    quote_lot_size: u64,           // 96-103
    min_price_multiplier: u64,     // 104-111
    max_price_multiplier: u64,     // 112-119
    system_decimal_value: u64,     // 120-127
    min_separate_numerator: u64,   // 128-135
    min_separate_denominator: u64, // 136-143
    trade_fee_numerator: u64,      // 144-151
    trade_fee_denominator: u64,    // 152-159
    pnl_numerator: u64,            // 160-167
    pnl_denominator: u64,          // 168-175
    swap_fee_numerator: u64,       // 176-183
    swap_fee_denominator: u64,     // 184-191
    base_need_take_pnl: u64,       // 192-199
    quote_need_take_pnl: u64,      // 200-207
    quote_total_pnl: u64,          // 208-215
    base_total_pnl: u64,           // 216-223
    pool_open_time: u64,           // 224-231
    punish_pc_amount: u64,         // 232-239
    punish_coin_amount: u64,       // 240-247
    orderbook_to_init_time: u64,   // 248-255
    // ====== used as a placeholder, not actually involved in deserialization ======
    swap_base_in_amount: u64,   // 256-263
    swap_quote_out_amount: u64, // 264-271
    swap_base2_quote_fee: u64,  // 272-279
    swap_quote_in_amount: u64,  // 280-287
    swap_base_out_amount: u64,  // 288-295
    swap_quote2_base_fee: u64,  // 296-303
    // =============================================================================
    // padding 0
    padding_0: [u8; 32],         // 304-335
    base_vault: [u8; 32],        // 336-367
    quote_vault: [u8; 32],       // 368-399
    base_mint: [u8; 32],         // 400-431
    quote_mint: [u8; 32],        // 432-463
    lp_mint: [u8; 32],           // 464-495
    open_orders: [u8; 32],       // 496-527
    market_id: [u8; 32],         // 528-559
    market_program_id: [u8; 32], // 560-591
    target_orders: [u8; 32],     // 592-623
    withdraw_queue: [u8; 32],    // 624-655
    lp_vault: [u8; 32],          // 656-687
    owner: [u8; 32],             // 752-783
    lp_reserve: u64,             // 688-719
    // end padding
    padding_1: [u64; 3],
}

impl RaydiumLiquidityPoolV4 {
    /// parsing the data structure of the Liquidity v4 pool
    /// # Example
    /// ```rust
    /// let address: &str = "public key";
    /// match Pubkey::from_str(address) {
    ///     Ok(pool_address) => match self.client.get_account_data(&pool_address).await {
    ///         Ok(v) => match RaydiumLiquidityPoolV4::get_liquidity_pool_info(&v) {
    ///             Ok(pool) => return Ok(pool.clone()),
    ///             Err(e) => return Err(e),
    ///         },
    ///         Err(e) => Err(format!("{:?}", e)),
    ///     },
    ///     Err(e) => {
    ///         return Err(format!("{:?}", e));
    ///     }
    /// }
    /// ```
    pub fn get_liquidity_pool_info(data: &[u8]) -> Result<RaydiumLiquidityPoolData, String> {
        if data.len() != RAYDIUM_LIQUIDITY_POOL_V4_DATA_SIZE {
            return Err(
                "raydium liquidity pool v4 data size does not meet requirements.".to_string(),
            );
        }
        if data.len() < std::mem::size_of::<Self>() {
            return Err("account data length error".to_string());
        }
        let pool = bytemuck::from_bytes::<Self>(&data[0..std::mem::size_of::<Self>()]);
        // ===================== manual parsing swap field =====================
        let swap_base_in_amount = r_u128(data, SWAP_BASE_IN_AMOUNT_OFFSET);
        let swap_quote_out_amount = r_u128(data, SWAP_QUOTE_OUT_AMOUNT_OFFSET);
        let swap_base2_quote_fee = r_u64(data, SWAP_BASE2_QUOTE_FEE_OFFSET);
        let swap_quote_in_amount = r_u128(data, SWAP_QUOTE_IN_AMOUNT_OFFSET);
        let swap_base_out_amount = r_u128(data, SWAP_BASE_OUT_AMOUNT_OFFSET);
        let swap_quote2_base_fee = r_u64(data, SWAP_QUOTE2_BASE_FEE_OFFSET);
        // ===================== manual parsing swap field =====================
        let pool_data: RaydiumLiquidityPoolData = RaydiumLiquidityPoolData {
            status: pool.status(),
            nonce: pool.nonce(),
            max_order: pool.max_order(),
            depth: pool.depth(),
            base_decimal: pool.base_decimal(),
            quote_decimal: pool.quote_decimal(),
            state: pool.state(),
            reset_flag: pool.reset_flag(),
            min_size: pool.min_size(),
            vol_max_cut_ratio: pool.vol_max_cut_ratio(),
            amount_wave_ratio: pool.amount_wave_ratio(),
            base_lot_size: pool.base_lot_size(),
            quote_lot_size: pool.quote_lot_size(),
            min_price_multiplier: pool.min_price_multiplier(),
            max_price_multiplier: pool.max_price_multiplier(),
            system_decimal_value: pool.system_decimal_value(),
            min_separate_numerator: pool.min_separate_numerator(),
            min_separate_denominator: pool.min_separate_denominator(),
            trade_fee_numerator: pool.trade_fee_numerator(),
            trade_fee_denominator: pool.trade_fee_denominator(),
            pnl_numerator: pool.pnl_numerator(),
            pnl_denominator: pool.pnl_denominator(),
            swap_fee_numerator: pool.swap_fee_numerator(),
            swap_fee_denominator: pool.swap_fee_denominator(),
            base_need_take_pnl: pool.base_need_take_pnl(),
            quote_need_take_pnl: pool.quote_need_take_pnl(),
            quote_total_pnl: pool.quote_total_pnl(),
            base_total_pnl: pool.base_total_pnl(),
            pool_open_time: pool.pool_open_time(),
            punish_pc_amount: pool.punish_pc_amount(),
            punish_coin_amount: pool.punish_coin_amount(),
            orderbook_to_init_time: pool.orderbook_to_init_time(),
            swap_base_in_amount,
            swap_quote_out_amount,
            swap_base2_quote_fee,
            swap_quote_in_amount,
            swap_base_out_amount,
            swap_quote2_base_fee,
            base_vault: pool.base_vault(),
            quote_vault: pool.quote_vault(),
            base_mint: pool.base_mint(),
            quote_mint: pool.quote_mint(),
            lp_mint: pool.lp_mint(),
            open_orders: pool.open_orders(),
            market_id: pool.market_id(),
            market_program_id: pool.market_program_id(),
            target_orders: pool.target_orders(),
            withdraw_queue: pool.withdraw_queue(),
            lp_vault: pool.lp_vault(),
            owner: pool.owner(),
            lp_reserve: pool.lp_reserve(),
        };
        Ok(pool_data)
    }
    pub fn base_mint_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.base_mint)
    }
    pub fn quote_mint_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.quote_mint)
    }
    pub fn base_vault_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.base_vault)
    }
    pub fn quote_vault_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.quote_vault)
    }
    pub fn status(&self) -> u8 {
        self.status as u8
    }
    pub fn nonce(&self) -> u8 {
        self.nonce as u8
    }
    pub fn max_order(&self) -> u8 {
        self.max_order as u8
    }
    pub fn base_decimal(&self) -> u8 {
        self.base_decimal as u8
    }
    pub fn quote_decimal(&self) -> u8 {
        self.quote_decimal as u8
    }
    pub fn depth(&self) -> u8 {
        self.depth as u8
    }
    pub fn state(&self) -> u8 {
        self.state as u8
    }
    pub fn reset_flag(&self) -> u8 {
        self.reset_flag as u8
    }
    pub fn min_size(&self) -> u64 {
        self.min_size as u64
    }
    pub fn vol_max_cut_ratio(&self) -> u64 {
        self.vol_max_cut_ratio as u64
    }
    pub fn amount_wave_ratio(&self) -> u64 {
        self.amount_wave_ratio as u64
    }
    pub fn base_lot_size(&self) -> u64 {
        self.base_lot_size as u64
    }
    pub fn quote_lot_size(&self) -> u64 {
        self.quote_lot_size as u64
    }
    pub fn min_price_multiplier(&self) -> u64 {
        self.min_price_multiplier as u64
    }
    pub fn max_price_multiplier(&self) -> u64 {
        self.max_price_multiplier as u64
    }
    pub fn system_decimal_value(&self) -> u64 {
        self.system_decimal_value as u64
    }
    pub fn min_separate_numerator(&self) -> u64 {
        self.min_separate_numerator as u64
    }
    pub fn min_separate_denominator(&self) -> u64 {
        self.min_separate_denominator as u64
    }
    pub fn trade_fee_numerator(&self) -> u64 {
        self.trade_fee_numerator as u64
    }
    pub fn trade_fee_denominator(&self) -> u64 {
        self.trade_fee_denominator as u64
    }
    pub fn pnl_numerator(&self) -> u64 {
        self.pnl_numerator as u64
    }
    pub fn pnl_denominator(&self) -> u64 {
        self.pnl_denominator as u64
    }
    pub fn swap_fee_numerator(&self) -> u64 {
        self.swap_fee_numerator as u64
    }
    pub fn swap_fee_denominator(&self) -> u64 {
        self.swap_fee_denominator as u64
    }
    pub fn base_need_take_pnl(&self) -> u64 {
        self.base_need_take_pnl as u64
    }
    pub fn quote_need_take_pnl(&self) -> u64 {
        self.quote_need_take_pnl as u64
    }
    pub fn quote_total_pnl(&self) -> u64 {
        self.quote_total_pnl as u64
    }
    pub fn base_total_pnl(&self) -> u64 {
        self.base_total_pnl
    }
    pub fn pool_open_time(&self) -> u64 {
        self.pool_open_time as u64
    }
    pub fn punish_pc_amount(&self) -> u64 {
        self.punish_pc_amount as u64
    }
    pub fn punish_coin_amount(&self) -> u64 {
        self.punish_coin_amount as u64
    }
    pub fn orderbook_to_init_time(&self) -> u64 {
        self.orderbook_to_init_time
    }
    pub fn lp_reserve(&self) -> u64 {
        self.lp_reserve
    }
    pub fn base_vault(&self) -> Pubkey {
        Pubkey::new_from_array(self.base_vault)
    }
    pub fn quote_vault(&self) -> Pubkey {
        Pubkey::new_from_array(self.quote_vault)
    }
    pub fn base_mint(&self) -> Pubkey {
        Pubkey::new_from_array(self.base_mint)
    }
    pub fn quote_mint(&self) -> Pubkey {
        Pubkey::new_from_array(self.quote_mint)
    }
    pub fn lp_mint(&self) -> Pubkey {
        Pubkey::new_from_array(self.lp_mint)
    }
    pub fn open_orders(&self) -> Pubkey {
        Pubkey::new_from_array(self.open_orders)
    }
    pub fn market_id(&self) -> Pubkey {
        Pubkey::new_from_array(self.market_id)
    }
    pub fn market_program_id(&self) -> Pubkey {
        Pubkey::new_from_array(self.market_program_id)
    }
    pub fn target_orders(&self) -> Pubkey {
        Pubkey::new_from_array(self.target_orders)
    }
    pub fn withdraw_queue(&self) -> Pubkey {
        Pubkey::new_from_array(self.withdraw_queue)
    }
    pub fn lp_vault(&self) -> Pubkey {
        Pubkey::new_from_array(self.lp_vault)
    }
    pub fn owner(&self) -> Pubkey {
        Pubkey::new_from_array(self.owner)
    }
}

#[derive(Debug, Clone)]
pub struct RaydiumLiquidityPoolData {
    pub status: u8,
    pub nonce: u8,
    pub max_order: u8,
    pub depth: u8,
    pub base_decimal: u8,
    pub quote_decimal: u8,
    pub state: u8,
    pub reset_flag: u8,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave_ratio: u64,
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub system_decimal_value: u64,
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
    pub base_need_take_pnl: u64,
    pub quote_need_take_pnl: u64,
    pub quote_total_pnl: u64,
    pub base_total_pnl: u64,
    pub pool_open_time: u64,
    pub punish_pc_amount: u64,
    pub punish_coin_amount: u64,
    pub orderbook_to_init_time: u64,
    pub swap_base_in_amount: u128,
    pub swap_quote_out_amount: u128,
    pub swap_base2_quote_fee: u64,
    pub swap_quote_in_amount: u128,
    pub swap_base_out_amount: u128,
    pub swap_quote2_base_fee: u64,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lp_mint: Pubkey,
    pub open_orders: Pubkey,
    pub market_id: Pubkey,
    pub market_program_id: Pubkey,
    pub target_orders: Pubkey,
    pub withdraw_queue: Pubkey,
    pub lp_vault: Pubkey,
    pub owner: Pubkey,
    pub lp_reserve: u64,
}

impl RaydiumLiquidityPoolData {
    /// get the current liquidity pool price
    /// Example
    /// ```rust
    /// // client
    /// let rpc = RpcClient::new("".to_string());
    /// let ray = Raydium::new(Arc::new(rpc));
    /// let pool_data = ray.get_liquidity_pool_v4(pool_address).await.unwrap();
    /// let price = pool_data.get_price(Arc::clone(&ray.client)).await;
    /// ```
    pub async fn get_price(&self, client: Arc<RpcClient>) -> f64 {
        let base_token_vault_address = self.base_vault;
        let base_quote_vault_address = self.quote_vault;
        let (base_token_balance, quote_token_balance) = join!(
            get_spl_token_balance(Arc::clone(&client), base_token_vault_address),
            get_spl_token_balance(Arc::clone(&client), base_quote_vault_address)
        );
        let base_amount_normalized = conver_balance(base_token_balance.unwrap(), self.base_decimal);
        let quote_amount_normalized =
            conver_balance(quote_token_balance.unwrap(), self.quote_decimal);
        quote_amount_normalized / base_amount_normalized
    }
    pub fn display(&self) {
        println!("status:{:?}", self.status);
        println!("nonce:{:?}", self.nonce);
        println!("max_order:{:?}", self.max_order);
        println!("depth:{:?}", self.depth);
        println!("base_decimal:{:?}", self.base_decimal);
        println!("quote_decimal:{:?}", self.quote_decimal);
        println!("state:{:?}", self.state);
        println!("resetFlag:{:?}", self.reset_flag);
        println!("minSize:{:?}", self.min_size);
        println!("volMaxCutRatio:{:?}", self.vol_max_cut_ratio);
        println!("amountWaveRatio:{:?}", self.amount_wave_ratio);
        println!("baseLotSize:{:?}", self.base_lot_size);
        println!("quoteLotSize:{:?}", self.quote_lot_size);
        println!("minPriceMultiplier:{:?}", self.min_price_multiplier);
        println!("maxPriceMultiplier:{:?}", self.max_price_multiplier);
        println!("systemDecimalValue:{:?}", self.system_decimal_value);
        println!("minSeparateNumerator:{:?}", self.min_separate_numerator);
        println!("minSeparateDenominator:{:?}", self.min_separate_denominator);
        println!("tradeFeeNumerator:{:?}", self.trade_fee_numerator);
        println!("tradeFeeDenominator:{:?}", self.trade_fee_denominator);
        println!("pnlNumerator:{:?}", self.pnl_numerator);
        println!("pnlDenominator:{:?}", self.pnl_denominator);
        println!("swapFeeNumerator:{:?}", self.swap_fee_numerator);
        println!("swapFeeDenominator:{:?}", self.swap_fee_denominator);
        println!("baseNeedTakePnl:{:?}", self.base_need_take_pnl);
        println!("quoteNeedTakePnl:{:?}", self.quote_need_take_pnl);
        println!("quoteTotalPnl:{:?}", self.quote_total_pnl);
        println!("baseTotalPnl:{:?}", self.base_total_pnl);
        println!("poolOpenTime:{:?}", self.pool_open_time);
        println!("punishPcAmount:{:?}", self.punish_pc_amount);
        println!("punishCoinAmount:{:?}", self.punish_coin_amount);
        println!("orderbookToInitTime:{:?}", self.orderbook_to_init_time);
        println!("swapBaseInAmount:{:?}", self.swap_base_in_amount);
        println!("swapQuoteOutAmount:{:?}", self.swap_quote_out_amount);
        println!("swapBase2QuoteFee:{:?}", self.swap_base2_quote_fee);
        println!("swapQuoteInAmount:{:?}", self.swap_quote_in_amount);
        println!("swapBaseOutAmount:{:?}", self.swap_base_out_amount);
        println!("swapQuote2BaseFee:{:?}", self.swap_quote2_base_fee);
        println!("baseVault:{:?}", self.base_vault);
        println!("quoteVault:{:?}", self.quote_vault);
        println!("baseMint:{:?}", self.base_mint);
        println!("quoteMint:{:?}", self.quote_mint);
        println!("lpMint:{:?}", self.lp_mint);
        println!("openOrders:{:?}", self.open_orders);
        println!("marketId:{:?}", self.market_id);
        println!("marketProgramId:{:?}", self.market_program_id);
        println!("targetOrders:{:?}", self.target_orders);
        println!("withdrawQueue:{:?}", self.withdraw_queue);
        println!("lpVault:{:?}", self.lp_vault);
        println!("owner:{:?}", self.owner);
        println!("lpReserve:{:?}", self.lp_reserve);
    }
}
