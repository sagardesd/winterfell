use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
    #[clap(
        short,
        long,
        env = "MENU_JSON",
        default_value = "data/sample_menu.yaml"
    )]
    pub menu_yaml_file: String,

    #[clap(
        long,
        env = "RUST_LOG",
        default_value = "info",
        help = "Log level. dubug, info, warn, error"
    )]
    pub log_level: String,

    #[clap(long, env = "ENABLE_DEV_LOG_FORMAT", default_value = "true")]
    pub enable_dev_log_format: bool,

    #[clap(long, env = "LISTEN_PORT", default_value = "8081")]
    pub listen_port: u16,

    #[clap(long, env = "BIND_ADDRESS", default_value = "127.0.0.1")]
    pub bind_address: String,

    #[clap(long, env = "DB_HOST", default_value = "127.0.0.1")]
    pub db_host: String,

    #[clap(long, env = "DB_PORT", default_value = "5432")]
    pub db_port: String,

    #[clap(long, env = "DB_USER", default_value = "kratos")]
    pub db_user: String,

    #[clap(long, env = "DB_PASSWORD", default_value = "password")]
    pub db_password: String,

    #[clap(long, env = "DB_NAME", default_value = "nephster")]
    pub db_name: String,
}
