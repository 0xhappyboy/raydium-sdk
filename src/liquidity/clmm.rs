use bytemuck::{Pod, Zeroable};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

/// CLMM liquidity pool data size
pub const RAYDIUM_LIQUIDITY_POOL_CLMM_DATA_SIZE: usize = 1544;
const DISCRIMINATOR_LEN: usize = 8;

unsafe impl Pod for RaydiumLiquidityPoolCLMM {}
unsafe impl Zeroable for RaydiumLiquidityPoolCLMM {}

/// CLMM liquidity pool raw data
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RaydiumLiquidityPoolCLMM {
    pub bump: u8,                        // 0: bump seed
    pub amm_config: [u8; 32],            // 1-32: AMM configuration
    pub owner: [u8; 32],                 // 33-64: Pool owner (program owner)
    pub token_mint_0: [u8; 32],          // 65-96: Token0 mint address
    pub token_mint_1: [u8; 32],          // 97-128: Token1 mint address
    pub token_vault_0: [u8; 32],         // 129-160: Token0 vault
    pub token_vault_1: [u8; 32],         // 161-192: Token1 vault
    pub observation_key: [u8; 32],       // 193-224: Observation address
    pub mint_decimals_0: u8,             // 225: Token0 decimals
    pub mint_decimals_1: u8,             // 226: Token1 decimals
    pub tick_spacing: u16,               // 227-228: Tick spacing
    pub liquidity: u128,                 // 229-244: Total liquidity
    pub sqrt_price_x64: u128,            // 245-260: Current sqrt price * 2^64
    pub tick_current: i32,               // 261-264: Current tick
    pub padding3: u16,                   // 265-266: Padding
    pub padding4: u16,                   // 267-268: Padding
    pub fee_growth_global_0_x64: u128,   // 269-284: Token0 global fee growth
    pub fee_growth_global_1_x64: u128,   // 285-300: Token1 global fee growth
    pub protocol_fees_token_0: u64,      // 301-308: Token0 protocol fees
    pub protocol_fees_token_1: u64,      // 309-316: Token1 protocol fees
    pub swap_in_amount_token_0: u128,    // 317-332: Token0 cumulative swap in amount
    pub swap_out_amount_token_1: u128,   // 333-348: Token1 cumulative swap out amount
    pub swap_in_amount_token_1: u128,    // 349-364: Token1 cumulative swap in amount
    pub swap_out_amount_token_0: u128,   // 365-380: Token0 cumulative swap out amount
    pub status: u8,                      // 381: Status
    pub padding: [u8; 7],                // 382-388: Padding
    pub reward_infos: [[u8; 104]; 3],    // 389-700: Reward info (3 reward pools)
    pub tick_array_bitmap: [u64; 16],    // 701-828: Tick array bitmap (16 u64s)
    pub total_fees_token_0: u64,         // 829-836: Token0 total fees
    pub total_fees_claimed_token_0: u64, // 837-844: Token0 claimed fees
    pub total_fees_token_1: u64,         // 845-852: Token1 total fees
    pub total_fees_claimed_token_1: u64, // 853-860: Token1 claimed fees
    pub fund_fees_token_0: u64,          // 861-868: Token0 fund fees
    pub fund_fees_token_1: u64,          // 869-876: Token1 fund fees
    pub open_time: u64,                  // 877-884: Open time
    pub recent_epoch: u64,               // 885-892: Recent epoch
    pub padding1: [[u8; 8]; 24],         // 893-1084: Padding (24 u64s)
    pub padding2: [[u8; 8]; 32],         // 1085-1340: Padding (32 u64s)
}

#[derive(Debug, Clone)]
pub struct RaydiumLiquidityPoolCLMMData {
    pub bump: u8,
    pub amm_config: Pubkey,
    pub owner: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_key: Pubkey,
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,
    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub swap_in_amount_token_0: u128,
    pub swap_out_amount_token_1: u128,
    pub swap_in_amount_token_1: u128,
    pub swap_out_amount_token_0: u128,
    pub status: u8,
    pub reward_infos: [RewardInfo; 3],
    pub tick_array_bitmap: [u64; 16],
    pub total_fees_token_0: u64,
    pub total_fees_claimed_token_0: u64,
    pub total_fees_token_1: u64,
    pub total_fees_claimed_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
}

#[derive(Debug, Clone)]
pub struct RewardInfo {
    pub reward_state: u8,
    pub open_time: u64,
    pub end_time: u64,
    pub last_update_time: u64,
    pub emissions_per_second_x64: u128,
    pub reward_total_emissioned: u64,
    pub reward_claimed: u64,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub authority: Pubkey,
    pub reward_growth_global_x64: u128,
}

