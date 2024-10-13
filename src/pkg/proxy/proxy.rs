use crate::pkg::config::client::new_rpc_client;
use crate::pkg::config::config;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionInput;
use alloy::{primitives::*, rpc::types::TransactionRequest};
use eyre::{eyre, Result};
use futures::join;
use std::boxed::Box;
use std::str::FromStr;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, DetectResult>> = Mutex::new(HashMap::new());
}

// contants
const EIP_1967_LOGIC_SLOT: &str =
    "0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc";
const EIP_1967_BEACON_SLOT: &str =
    "0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50";
const OPEN_ZEPPELIN_IMPLEMENTATION_SLOT: &str =
    "0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3";
const EIP_1822_LOGIC_SLOT: &str =
    "0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7";

// methods
const EIP_1167_BEACON_METHODS: [&str; 2] = [
    "0x5c60da1b00000000000000000000000000000000000000000000000000000000",
    "0xda52571600000000000000000000000000000000000000000000000000000000",
];
const EIP_897_METHOD: [&str; 1] =
    ["0x5c60da1b00000000000000000000000000000000000000000000000000000000"];
const GNOSIS_SAFE_PROXY_METHOD: [&str; 1] =
    ["0xa619486e00000000000000000000000000000000000000000000000000000000"];
const COMPTROLLER_PROXY_METHOD: [&str; 1] =
    ["0xbb82aa5e00000000000000000000000000000000000000000000000000000000"];

// EIP-1167 Bytecode
const EIP_1167_BYTECODE_PREFIX: &str = "0x363d3d373d3d3d363d";
const EIP_1167_BYTECODE_SUFFIX: &str = "57fd5bf3";

// proxy types
#[derive(Debug)]
enum ProxyType {
    Eip1167,
    Eip1967Beacon,
    OpenZeppelin,
    Eip1822,
    Eip897,
    GnosisSafe,
    Compound,
}

// detection result
#[derive(Debug, Clone)]
pub struct DetectResult {
    pub standard: String,
    pub target: Option<Address>,
}

// detector
pub struct ProxyDetector {
    provider: Box<dyn Provider>,
}

impl ProxyDetector {
    pub async fn new(chain_config: &config::ChainConfig) -> Result<Self> {
        let provider = new_rpc_client(chain_config).await?;
        Ok(Self { provider })
    }

    pub async fn detect_proxy_target(&self, proxy_address: &str) -> Result<DetectResult> {
        let address = proxy_address
            .parse::<Address>()
            .map_err(|e| eyre!("Invalid proxy address {}: {}", proxy_address, e))?;

        // Check cache first
        let mut cache = CACHE.lock().unwrap();
        if let Some(result) = cache.get(proxy_address) {
            return Ok(result.clone());
        }


        // 使用 join! 宏并行执行 futures
        let (
            eip_1167,
            eip_1967_beacon,
            open_zeppelin,
            eip_1822,
            eip_897,
            eip_1967_logic,
            gnosis_safe,
            compound,
        ) = join!(
            async {
                (
                    "eip_1167",
                    self._get_code_and_parse_1167_minimal(address).await,
                )
            },
            async { ("eip_1967_beacon", self._get_eip_1967_beacon(address).await) },
            async {
                (
                    "open_zeppelin",
                    self._get_open_zeppelin_implementation(address).await,
                )
            },
            async { ("eip_1822", self._get_eip_1822_logic(address).await) },
            async {
                (
                    "eip_897",
                    self._request_from_eth_call(address, EIP_897_METHOD[0].to_string())
                        .await,
                )
            },
            async { ("eip_1967_logic", self._get_eip_1967_logic(address).await) },
            async {
                (
                    "gnosis_safe",
                    self._request_from_eth_call(address, GNOSIS_SAFE_PROXY_METHOD[0].to_string())
                        .await,
                )
            },
            async {
                (
                    "compound",
                    self._request_from_eth_call(address, COMPTROLLER_PROXY_METHOD[0].to_string())
                        .await,
                )
            }
        );

        let results = vec![
            eip_1167,
            eip_1967_beacon,
            open_zeppelin,
            eip_1822,
            eip_897,
            eip_1967_logic,
            gnosis_safe,
            compound,
        ];

        for (key, result) in results {
            if let Ok(Some(target)) = result {
                let detect_reuslt = DetectResult {
                    standard: key.to_string(),
                    target: Some(target),
                };
                cache.insert(proxy_address.to_string(), detect_reuslt.clone());
                return Ok(detect_reuslt);
            }
        }
        let detect_result= DetectResult {
            standard: "".to_string(),
            target: None,
        };

        cache.insert(proxy_address.to_string(), detect_result.clone());
        Ok(detect_result)
    }

    // read address from the storage slot
    async fn _read_storage_slot(&self, proxy: Address, slot: &str) -> Result<Option<Address>> {
        let slot = slot.parse::<U256>()?;
        let storage = self.provider.get_storage_at(proxy.clone(), slot).await?;
        let addr = Self::_read_address(&B256::from(storage))?;
        Ok(Some(addr))
    }

