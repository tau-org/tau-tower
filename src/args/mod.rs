use clap::Parser;
use crate::util::ip::{validate_port, parse_port, validate_endpoint};

#[derive(Parser)]
#[command(name = "tau-tower")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command( about = "Webradio server, distributes audio stream from a tau-radio client")]
pub(crate) struct Args {
    /// Webradio username
    #[arg(long)]
    pub username: Option<String>,

    /// Webradio password
    #[arg(long)]
    pub password: Option<String>,

    /// Stream port
    #[arg(short='l', long, value_parser=|p: &str| { validate_port(parse_port(p).unwrap()) })]
    pub listen_port: Option<u16>,
    
    /// Stream port
    #[arg(short='p', long, value_parser=|p: &str| { validate_port(parse_port(p).unwrap()) })]
    pub mount_port: Option<u16>,

    #[arg(short, long, value_parser=|m: &str| { validate_endpoint(m) })]
    pub mount: Option<String>,

    #[arg(long)]
    pub reset_config: bool,
}

