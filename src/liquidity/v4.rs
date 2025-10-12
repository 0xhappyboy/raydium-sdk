use std::str::from_utf8;

use bytemuck::{Pod, Zeroable};
use solana_sdk::pubkey::Pubkey;

// raydium liquidity pool v4 data size
const RAYDIUM_LIQUIDITY_POOL_V4_DATA_SIZE: usize = 752;

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

    swap_base_in_amount: u64,   // 256-263
    swap_quote_out_amount: u64, // 264-271
    swap_base2_quote_fee: u64,  // 272-279
    swap_quote_in_amount: u64,  // 280-287
    swap_base_out_amount: u64,  // 288-295
    swap_quote2_base_fee: u64,  // 296-303

    // padding
    padding: [u8; 32],           // 304-335
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
}

impl RaydiumLiquidityPoolV4 {
    pub fn get_liquidity_pool_info(data: &[u8]) -> Result<&Self, String> {
        if data.len() != RAYDIUM_LIQUIDITY_POOL_V4_DATA_SIZE {
            return Err(
                "raydium liquidity pool v4 data size does not meet requirements.".to_string(),
            );
        }
        if data.len() < std::mem::size_of::<Self>() {
            return Err("account data length error".to_string());
        }
        let pool = bytemuck::from_bytes::<Self>(&data[0..std::mem::size_of::<Self>()]);
        Ok(pool)
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
    pub fn swap_base_in_amount(&self) -> u64 {
        self.swap_base_in_amount as u64
    }
    pub fn swap_quote_out_amount(&self) -> u64 {
        self.swap_base_out_amount as u64
    }
    pub fn swap_base2_quote_fee(&self) -> u64 {
        self.swap_base2_quote_fee as u64
    }
    pub fn swap_quote_in_amount(&self) -> u64 {
        self.swap_base_in_amount as u64
    }
    pub fn swap_base_out_amount(&self) -> u64 {
        self.swap_base_out_amount as u64
    }
    pub fn swap_quote2_base_fee(&self) -> u64 {
        self.swap_quote2_base_fee as u64
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
    pub fn display(&self) {
        println!("status:{:?}", self.status());
        println!("nonce:{:?}", self.nonce());
        println!("max_order:{:?}", self.max_order());
        println!("depth:{:?}", self.depth());
        println!("base_decimal:{:?}", self.base_decimal());
        println!("quote_decimal:{:?}", self.quote_decimal());
        println!("state:{:?}", self.state());
        println!("resetFlag:{:?}", self.reset_flag());
        println!("minSize:{:?}", self.min_size());
        println!("volMaxCutRatio:{:?}", self.vol_max_cut_ratio());
        println!("amountWaveRatio:{:?}", self.amount_wave_ratio());
        println!("baseLotSize:{:?}", self.base_lot_size());
        println!("quoteLotSize:{:?}", self.quote_lot_size());
        println!("minPriceMultiplier:{:?}", self.min_price_multiplier());
        println!("maxPriceMultiplier:{:?}", self.max_price_multiplier());
        println!("systemDecimalValue:{:?}", self.system_decimal_value());
        println!("minSeparateNumerator:{:?}", self.min_separate_numerator());
        println!(
            "minSeparateDenominator:{:?}",
            self.min_separate_denominator()
        );
        println!("tradeFeeNumerator:{:?}", self.trade_fee_numerator());
        println!("tradeFeeDenominator:{:?}", self.trade_fee_denominator());
        println!("pnlNumerator:{:?}", self.pnl_numerator());
        println!("pnlDenominator:{:?}", self.pnl_denominator());
        println!("swapFeeNumerator:{:?}", self.swap_fee_numerator());
        println!("swapFeeDenominator:{:?}", self.swap_fee_denominator());
        println!("baseNeedTakePnl:{:?}", self.base_need_take_pnl());
        println!("quoteNeedTakePnl:{:?}", self.quote_need_take_pnl());
        println!("quoteTotalPnl:{:?}", self.quote_total_pnl());
        println!("baseTotalPnl:{:?}", self.base_total_pnl());
        println!("poolOpenTime:{:?}", self.pool_open_time());
        println!("punishPcAmount:{:?}", self.punish_pc_amount());
        println!("punishCoinAmount:{:?}", self.punish_coin_amount());
        println!("orderbookToInitTime:{:?}", self.orderbook_to_init_time());

        println!("swapBaseInAmount:{:?}", self.swap_base_in_amount());
        println!("swapQuoteOutAmount:{:?}", self.swap_base2_quote_fee());
        println!("swapBase2QuoteFee:{:?}", self.swap_base_out_amount());
        println!("swapQuoteInAmount:{:?}", self.swap_quote2_base_fee());
        println!("swapBaseOutAmount:{:?}", self.swap_quote_in_amount());
        println!("swapQuote2BaseFee:{:?}", self.swap_quote_out_amount());

        println!("baseVault:{:?}", self.base_vault());
        println!("quoteVault:{:?}", self.quote_vault());
        println!("baseMint:{:?}", self.base_mint());
        println!("quoteMint:{:?}", self.quote_mint());
        println!("lpMint:{:?}", self.lp_mint());
        println!("openOrders:{:?}", self.open_orders());
        println!("marketId:{:?}", self.market_id());
        println!("marketProgramId:{:?}", self.market_program_id());
        println!("targetOrders:{:?}", self.target_orders());
        println!("withdrawQueue:{:?}", self.withdraw_queue());
        println!("lpVault:{:?}", self.lp_vault());
        println!("owner:{:?}", self.owner());
        println!("lpReserve:{:?}", self.lp_reserve());
    }
}
