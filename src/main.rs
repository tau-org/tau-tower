mod server;
mod threads;
mod config;
mod args;
mod util;

use std::any;
use std::net::{Ipv4Addr, SocketAddr};
use anyhow::{Context, Ok};
use tokio::sync::{RwLock, broadcast};
use tokio::task;
use std::sync::Arc;
use std::str::FromStr;
use hyper::body::Bytes;
use clap::Parser;

use crate::threads::{http, ws};
use crate::util::credentials::Credentials;
use crate::util::ui::server_started_info;
use crate::config::Config;
use crate::args::Args;
use crate::util::ip::filter_mount_endpoint;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = Args::parse();
  let config = Config::load_or_create(args.reset_config)
    .map(|c| c.merge_cli_args(&args))?;


  let creds = Credentials{
    username: config.username.clone(), 
    password: config.password.clone(),
  };

  
  /* 
   * Set the endpoint where the broadcast is served from this server
   * Validate endpoint - allow for either `endpoint` or `/endpoint` format
   */
  let endpoint = filter_mount_endpoint(config.mount);
  let mount: Arc<String> = Arc::from(endpoint.unwrap());
  let mount_clone = mount.clone();

  /*
   * Headers container to store OggOpus headers from source broadcast, for rebroadcasting 
   * when a listener connects to this servers stream.
   */
  let headers = Arc::new(RwLock::new(None)); 
  let headers_clone = headers.clone();

  /* 
   * Single producer - multiple identical streams. Used to capture the audio signal from the 
   * broadcasting source to each listener of this server. 
   * */ 
  let (tx, _) = broadcast::channel::<Bytes>(1024);
  let tx_clone = tx.clone();

  // remote source address: 
  let ip = Ipv4Addr::from_str(&config.ip).context("Invalid IP in config")?;

  // local listening and broadcasting addresses:
  let local_ip = std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED);
  let listen_addr = SocketAddr::new(local_ip, config.listen_port);
  let server_addr = SocketAddr::new(local_ip, config.mount_port);

  /*
   * Receiving task, listens to remote stream over WebSocket
   */
  let listener_task = task::spawn( 
    ws::thread(
      tx_clone,
      listen_addr,
      creds,
      headers_clone
    )
  );

  /*
   * Broadcasting task, broadcasts to all listeners over an http media stream
   */
  let server_task = task::spawn(
    http::thread(
      server_addr,
      tx,
      headers,
      mount_clone
    )
  );

  server_started_info(ip, config.mount_port, &mount);

  /*
   * Server will shut down if ctrl_c or error in either task is throwed. 
   * The tasks will loop indefinitely if they are able to bind to their respective TCP port.
   */
  tokio::select! {
    res = listener_task => { res?? },
    res = server_task => {res??},
    _ = tokio::signal::ctrl_c() => {
        println!("Shutdown signal received");
    }
  }
  Ok(())
}
