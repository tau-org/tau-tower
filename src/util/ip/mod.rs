use crate::config::TauConfigError;
use is_ip::is_ip;
use regex_lite::Regex;

pub fn validate_ip(ip: String) -> Result<String, TauConfigError> {
  if !is_ip(&ip) {
    return Err(TauConfigError::InvalidIp(ip));
  }
  Ok(ip)
}

pub(crate) fn parse_port(p: &str) -> Result<u16, TauConfigError> {
  p.parse::<u16>()
    .map_err(|e| TauConfigError::Input(format!("Unable to parse as number: {e}")))
}

pub(crate) fn validate_port(port: u16) -> Result<u16, TauConfigError> {
  if !(1..=0xFFFF).contains(&port) {
    return Err(TauConfigError::InvalidPort(port));
  }
  Ok(port)
}

pub(crate) fn validate_endpoint(endpoint: &str) -> Result<String, TauConfigError> {
  if Regex::new(r"^/?[a-zA-Z0-9._]+$")
    .expect("regex could not be built")
      .is_match(&endpoint) {
        return Ok(endpoint.to_string());
  } 
  Err(TauConfigError::InvalidEndpoint(endpoint.to_string()))
}

pub(crate) fn filter_mount_endpoint(endpoint: String) -> anyhow::Result<String> {
  if let Ok(endpoint) = validate_endpoint(&endpoint) {
    // if missing beginning '/', add it
    return Ok(if &endpoint[..1] != "/" {format!("/{}", endpoint)} else { endpoint });
  }

  anyhow::bail!("endpoint is badly formatted - check your config : {}", endpoint)
}
