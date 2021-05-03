use once_cell::sync::Lazy;

pub static ENV_CONFIG: Lazy<Config> = Lazy::new(load_config);
pub const PAGE_SIZE: u64 = 4096;
pub struct Config {
    pub page_size: u64,
}

pub fn load_config() -> Config {
    let page_size = dotenv::var("PAGE_SIZE").unwrap().parse().unwrap();

    Config { page_size }
}
