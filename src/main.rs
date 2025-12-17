mod server;
mod threads;
mod config;
mod args;
mod util;

use std::net::{Ipv4Addr, SocketAddr};
use tokio::sync::broadcast;
use tokio::task;
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use hyper::body::Bytes;
use clap::Parser;
use inline_colorization::*;

use crate::threads::{http, ws};
use crate::util::{Headers, Credentials};
use crate::config::Config;
use crate::args::Args;

const END_POINT: &str = "tau.ogg";

// const MODE: ServerMode = ServerMode::WebSocket;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();
  let config = Config::load_or_create(args.reset_config).map(|c| c.merge_cli_args(&args))?;
  let creds = Credentials{
    username: config.username.clone(), 
    password: config.password.clone(),
  };

  let headers = Arc::new(Mutex::new(Headers{headers: None})); 
  // used to send ogg opus blocks between Udp thread to WebSocket thread
  let (tx, _) = broadcast::channel::<Bytes>(128);

  let ip = Ipv4Addr::from_str(&config.ip).unwrap();
  let server_addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED), config.mount_port);
  let listen_addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED), config.listen_port);

  // TODO: swap to value from config
  let end_point = format!("/{END_POINT}");
  let mount: Arc<str> = Arc::from(end_point);
  let mount_clone = mount.clone();

  let tx_clone = tx.clone();
  let headers_clone = headers.clone();
  // receive audio
  task::spawn(async move {
    ws::thread(tx_clone, listen_addr, creds, headers_clone).await;
  });

  // serve audio stream
  task::spawn(async move {
    http::thread(server_addr, tx, &headers, mount_clone).await
  });

  println!("{color_bright_yellow}Broadcasting on:{color_reset}\n\t{color_cyan}http://{}:{}{}{color_cyan}", ip, config.mount_port, mount);

  futures_util::future::pending::<()>().await;
  Ok(())
}
