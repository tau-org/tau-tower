use hyper::body::Bytes;

#[derive(Debug, Clone)]
pub struct Headers {
  pub head: Bytes,
  pub tags: Bytes
}

impl Headers {
  pub fn new(headers: (Bytes, Bytes)) -> Self {
    Self{
      head: headers.0,
      tags: headers.1,
    }
  }
}

fn get_header_segment(data: &Bytes) -> Result<usize, ()> {
  let n_segs = data[26] as usize;
  let offset = 27+n_segs;
  if data.len() < 27 + 8 { return Err(()) }
  Ok(offset)
}

pub fn validate_tags(data: Bytes) -> Result<Bytes, ()> {
  let offset = get_header_segment(&data)?;
  if &data[offset..offset+8] == b"OpusTags" {
    println!("header tags found");
    return Ok(data);
  }
  Err(())
}

pub fn validate_header(data: Bytes) -> Result<Bytes, ()> {
  let offset = get_header_segment(&data)?;
  if &data[offset..offset+8] == b"OpusHead" {
    println!("header found");
    return Ok(data);
  }
  Err(())
}


