pub mod liquidity;
pub mod typs;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::{str::FromStr, sync::Arc};

use crate::liquidity::v4::RaydiumLiquidityPoolV4;

pub struct Raydium {
    pub client: Arc<RpcClient>,
}

impl Raydium {
    pub fn new(client: Arc<RpcClient>) -> Self {
        Self { client: client }
    }
    /// get v4 raydium liquidity pool
    pub async fn get_liquidity_pool_v4(
        &self,
        address: &str,
    ) -> Result<RaydiumLiquidityPoolV4, String> {
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
