use ethers::core::k256::elliptic_curve::consts::False;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use humantime_serde::re::humantime::Duration as HumanDuration;
use log::LevelFilter;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub network: NetworkConfig,
    pub bot_settings: SoftSettings,
    pub wallet_management: WalletManagementConfig,
    pub contract_deployment: ContractDeploymentConfig,
    pub logging: LoggingConfig,
    pub proxies: Option<Vec<ProxyConfig>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    pub rpc_url: String,
    pub chain_id: u64,
    pub explorer_url: Option<String>,
    pub gas_price_gwei: Option<u64>,
    pub default_gas_limit: u64,
    pub confirmations: u64,
    #[serde(default = "default_transaction_timeout", with = "humantime_serde")]
    pub transaction_timeout: Duration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SoftSettings {
    #[serde(default = "default_delay_range", with = "humantime_serde_range")]
    pub delay_range: (Duration, Duration),
    pub faucet_url: Option<String>,
    pub faucet_attempts: u32,
    pub random_send_amount: (f64, f64),
    pub min_stt_balance_for_action: f64,
    pub shuffle_wallets: bool,
    pub tasks_per_wallet: usize,
    pub global_tasks_limit: Option<usize>,
    pub use_proxies: bool, 
}

#[derive(Debug, Deserialize, Clone)]
pub struct WalletManagementConfig {
    pub wallet_file_path: String,
    pub auto_save_interval: Option<u64>,
    pub update_balance: bool,
    pub enable_auto_activation: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractDeploymentConfig {
    pub erc20_bytecode_path: Option<String>,
    pub erc20_abi_path: Option<String>,
    pub erc721_bytecode_path: Option<String>,
    pub erc721_abi_path: Option<String>,
    pub default_erc20_initial_supply: Option<String>,
    pub default_erc721_name: Option<String>,
    pub default_erc721_symbol: Option<String>,
    pub default_erc721_base_uri: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    #[serde(default ="default_log_level")]
    pub level: String,
    #[serde(default)]
    pub enable_file_logging: bool,
    pub log_file_path: Option<String>,
    #[serde(default = "default_console_logging")]
    pub enable_console_logging: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig { level: (default_log_level()), enable_file_logging: (false), log_file_path: (None), enable_console_logging: (default_console_logging())}
    }
}

fn default_log_level() -> String {"info".to_string()}
fn default_console_logging() -> bool {true}

#[derive(Debug, Deserialize, Clone)]
pub struct ProxyConfig {
    pub url: String,
    pub auth_user: Option<String>,
    pub auth_password: Option<String>,
    pub proxy_type: ProxyType,
} 

#[derive(Debug, Deserialize, Clone)]
pub enum ProxyType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "socks5")]
    Socks5,
}

fn default_delay_range() -> (Duration, Duration) {
    (Duration::from_secs(5), Duration::from_secs(60))
} 

fn default_transaction_timeout() -> Duration {
    Duration::from_secs(90)
}

mod humantime_serde_range {
    use serde::{self, Deserialize, Deserializer, Serializer, Serialize};
    use std::time::Duration;
    use humantime_serde::Serde as HumanSerde;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(Duration, Duration), D::Error>
    where
        D: Deserializer<'de>,
    {
        let tuple_strings: (String, String) = Deserialize::deserialize(deserializer)?;
        let d1 = humantime_serde::re::humantime::parse_duration(&tuple_strings.0)
            .map_err(serde::de::Error::custom)?;
        let d2 = humantime_serde::re::humantime::parse_duration(&tuple_strings.1)
            .map_err(serde::de::Error::custom)?;

        Ok((d1, d2))
    }
    pub fn serialize<S>(value: &(Duration, Duration), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s1 = humantime_serde::re::humantime::format_duration(value.0).to_string();
        let s2 = humantime_serde::re::humantime::format_duration(value.1).to_string();

        (s1, s2).serialize(serializer)
    }
}

impl AppConfig {
    pub fn new() -> Result<Self, config::ConfigError> {
        let s = config::Config::builder()
            .add_source(config::File::with_name("data/default_config.toml").required(false))
            .add_source(config::File::with_name("data/config.toml").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .add_source(config::Environment::with_prefix("SOMNIA_BOT").separator("__"))
            .build()?;
        s.try_deserialize()
    }
}