use bytemuck::{Pod, Zeroable};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_tool::account::get_spl_token_balance;
use solana_tool::unit::conver_balance;
use std::sync::Arc;
use tokio::join;

/// CPMM liquidity pool data size
pub const RAYDIUM_LIQUIDITY_POOL_CPMM_DATA_SIZE: usize = 637;
const DISCRIMINATOR_LEN: usize = 8;

/// CPMM liquidity pool raw data
#[derive(Debug, Copy, Clone, Zeroable)]
pub struct RaydiumLiquidityPoolCPMM {
    pub amm_config: [u8; 32],       // 0-31:
    pub pool_creator: [u8; 32],     // 32-63:
    pub token_0_vault: [u8; 32],    // 64-95: token0
    pub token_1_vault: [u8; 32],    // 96-127: token1
    pub lp_mint: [u8; 32],          // 128-159: LP token mint
    pub token_0_mint: [u8; 32],     // 160-191: token0 mint
    pub token_1_mint: [u8; 32],     // 192-223: token1 mint
    pub token_0_program: [u8; 32],  // 224-255: token0
    pub token_1_program: [u8; 32],  // 256-287: token1
    pub observation_key: [u8; 32],  // 288-319:
    pub auth_bump: u8,              // 320:  bump
    pub status: u8,                 // 321:
    pub lp_mint_decimals: u8,       // 322: LP token
    pub mint_0_decimals: u8,        // 323: token0
    pub mint_1_decimals: u8,        // 324: token1
    pub lp_supply: u64,             // 328-335: LP token
    pub protocol_fees_token_0: u64, // 336-343: token0
    pub protocol_fees_token_1: u64, // 344-351: token1
    pub fund_fees_token_0: u64,     // 352-359: token0
    pub fund_fees_token_1: u64,     // 360-367: token1
    pub open_time: u64,             // 368-375
    pub recent_epoch: u64,          // 376-383
    pub creator_fee_on: u8,         // 384
    pub enable_creator_fee: u8,     // 385
    pub creator_fees_token_0: u64,  // 392-399: token0
    pub creator_fees_token_1: u64,  // 400-407: token1
    pub padding: [u64; 28],         // 632-636:
}

#[derive(Debug, Clone)]
pub struct RaydiumLiquidityPoolCPMMData {
    pub amm_config: Pubkey,
    pub pool_creator: Pubkey,
    pub token_0_vault: Pubkey,
    pub token_1_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,
    pub token_0_program: Pubkey,
    pub token_1_program: Pubkey,
    pub observation_key: Pubkey,
    pub auth_bump: u8,
    pub status: u8,
    pub lp_mint_decimals: u8,
    pub mint_0_decimals: u8,
    pub mint_1_decimals: u8,
    pub lp_supply: u64,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub creator_fee_on: u8,
    pub enable_creator_fee: bool,
    pub creator_fees_token_0: u64,
    pub creator_fees_token_1: u64,
}

