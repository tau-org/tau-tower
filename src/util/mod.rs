use hyper::body::Bytes;

pub struct Credentials {
  pub username: String,
  pub password: String,
}

impl Credentials {
  pub fn validate(&self, username: Option<&str>, password: Option<&str>) -> bool {
    if (username, password) == (
      Some(&self.username),
      Some(&self.password),
      ) {
      return true
    }
    false
  }
}

#[derive(Debug)]
pub struct Headers {pub headers: Option<Bytes>}

impl Headers {
  pub fn prepare_headers(&mut self, buf: &(&Bytes, &Bytes)) {
    self.headers = Some(Bytes::copy_from_slice(&[&buf.0[..], &buf.1[..]].concat()))
  }
}

fn get_header_segment(data: &Bytes) -> Result<usize, ()> {
  let n_segs = data[26] as usize;
  let offset = 27+n_segs;
  if data.len() < 27 + 8 { return Err(()) }
  Ok(offset)
}

pub fn validate_tags(data: Bytes) -> Result<Option<Bytes>, ()> {
  let offset = get_header_segment(&data)?;
  if &data[offset..offset+8] == b"OpusTags" {
    println!("header tags found");
    return Ok(Some(data));
  }
  Err(())
}

pub fn validate_header(data: Bytes) -> Result<Option<Bytes>, ()> {
  let offset = get_header_segment(&data)?;
  if &data[offset..offset+8] == b"OpusHead" {
    println!("header found");
    return Ok(Some(data));
  }
  Err(())
}