    // request address by eth_call
    async fn _request_from_eth_call(
        &self,
        proxy: Address,
        data: String,
    ) -> Result<Option<Address>> {
        let call = TransactionRequest::default()
            .to(proxy.clone())
            .input(TransactionInput::new(data.parse::<Bytes>()?));
        let response = self.provider.call(&call).await?;

        if response.0.len() < 32 {
            return Err(eyre!("Response is less than 32 bytes"));
        }

        // ensure the response is 32 bytes long for B256
        let response_bytes: [u8; 32] = response.0.as_ref()[..32]
            .try_into()
            .map_err(|_| eyre!("not 32 bytes"))?;

        let address = Self::_read_address(&B256::new(response_bytes))?;
        Ok(Some(address))
    }

    async fn _get_code_and_parse_1167_minimal(&self, proxy: Address) -> Result<Option<Address>> {
        let bytecode = self.provider.get_code_at(proxy.clone()).await?.0;
        let bytecode_str = hex::encode(bytecode);

        if !bytecode_str.starts_with(&EIP_1167_BYTECODE_PREFIX.trim_start_matches("0x")) {
            return Err(eyre!("Not an EIP-1167 bytecode"));
        }

        let prefix_len = EIP_1167_BYTECODE_PREFIX.len() - 2; // remove "0x"
        let push_n_hex = &bytecode_str[prefix_len..prefix_len + 2];
        let address_length = usize::from_str_radix(push_n_hex, 16)? - 0x5f;

        if address_length < 1 || address_length > 20 {
            return Err(eyre!("Invalid address length in EIP-1167 bytecode"));
        }

        let address_start = prefix_len + 2;
        let address_end = address_start + address_length * 2;
        let address_hex = &bytecode_str[address_start..address_end];
        let suffix = &bytecode_str[address_end + 22..];

        if !suffix.starts_with(&EIP_1167_BYTECODE_SUFFIX.trim_start_matches("0x")) {
            return Err(eyre!("Invalid EIP-1167 bytecode suffix"));
        }

        let addr = Address::from_str(&format!("0x{}", address_hex.trim_start_matches('0')))?;
        Ok(Some(addr))
    }

    // get logic address for EIP-1967
    async fn _get_eip_1967_logic(&self, proxy: Address) -> Result<Option<Address>> {
        self._read_storage_slot(proxy, EIP_1967_LOGIC_SLOT).await
    }

    // beacon address for EIP-1967
    async fn _get_eip_1967_beacon(&self, proxy: Address) -> Result<Option<Address>> {
        let beacon = self._read_storage_slot(proxy, EIP_1967_BEACON_SLOT).await?;
        if let Some(beacon_addr) = beacon {
            if let Ok(target) = self
                ._request_from_eth_call(beacon_addr, EIP_1167_BEACON_METHODS[0].to_string())
                .await
            {
                return Ok(target);
            }
            if let Ok(target) = self
                ._request_from_eth_call(beacon_addr, EIP_1167_BEACON_METHODS[1].to_string())
                .await
            {
                return Ok(target);
            }
        }
        Ok(None)
    }

    // OpenZeppelin implementation address
    async fn _get_open_zeppelin_implementation(&self, proxy: Address) -> Result<Option<Address>> {
        self._read_storage_slot(proxy, OPEN_ZEPPELIN_IMPLEMENTATION_SLOT)
            .await
    }

    // EIP-1822 logic adddress
    async fn _get_eip_1822_logic(&self, proxy: Address) -> Result<Option<Address>> {
        self._read_storage_slot(proxy, EIP_1822_LOGIC_SLOT).await
    }

    // read and validate the address
    fn _read_address(storage: &B256) -> Result<Address> {
        if storage.is_empty() || storage.len() < 20 {
            return Err(eyre!("Invalid address value"));
        }
        let addr = Address::from_slice(&storage[storage.len() - 20..]);
        if addr == Address::ZERO {
            return Err(eyre!("Invalid address value: zero address"));
        }
        Ok(addr)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::CONFIG;

    #[tokio::test]
    async fn test_proxy() {
        let chain_config = CONFIG.chains.get(0).expect("chain config not found");

        let detector = match ProxyDetector::new(chain_config).await {
            Ok(d) => d,
            Err(e) => panic!("{}", e),
        };

        let non_proxy= "0xd4e96ef8eee8678dbff4d535e033ed1a4f7605b7";
        let result = match detector
            .detect_proxy_target(&non_proxy)
            .await
        {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        println!("{} {:?}", non_proxy, result);

        let eip_1967 = "0x4aa799c5dfc01ee7d790e3bf1a7c2257ce1dceff";
        let result = match detector
            .detect_proxy_target(&eip_1967)
            .await
        {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        println!("{} {:?}", eip_1967, result);


        let eip_897 = "0x8260b9eC6d472a34AD081297794d7Cc00181360a";
        let specific_result = match detector
            ._request_from_eth_call(
                Address::from_str(&eip_897)
                    .expect("wrong address"),
                EIP_897_METHOD[0].to_string(),
            )
            .await
        {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        println!("{} {:?}", eip_897, specific_result);
    }
}
