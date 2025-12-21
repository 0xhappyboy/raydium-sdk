use bytemuck::{Pod, Zeroable};
use solana_sdk::pubkey::Pubkey;
use std::convert::TryInto;

pub const LAUNCHPAD_POOL_STATE_DATA_SIZE: usize = 429;
const DISCRIMINATOR_LEN: usize = 8;

unsafe impl Pod for LaunchpadPool {}
unsafe impl Zeroable for LaunchpadPool {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LaunchpadPool {
    pub epoch: u64,         // 0-7: Account update epoch
    pub auth_bump: u8,      // 8: Bump seed
    pub status: u8,         // 9: Pool status (0: Fund, 1: Migrate, 2: Trade)
    pub base_decimals: u8,  // 10: Base token decimals
    pub quote_decimals: u8, // 11: Quote token decimals
    pub migrate_type: u8,   // 12: Migrate type (0: AMM, 1: CPSWAP)
    _padding1: [u8; 3],
    pub supply: u64,                   // 16-23: Supply of the pool base token
    pub total_base_sell: u64,          // 24-31: Total sell amount of the base token
    pub virtual_base: u64,             // 32-39: Virtual base amount
    pub virtual_quote: u64,            // 40-47: Virtual quote amount
    pub real_base: u64,                // 48-55: Actual base token amount in the pool
    pub real_quote: u64,               // 56-63: Actual quote token amount in the pool
    pub total_quote_fund_raising: u64, // 64-71: Total quote fund raising
    pub quote_protocol_fee: u64,       // 72-79: Protocol fees in quote tokens
    pub platform_fee: u64,             // 80-87: Platform fees in quote tokens
    pub migrate_fee: u64,              // 88-95: Migrate fee
    // VestingSchedule (40 bytes)
    pub total_locked_amount: u64,    // 96-103: Total locked amount
    pub cliff_period: u64,           // 104-111: Cliff period in seconds
    pub unlock_period: u64,          // 112-119: Unlock period in seconds
    pub start_time: u64,             // 120-127: Start time
    pub allocated_share_amount: u64, // 128-135: Allocated share amount
    pub global_config: [u8; 32],     // 136-167: Global config address
    pub platform_config: [u8; 32],   // 168-199: Platform config address
    pub base_mint: [u8; 32],         // 200-231: Base mint address
    pub quote_mint: [u8; 32],        // 232-263: Quote mint address
    pub base_vault: [u8; 32],        // 264-295: Base token vault
    pub quote_vault: [u8; 32],       // 296-327: Quote token vault
    pub creator: [u8; 32],           // 328-359: Creator address
    pub token_program_flag: u8,      // 360: Token program flags
    pub amm_creator_fee_on: u8,      // 361: Creator fee on (0: QuoteToken, 1: BothToken)
    pub padding: [u8; 59],           // 362-420: Padding
}

#[derive(Debug, Clone)]
pub struct LaunchpadPoolData {
    pub epoch: u64,
    pub auth_bump: u8,
    pub status: PoolStatus,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub migrate_type: MigrateType,
    pub supply: u64,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base: u64,
    pub real_quote: u64,
    pub total_quote_fund_raising: u64,
    pub quote_protocol_fee: u64,
    pub platform_fee: u64,
    pub migrate_fee: u64,
    pub vesting_schedule: VestingSchedule,
    pub global_config: Pubkey,
    pub platform_config: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub creator: Pubkey,
    pub token_program_flag: TokenProgramFlagBits,
    pub amm_creator_fee_on: AmmCreatorFeeOn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolStatus {
    Fund,
    Migrate,
    Trade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrateType {
    AMM,
    CPSWAP,
}

#[derive(Debug, Clone)]
pub struct TokenProgramFlagBits {
    pub base_token_program: TokenProgramFlag,
    pub quote_token_program: TokenProgramFlag,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenProgramFlag {
    SPLTokenProgram,
    TokenProgram2022,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmmCreatorFeeOn {
    QuoteToken,
    BothToken,
}

#[derive(Debug, Clone)]
pub struct VestingSchedule {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
    pub start_time: u64,
    pub allocated_share_amount: u64,
}

impl LaunchpadPool {
    pub fn get_liquidity_pool_info(data: &[u8]) -> Result<LaunchpadPoolData, String> {
        if data.len() != LAUNCHPAD_POOL_STATE_DATA_SIZE {
            return Err(format!(
                "Launchpad pool data size mismatch. Expected {}, got {}",
                LAUNCHPAD_POOL_STATE_DATA_SIZE,
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
        let read_u64 = |d: &[u8], o: &mut usize| -> u64 {
            let v = u64::from_le_bytes(d[*o..*o + 8].try_into().unwrap());
            *o += 8;
            v
        };
        let epoch = read_u64(data, &mut offset);
        let auth_bump = read_u8(data, &mut offset);
        let status_byte = read_u8(data, &mut offset);
        let base_decimals = read_u8(data, &mut offset);
        let quote_decimals = read_u8(data, &mut offset);
        let migrate_type_byte = read_u8(data, &mut offset);
        let supply = read_u64(data, &mut offset);
        let total_base_sell = read_u64(data, &mut offset);
        let virtual_base = read_u64(data, &mut offset);
        let virtual_quote = read_u64(data, &mut offset);
        let real_base = read_u64(data, &mut offset);
        let real_quote = read_u64(data, &mut offset);
        let total_quote_fund_raising = read_u64(data, &mut offset);
        let quote_protocol_fee = read_u64(data, &mut offset);
        let platform_fee = read_u64(data, &mut offset);
        let migrate_fee = read_u64(data, &mut offset);
        let total_locked_amount = read_u64(data, &mut offset);
        let cliff_period = read_u64(data, &mut offset);
        let unlock_period = read_u64(data, &mut offset);
        let start_time = read_u64(data, &mut offset);
        let allocated_share_amount = read_u64(data, &mut offset);
        let global_config = read_pubkey(data, &mut offset);
        let platform_config = read_pubkey(data, &mut offset);
        let base_mint = read_pubkey(data, &mut offset);
        let quote_mint = read_pubkey(data, &mut offset);
        let base_vault = read_pubkey(data, &mut offset);
        let quote_vault = read_pubkey(data, &mut offset);
        let creator = read_pubkey(data, &mut offset);
        let token_program_flag_byte = read_u8(data, &mut offset);
        let amm_creator_fee_on_byte = read_u8(data, &mut offset);
        let remaining_padding = LAUNCHPAD_POOL_STATE_DATA_SIZE - offset;
        if remaining_padding > 0 {
            offset += remaining_padding;
        }
        if offset != data.len() {
            return Err(format!(
                "Data parsing incomplete. Expected offset {}, got {}",
                data.len(),
                offset
            ));
        }
        let status = match status_byte {
            0 => PoolStatus::Fund,
            1 => PoolStatus::Migrate,
            2 => PoolStatus::Trade,
            _ => return Err(format!("Invalid pool status: {}", status_byte)),
        };
        let migrate_type = match migrate_type_byte {
            0 => MigrateType::AMM,
            1 => MigrateType::CPSWAP,
            _ => return Err(format!("Invalid migrate type: {}", migrate_type_byte)),
        };
        let base_token_program = if token_program_flag_byte & 0b1 == 0 {
            TokenProgramFlag::SPLTokenProgram
        } else {
            TokenProgramFlag::TokenProgram2022
        };
        let quote_token_program = if token_program_flag_byte & 0b10 == 0 {
            TokenProgramFlag::SPLTokenProgram
        } else {
            TokenProgramFlag::TokenProgram2022
        };
        let amm_creator_fee_on = match amm_creator_fee_on_byte {
            0 => AmmCreatorFeeOn::QuoteToken,
            1 => AmmCreatorFeeOn::BothToken,
            _ => {
                return Err(format!(
                    "Invalid creator fee on value: {}",
                    amm_creator_fee_on_byte
                ));
            }
        };
        Ok(LaunchpadPoolData {
            epoch,
            auth_bump,
            status,
            base_decimals,
            quote_decimals,
            migrate_type,
            supply,
            total_base_sell,
            virtual_base,
            virtual_quote,
            real_base,
            real_quote,
            total_quote_fund_raising,
            quote_protocol_fee,
            platform_fee,
            migrate_fee,
            vesting_schedule: VestingSchedule {
                total_locked_amount,
                cliff_period,
                unlock_period,
                start_time,
                allocated_share_amount,
            },
            global_config,
            platform_config,
            base_mint,
            quote_mint,
            base_vault,
            quote_vault,
            creator,
            token_program_flag: TokenProgramFlagBits {
                base_token_program,
                quote_token_program,
            },
            amm_creator_fee_on,
        })
    }

    pub fn global_config_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.global_config)
    }

    pub fn platform_config_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.platform_config)
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

    pub fn creator_pubkey(&self) -> Pubkey {
        Pubkey::new_from_array(self.creator)
    }
}

impl LaunchpadPoolData {
    pub fn get_price(&self) -> f64 {
        if self.virtual_base == 0 {
            return 0.0;
        }
        self.virtual_quote as f64 / self.virtual_base as f64
    }

    pub fn get_real_price(&self) -> f64 {
        if self.real_base == 0 {
            return 0.0;
        }
        self.real_quote as f64 / self.real_base as f64
    }

    pub fn get_total_value(&self) -> f64 {
        let base_value = self.real_base as f64 * self.get_real_price();
        base_value + self.real_quote as f64
    }

    pub fn is_funding(&self) -> bool {
        matches!(self.status, PoolStatus::Fund)
    }

    pub fn is_tradable(&self) -> bool {
        matches!(self.status, PoolStatus::Trade)
    }

    pub fn get_funding_progress(&self) -> f64 {
        if self.total_quote_fund_raising == 0 {
            return 0.0;
        }
        (self.real_quote as f64 / self.total_quote_fund_raising as f64) * 100.0
    }

    pub fn get_unlocked_amount(&self) -> u64 {
        if self.vesting_schedule.total_locked_amount == 0 {
            return 0;
        }
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if current_time < self.vesting_schedule.start_time + self.vesting_schedule.cliff_period {
            0
        } else if current_time
            >= self.vesting_schedule.start_time
                + self.vesting_schedule.cliff_period
                + self.vesting_schedule.unlock_period
        {
            self.vesting_schedule.total_locked_amount
        } else {
            let elapsed = current_time
                - (self.vesting_schedule.start_time + self.vesting_schedule.cliff_period);
            let unlock_ratio = elapsed as f64 / self.vesting_schedule.unlock_period as f64;
            (self.vesting_schedule.total_locked_amount as f64 * unlock_ratio.min(1.0)) as u64
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_network_client::SolanaClient;

    use crate::Raydium;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_clmm_data_parsing() -> Result<(), Box<dyn std::error::Error>> {
        let solana_client = SolanaClient::new(solana_network_client::Mode::MAIN).unwrap();
        let raydium = Raydium::new(Arc::new(solana_client));
        let pool_data = raydium
            .get_liquidity_pool_launchpad("GSxb28CtEf9vJHEoB9D2NoNwbbkj8SxQN3WN86qvMULZ")
            .await;
        println!("Pool Info: {:?}", pool_data);
        Ok(())
    }
}
