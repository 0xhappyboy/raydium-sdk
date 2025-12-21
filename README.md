<h1 align="center">
    Raydium SDK
</h1>
<h4 align="center">
Implemented functions related to interaction with raydium.
</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/raydium-sdk/LICENSE"><img src="https://img.shields.io/badge/License-GPL3.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

## Example

### Retrieves information about the specified Raydium launcher pool.

```rust
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
```

### Parse Raydium CPMM pool data

```rust
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
```

### Parse Raydium CLMM pool data

```rust
#[cfg(test)]
mod tests {
    use solana_network_client::SolanaClient;

    use crate::Raydium;

    use super::*;

    #[tokio::test]
    async fn test_clmm_data_parsing() -> Result<(), Box<dyn std::error::Error>> {
        let solana_client = SolanaClient::new(solana_network_client::Mode::MAIN).unwrap();
        let raydium = Raydium::new(Arc::new(solana_client));
        let pool_data = raydium
            .get_liquidity_pool_clmm("DYZopjL34W4XpxbZaEjsCsXsrt6HbgE8WMCmPF1oPCwM")
            .await;
        println!("Pool Info: {:?}", pool_data);
        Ok(())
    }
}
```

### Parse Raydium V4 pool data

```rust

#[cfg(test)]
mod tests {

    use solana_network_client::SolanaClient;

    use crate::Raydium;
    use std::sync::Arc;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let solana_client = SolanaClient::new(solana_network_client::Mode::MAIN).unwrap();
        let raydium = Raydium::new(Arc::new(solana_client));
        // 58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2 SOL-USDC pool
        let pool_data = raydium
            .get_liquidity_pool_v4("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2")
            .await;
        println!("Pool Info: {:?}", pool_data);
        Ok(())
    }
}
```
