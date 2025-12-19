<h1 align="center">
    Raydium SDK
</h1>
<h4 align="center">
实现了与 Raydium 交互的相关功能.
</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/raydium-sdk/LICENSE"><img src="https://img.shields.io/badge/License-GPL3.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

## 例子

### 解析 Raydium CPMM 流动性池子数据

```rust
#[cfg(test)]
mod tests {

    use crate::{Raydium, Solana};
    use solana_network_sdk::types::Mode::MAIN;
    use std::sync::Arc;

    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let solana = Solana::new(MAIN).unwrap();
        let raydium = Raydium::new(Arc::new(solana));
        let pool_data = raydium
            .get_liquidity_pool_cpmm("8Lq7gz2aEzkMQNfLpYmjv3V8JbD26LRbFd11SnRicCE6")
            .await;
        println!("Pool Info: {:?}", pool_data);
        Ok(())
    }
}
```

### 解析 Raydium CLMM 流动性池子数据

```rust

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
```

### 创建 Raydium 对象

```rust
let rpc = RpcClient::new("rpc url");
let raydium = Raydium::new(Arc::new(rpc));
```

### 根据指定地址获取 v4 pool 的数据

```rust
let rpc = RpcClient::new("rpc url");
let raydium = Raydium::new(Arc::new(rpc));
// 58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2 SOL-USDC pool
let pool_data = raydium.get_liquidity_pool_v4("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
```

### 解析 v4 pool 的数据

```rust
let address: &str = "public key";
match Pubkey::from_str(address) {
    Ok(pool_address) => match self.client.get_account_data(&pool_address).await {
        Ok(v) => match RaydiumLiquidityPoolV4::get_liquidity_pool_info(&v) {
            Ok(pool) => return Ok(pool.clone()),
            Err(e) => return Err(e),
        },
        Err(e) => Err(format!("{:?}", e)),
    },
    Err(e) => {
        return Err(format!("{:?}", e));
    }
}
```

### 获取当前池子的价格

```rust
let rpc = RpcClient::new("".to_string());
let ray = Raydium::new(Arc::new(rpc));
let pool_data = ray.get_liquidity_pool_v4(pool_address).await.unwrap();
let price = pool_data.get_price(Arc::clone(&ray.client)).await;
```
