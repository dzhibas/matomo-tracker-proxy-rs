use std::env;

#[derive(Clone, Default, Debug)]
pub struct Config {
    pub matomo_url: Option<String>,
    pub matomo_auth_token: Option<String>,
    pub matomo_timeout: u16,
    pub matomo_process_url: Option<String>,
    pub use_redis: bool,
    pub redis_host: Option<String>,
    pub redis_auth: Option<String>,
    pub redis_port: u16,
    pub redis_queue_name: Option<String>,
    pub redis_worker_amount: u8,
    pub cache_max_entries: u32,
    pub use_redis_sentinel: bool,
    pub redis_sentinel_hosts: Option<String>,
    pub redis_sentinel_auth: Option<String>,
    pub redis_sentinel_master: Option<String>,
    pub redis_sentinel_port: u16,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn load_from_env() -> Self {
        let mut config = Config::new();

        config.matomo_url = env::var("MATOMO_URL").ok();
        config.matomo_auth_token = env::var("MATOMO_AUTH_TOKEN").ok();
        config.matomo_process_url = env::var("MATOMO_PROCESS_URL").ok();

        config.matomo_timeout =
            env::var("MATOMO_TIMEOUT").map_or(5, |v| v.parse::<u16>().unwrap_or(5));

        config.use_redis =
            env::var("USE_REDIS_QUEUE").map_or(false, |v| v.eq_ignore_ascii_case("true"));
        config.redis_host = env::var("REDIS_HOST").ok();
        config.redis_auth = env::var("REDIS_AUTH").ok();
        config.redis_queue_name = env::var("REDIS_QUEUE_NAME")
            .ok()
            .or_else(|| Some(String::from("trackingQueueV1")));
        config.redis_port =
            env::var("REDIS_PORT").map_or(6379, |v| v.parse::<u16>().unwrap_or(6379));

        config.redis_worker_amount =
            env::var("REDIS_WORKER_AMOUNT").map_or(2, |v| v.parse::<u8>().unwrap_or(2));

        config.cache_max_entries =
            env::var("REDIS_WORKER_AMOUNT").map_or(0, |v| v.parse::<u32>().unwrap_or(0));

        config.use_redis_sentinel =
            env::var("USE_REDIS_SENTINEL").map_or(false, |v| v.eq_ignore_ascii_case("true"));
        config.redis_sentinel_hosts = env::var("REDIS_SENTINEL_HOSTS").ok();
        config.redis_sentinel_auth = env::var("REDIS_SENTINEL_AUTH").ok();
        config.redis_sentinel_master = env::var("REDIS_SENTINEL_MASTER")
            .ok()
            .or_else(|| Some(String::from("mymaster")));
        config.redis_sentinel_port =
            env::var("REDIS_PORT").map_or(26379, |v| v.parse::<u16>().unwrap_or(26379));

        config
    }
}