impl RaydiumLiquidityPoolCPMM {
    pub fn get_liquidity_pool_info(data: &[u8]) -> Result<RaydiumLiquidityPoolCPMMData, String> {
        const DISCRIMINATOR_LEN: usize = 8;
        const EXPECTED_LEN: usize = 637;
        if data.len() != EXPECTED_LEN {
            return Err(format!(
                "CPMM pool data size mismatch. Expected {}, got {}",
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
        let read_u64 = |d: &[u8], o: &mut usize| -> u64 {
            let v = u64::from_le_bytes(d[*o..*o + 8].try_into().unwrap());
            *o += 8;
            v
        };
        let amm_config = read_pubkey(data, &mut offset);
        let pool_creator = read_pubkey(data, &mut offset);
        let token_0_vault = read_pubkey(data, &mut offset);
        let token_1_vault = read_pubkey(data, &mut offset);
        let lp_mint = read_pubkey(data, &mut offset);
        let token_0_mint = read_pubkey(data, &mut offset);
        let token_1_mint = read_pubkey(data, &mut offset);
        let token_0_program = read_pubkey(data, &mut offset);
        let token_1_program = read_pubkey(data, &mut offset);
        let observation_key = read_pubkey(data, &mut offset);
        let auth_bump = data[offset];
        let status = data[offset + 1];
        let lp_mint_decimals = data[offset + 2];
        let mint_0_decimals = data[offset + 3];
        let mint_1_decimals = data[offset + 4];
        offset += 5;
        let lp_supply = read_u64(data, &mut offset);
        let protocol_fees_token_0 = read_u64(data, &mut offset);
        let protocol_fees_token_1 = read_u64(data, &mut offset);
        let fund_fees_token_0 = read_u64(data, &mut offset);
        let fund_fees_token_1 = read_u64(data, &mut offset);
        let open_time = read_u64(data, &mut offset);
        let recent_epoch = read_u64(data, &mut offset);
        let creator_fee_on = data[offset];
        let enable_creator_fee = data[offset + 1] != 0;
        offset += 2;
        let padding1: [u8; 6] = data[offset..offset + 6].try_into().unwrap();
        offset += 6;
        let creator_fees_token_0 = read_u64(data, &mut offset);
        let creator_fees_token_1 = read_u64(data, &mut offset);
        let mut padding = [0u64; 28];
        for i in 0..28 {
            let end = offset + 8;
            if end > data.len() {
                break;
            }
            padding[i] = read_u64(data, &mut offset);
        }
        Ok(RaydiumLiquidityPoolCPMMData {
            amm_config,
            pool_creator,
            token_0_vault,
            token_1_vault,
            lp_mint,
            token_0_mint,
            token_1_mint,
            token_0_program,
            token_1_program,
            observation_key,
            auth_bump,
            status,
            lp_mint_decimals,
            mint_0_decimals,
            mint_1_decimals,
            lp_supply,
            protocol_fees_token_0,
            protocol_fees_token_1,
            fund_fees_token_0,
            fund_fees_token_1,
            open_time,
            recent_epoch,
            creator_fee_on,
            enable_creator_fee,
            creator_fees_token_0,
            creator_fees_token_1,
        })
    }

    pub fn amm_config_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.amm_config)
    }

    pub fn pool_creator_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.pool_creator)
    }

    pub fn token_0_vault_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_0_vault)
    }

    pub fn token_1_vault_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_1_vault)
    }

    pub fn lp_mint_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.lp_mint)
    }

    pub fn token_0_mint_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_0_mint)
    }

    pub fn token_1_mint_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_1_mint)
    }

    pub fn token_0_program_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_0_program)
    }

    pub fn token_1_program_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.token_1_program)
    }

    pub fn observation_key_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.observation_key)
    }
}

impl RaydiumLiquidityPoolCPMMData {
    pub async fn get_price(&self, client: Arc<RpcClient>) -> Result<f64, String> {
        let base_vault_address = self.token_0_vault;
        let quote_vault_address = self.token_1_vault;
        let (base_token_balance, quote_token_balance) = join!(
            get_spl_token_balance(Arc::clone(&client), base_vault_address),
            get_spl_token_balance(Arc::clone(&client), quote_vault_address)
        );
        let base_balance = match base_token_balance {
            Ok(balance) => balance,
            Err(e) => {
                return Err(format!("Failed to get base token balance: {}", e));
            }
        };
        let quote_balance = match quote_token_balance {
            Ok(balance) => balance,
            Err(e) => {
                return Err(format!("Failed to get quote token balance: {}", e));
            }
        };
        let base_amount = conver_balance(base_balance, self.mint_0_decimals);
        let quote_amount = conver_balance(quote_balance, self.mint_1_decimals);
        if base_amount > 0.0 {
            let price = quote_amount / base_amount;
            Ok(price)
        } else {
            Err("Base token amount is zero, cannot calculate price".to_string())
        }
    }
}

#[cfg(test)]
mod tests {

    use solana_network_client::SolanaClient;

    use crate::Raydium;
    use std::sync::Arc;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let solana_client = SolanaClient::new(solana_network_client::Mode::MAIN).unwrap();
        let raydium = Raydium::new(Arc::new(solana_client));
        let pool_data = raydium
            .get_liquidity_pool_cpmm("8Lq7gz2aEzkMQNfLpYmjv3V8JbD26LRbFd11SnRicCE6")
            .await;
        println!("Pool Info: {:?}", pool_data);
        Ok(())
    }
}
