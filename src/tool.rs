/// reader associated with byte arrays
pub mod reader {
    use solana_sdk::pubkey::Pubkey;
    pub fn r_u8(data: &[u8], offset: usize) -> u8 {
        if data.len() <= offset {
            return 0;
        }
        data[offset]
    }
    pub fn r_u16(data: &[u8], offset: usize) -> u16 {
        if data.len() < offset + 2 {
            return 0;
        }
        u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
    }
    pub fn r_u32(data: &[u8], offset: usize) -> u32 {
        if data.len() < offset + 4 {
            return 0;
        }
        u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
    }
    pub fn r_u64(data: &[u8], offset: usize) -> u64 {
        if data.len() < offset + 8 {
            return 0;
        }
        u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap())
    }
    pub fn r_u128(data: &[u8], offset: usize) -> u128 {
        if data.len() < offset + 16 {
            return 0;
        }
        u128::from_le_bytes(data[offset..offset + 16].try_into().unwrap())
    }
    pub fn r_pubkey(data: &[u8], offset: usize) -> Pubkey {
        if data.len() < offset + 32 {
            return Pubkey::default();
        }
        Pubkey::new_from_array(data[offset..offset + 32].try_into().unwrap())
    }
    pub fn r_string(data: &[u8], offset: usize) -> String {
        if data.len() <= offset {
            return String::new();
        }

        let len = data[offset] as usize;
        if data.len() < offset + 1 + len {
            return String::new();
        }

        String::from_utf8_lossy(&data[offset + 1..offset + 1 + len]).to_string()
    }
    pub fn r_bool(data: &[u8], offset: usize) -> bool {
        r_u8(data, offset) != 0
    }
}