impl RaydiumLiquidityPoolCLMM {
    pub fn get_liquidity_pool_info(data: &[u8]) -> Result<RaydiumLiquidityPoolCLMMData, String> {
        const EXPECTED_LEN: usize = 1544;
        if data.len() != EXPECTED_LEN {
            return Err(format!(
                "CLMM pool data size mismatch. Expected {}, got {}",
                EXPECTED_LEN,
                data.len()
            ));
        }
        let mut offset: usize = DISCRIMINATOR_LEN;
        let read_pubkey = |d: &[u8], o: &mut usize| -> Pubkey {
            let pk = Pubkey::new_from_array(d[*o..*o + 32].try_into().unwrap());
            *o += 32;
            pk
        };
        let read_u8 = |d: &[u8], o: &mut usize| -> u8 {
            let v = d[*o];
            *o += 1;
            v
        };
        let read_u16 = |d: &[u8], o: &mut usize| -> u16 {
            let v = u16::from_le_bytes(d[*o..*o + 2].try_into().unwrap());
            *o += 2;
            v
        };
        let read_i32 = |d: &[u8], o: &mut usize| -> i32 {
            let v = i32::from_le_bytes(d[*o..*o + 4].try_into().unwrap());
            *o += 4;
            v
        };
        let read_u64 = |d: &[u8], o: &mut usize| -> u64 {
            let v = u64::from_le_bytes(d[*o..*o + 8].try_into().unwrap());
            *o += 8;
            v
        };
        let read_u128 = |d: &[u8], o: &mut usize| -> u128 {
            let v = u128::from_le_bytes(d[*o..*o + 16].try_into().unwrap());
            *o += 16;
            v
        };
        let bump = read_u8(data, &mut offset);
        let amm_config = read_pubkey(data, &mut offset);
        let owner = read_pubkey(data, &mut offset);
        let token_mint_0 = read_pubkey(data, &mut offset);
        let token_mint_1 = read_pubkey(data, &mut offset);
        let token_vault_0 = read_pubkey(data, &mut offset);
        let token_vault_1 = read_pubkey(data, &mut offset);
        let observation_key = read_pubkey(data, &mut offset);
        let mint_decimals_0 = read_u8(data, &mut offset);
        let mint_decimals_1 = read_u8(data, &mut offset);
        let tick_spacing = read_u16(data, &mut offset);
        let liquidity = read_u128(data, &mut offset);
        let sqrt_price_x64 = read_u128(data, &mut offset);
        let tick_current = read_i32(data, &mut offset);
        // padding bits
        let padding3 = read_u16(data, &mut offset);
        let padding4 = read_u16(data, &mut offset);
        let fee_growth_global_0_x64 = read_u128(data, &mut offset);
        let fee_growth_global_1_x64 = read_u128(data, &mut offset);
        let protocol_fees_token_0 = read_u64(data, &mut offset);
        let protocol_fees_token_1 = read_u64(data, &mut offset);
        let swap_in_amount_token_0 = read_u128(data, &mut offset);
        let swap_out_amount_token_1 = read_u128(data, &mut offset);
        let swap_in_amount_token_1 = read_u128(data, &mut offset);
        let swap_out_amount_token_0 = read_u128(data, &mut offset);
        let status = read_u8(data, &mut offset);
        // padding bits
        let mut padding = read_u8(data, &mut offset);
        padding = read_u8(data, &mut offset);
        padding = read_u8(data, &mut offset);
        padding = read_u8(data, &mut offset);
        padding = read_u8(data, &mut offset);
        padding = read_u8(data, &mut offset);
        padding = read_u8(data, &mut offset);
        let mut reward_infos: [RewardInfo; 3] = std::array::from_fn(|_| RewardInfo {
            reward_state: 0,
            open_time: 0,
            end_time: 0,
            last_update_time: 0,
            emissions_per_second_x64: 0,
            reward_total_emissioned: 0,
            reward_claimed: 0,
            token_mint: Pubkey::default(),
            token_vault: Pubkey::default(),
            authority: Pubkey::default(),
            reward_growth_global_x64: 0,
        });
        for i in 0..3 {
            // reward_state (u8)
            reward_infos[i].reward_state = read_u8(data, &mut offset);
            // padding [u8;7]
            offset += 7;
            // u64 * 3
            reward_infos[i].open_time = read_u64(data, &mut offset);
            reward_infos[i].end_time = read_u64(data, &mut offset);
            reward_infos[i].last_update_time = read_u64(data, &mut offset);
            // u128
            reward_infos[i].emissions_per_second_x64 = read_u128(data, &mut offset);
            // u64 * 2
            reward_infos[i].reward_total_emissioned = read_u64(data, &mut offset);
            reward_infos[i].reward_claimed = read_u64(data, &mut offset);
            // pubkey * 3
            reward_infos[i].token_mint = read_pubkey(data, &mut offset);
            reward_infos[i].token_vault = read_pubkey(data, &mut offset);
            reward_infos[i].authority = read_pubkey(data, &mut offset);
            // u128
            reward_infos[i].reward_growth_global_x64 = read_u128(data, &mut offset);
        }
        let mut tick_array_bitmap = [0u64; 16];
        for i in 0..16 {
            tick_array_bitmap[i] = read_u64(data, &mut offset);
        }
        let total_fees_token_0 = read_u64(data, &mut offset);
        let total_fees_claimed_token_0 = read_u64(data, &mut offset);
        let total_fees_token_1 = read_u64(data, &mut offset);
        let total_fees_claimed_token_1 = read_u64(data, &mut offset);
        let fund_fees_token_0 = read_u64(data, &mut offset);
        let fund_fees_token_1 = read_u64(data, &mut offset);
        let open_time = read_u64(data, &mut offset);
        let recent_epoch = read_u64(data, &mut offset);
        offset += 24 * 8;
        offset += 32 * 8;
        Ok(RaydiumLiquidityPoolCLMMData {
            bump,
            amm_config,
            owner,
            token_mint_0,
            token_mint_1,
            token_vault_0,
            token_vault_1,
            observation_key,
            mint_decimals_0,
            mint_decimals_1,
            tick_spacing,
            liquidity,
            sqrt_price_x64,
            tick_current,
            fee_growth_global_0_x64,
            fee_growth_global_1_x64,
            protocol_fees_token_0,
            protocol_fees_token_1,
            swap_in_amount_token_0,
            swap_out_amount_token_1,
            swap_in_amount_token_1,
            swap_out_amount_token_0,
            status,
            reward_infos,
            tick_array_bitmap,
            total_fees_token_0,
            total_fees_claimed_token_0,
            total_fees_token_1,
            total_fees_claimed_token_1,
            fund_fees_token_0,
            fund_fees_token_1,
            open_time,
            recent_epoch,
        })
    }

