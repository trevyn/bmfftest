use four_cc::FourCC;
use nom::bytes::streaming::take;
use nom::number::streaming::{be_u32, be_u64};
use nom::IResult;
use std::convert::TryInto;

#[derive(Debug)]
struct Box {
 size: u64,
 ty: FourCC,
}

impl Box {
 fn parse(i: &[u8]) -> IResult<&[u8], Box> {
  let (i, size) = be_u32(i)?;
  let (i, ty) = take(4usize)(i)?;
  let ty: FourCC = ty.try_into().unwrap();
  let (i, size) = match size {
   1 => be_u64(i)?,
   _ => (i, size.into()),
  };

  Ok((i, Box { size, ty }))
 }
}

pub fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
 let data = include_bytes!("/Users/e/Desktop/ep2.mp4");
 let mut offset: usize = 0;
 loop {
  let (_, b) = Box::parse(&data[offset..])?;
  dbg!(&b);
  offset += b.size as usize;
 }
}
