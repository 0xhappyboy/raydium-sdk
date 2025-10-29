pub mod liquidity;
pub mod typs;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_network_sdk::Solana;
use solana_sdk::pubkey::Pubkey;
use std::{str::FromStr, sync::Arc};

use crate::liquidity::v4::{RaydiumLiquidityPoolData, RaydiumLiquidityPoolV4};

/// raydium data structure
pub struct Raydium {
    pub solana: Arc<Solana>,
}

impl Raydium {
    /// crreate raydium
    /// Example
    /// ```rust
    /// let sol = Solana::new(solana_network_sdk::types::Mode::MAIN);
    /// let raydium = Raydium::new(Arc::new(sol));
    /// ```
    pub fn new(solana: Arc<Solana>) -> Self {
        Self { solana: solana }
    }
    /// get v4 raydium liquidity pool
    /// Example
    /// ```rust
    /// let sol = Solana::new(solana_network_sdk::types::Mode::MAIN);
    /// let raydium = Raydium::new(Arc::new(sol));
    /// // 58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2 SOL-USDC pool
    /// let pool_data = raydium.get_liquidity_pool_v4("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2").await;
    /// ```
    pub async fn get_liquidity_pool_v4(
        &self,
        address: &str,
    ) -> Result<RaydiumLiquidityPoolData, String> {
        match Pubkey::from_str(address) {
            Ok(pool_address) => match self.client.get_account_data(&pool_address).await {
                Ok(v) => match RaydiumLiquidityPoolV4::get_liquidity_pool_info(&v) {
                    Ok(pool) => return Ok(pool.clone()),
                    Err(e) => return Err(e),
                },
                Err(e) => Err(format!("get liquidity pool error:{:?}", e)),
            },
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        }
    }
    // get token price by address
    pub async fn get_token_price_by_address(&self) {}
}