    pub fn amm_config_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.amm_config)
    }

    pub fn owner_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.owner)
    }

    pub fn token_mint_0_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_mint_0)
    }

    pub fn token_mint_1_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_mint_1)
    }

    pub fn token_vault_0_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_vault_0)
    }

    pub fn token_vault_1_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_vault_1)
    }

    pub fn observation_key_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.observation_key)
    }
}

impl RaydiumLiquidityPoolCLMMData {
    pub async fn get_price(&self, client: Arc<RpcClient>) -> Result<f64, String> {
        let sqrt_price_f64 = self.sqrt_price_x64 as f64;
        let sqrt_price = sqrt_price_f64 / (1u128 << 64) as f64;
        let price = sqrt_price * sqrt_price;
        Ok(price)
    }

    pub fn get_tick_price(&self) -> f64 {
        let tick = self.tick_current as f64;
        let base = 1.0001_f64;
        base.powf(tick)
    }

    pub fn get_token_0_amount(&self) -> f64 {
        if self.liquidity == 0 {
            return 0.0;
        }
        let sqrt_price = (self.sqrt_price_x64 as f64) / (1u128 << 64) as f64;
        let liquidity = self.liquidity as f64;
        liquidity / sqrt_price
    }

    pub fn get_token_1_amount(&self) -> f64 {
        if self.liquidity == 0 {
            return 0.0;
        }
        let sqrt_price = (self.sqrt_price_x64 as f64) / (1u128 << 64) as f64;
        let liquidity = self.liquidity as f64;
        liquidity * sqrt_price
    }
}

#[cfg(test)]
mod tests {
    use crate::Raydium;

    use super::*;
    use solana_network_sdk::Solana;
    use solana_network_sdk::types::Mode::MAIN;
    use solana_sdk::signature::Keypair;

    #[tokio::test]
    async fn test_clmm_data_parsing() -> Result<(), Box<dyn std::error::Error>> {
        let solana = Solana::new(MAIN).unwrap();
        let raydium = Raydium::new(Arc::new(solana));
        let pool_data = raydium
            .get_liquidity_pool_clmm("DYZopjL34W4XpxbZaEjsCsXsrt6HbgE8WMCmPF1oPCwM")
            .await;
        println!("Pool Info: {:?}", pool_data);
        Ok(())
    }
    //
}
