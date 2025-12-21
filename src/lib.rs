pub mod launchpad;
pub mod liquidity;
pub mod typs;

use solana_network_client::SolanaClient;
use solana_sdk::pubkey::Pubkey;

use std::{str::FromStr, sync::Arc};

use crate::{
    launchpad::{LaunchpadPool, LaunchpadPoolData},
    liquidity::{
        clmm::{RaydiumLiquidityPoolCLMM, RaydiumLiquidityPoolCLMMData},
        cpmm::{RaydiumLiquidityPoolCPMM, RaydiumLiquidityPoolCPMMData},
        v4::{RaydiumLiquidityPoolData, RaydiumLiquidityPoolV4},
    },
};

/// raydium data structure
pub struct Raydium {
    pub solana_client: Arc<SolanaClient>,
}

impl Raydium {
    /// crreate raydium
    /// Example
    /// ```rust
    /// let sol = Solana::new(solana_network_sdk::types::Mode::MAIN);
    /// let raydium = Raydium::new(Arc::new(sol));
    /// ```
    pub fn new(solana_client: Arc<SolanaClient>) -> Self {
        Self {
            solana_client: solana_client,
        }
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
        let v = self
            .solana_client
            .client_arc()
            .get_account_data(
                &Pubkey::from_str(address)
                    .map_err(|e| format!("{:?}", e))
                    .unwrap(),
            )
            .await
            .map_err(|e| format!("{:?}", e))?;
        let pool =
            RaydiumLiquidityPoolV4::get_liquidity_pool_info(&v).map_err(|e| format!("{:?}", e))?;
        Ok(pool)
    }

    pub async fn get_liquidity_pool_cpmm(
        &self,
        address: &str,
    ) -> Result<RaydiumLiquidityPoolCPMMData, String> {
        let v = self
            .solana_client
            .client_arc()
            .get_account_data(
                &Pubkey::from_str(address)
                    .map_err(|e| format!("{:?}", e))
                    .unwrap(),
            )
            .await
            .map_err(|e| format!("{:?}", e))?;
        let pool = RaydiumLiquidityPoolCPMM::get_liquidity_pool_info(&v)
            .map_err(|e| format!("{:?}", e))?;
        Ok(pool)
    }

    pub async fn get_liquidity_pool_clmm(
        &self,
        address: &str,
    ) -> Result<RaydiumLiquidityPoolCLMMData, String> {
        let v = self
            .solana_client
            .client_arc()
            .get_account_data(
                &Pubkey::from_str(address)
                    .map_err(|e| format!("{:?}", e))
                    .unwrap(),
            )
            .await
            .map_err(|e| format!("{:?}", e))?;
        let pool = RaydiumLiquidityPoolCLMM::get_liquidity_pool_info(&v)
            .map_err(|e| format!("{:?}", e))?;
        Ok(pool)
    }

    pub async fn get_liquidity_pool_launchpad(
        &self,
        address: &str,
    ) -> Result<LaunchpadPoolData, String> {
        let v = self
            .solana_client
            .client_arc()
            .get_account_data(
                &Pubkey::from_str(address)
                    .map_err(|e| format!("{:?}", e))
                    .unwrap(),
            )
            .await
            .map_err(|e| format!("{:?}", e))?;
        let pool = LaunchpadPool::get_liquidity_pool_info(&v).map_err(|e| format!("{:?}", e))?;
        Ok(pool)
    }
    // get token price by address
    pub async fn get_token_price_by_address(&self) {}
}
