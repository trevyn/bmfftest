use four_cc::FourCC;
use nom::bytes::streaming::take;
use nom::number::streaming::{be_u32, be_u64};
use nom::IResult;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_longlong, c_uchar};

#[allow(dead_code, non_snake_case)]
extern "C" {
 fn GoListJSON(path: *const c_char);
 fn GoFetchFiledata(path: *const c_char, startbytepos: c_longlong, endbytepos: c_longlong);
}

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

/// Receive an array of File entries from Go and insert into turbosql
/// # Safety
/// `json` must be a valid pointer to valid C string until this function returns.
#[no_mangle]
extern "C" fn rust_insert_files_from_go(json: *const c_char) {
 let c_str = unsafe { CStr::from_ptr(json) };
 let string = c_str.to_str().unwrap().to_owned();
 dbg!(string);

 // let mut sender = RESPONSE_TX_CHANNEL.lock().unwrap().clone().unwrap();

 // tokio::spawn(async move {
 //  sender.send(string).await.unwrap();
 // });
}

/// Receive a Filecache entry from Go and insert into turbosql
/// buf is only valid until function return, must be copied
#[no_mangle]
extern "C" fn rust_insert_filecache_from_go(
 json: *const c_char,
 buf: *const c_uchar,
 len: c_longlong,
) {
 let c_str = unsafe { CStr::from_ptr(json) };
 let str = c_str.to_str().unwrap();

 dbg!(str);

 // log::info!("rust_insert_filecache_from_go: {:#?}", str);

 // let mut fc: FileCache = serde_json::from_str(str).unwrap();

 // log::info!("rust_insert_filecache_from_go fc: {:#?}", fc);

 // let slice = unsafe { std::slice::from_raw_parts(buf, len as usize) };
 // fc.bytes = Some(slice.to_vec());

 // fc.insert().unwrap();
}

pub fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
 let cstring = CString::new("".to_owned()).unwrap();

 unsafe {
  GoListJSON(cstring.as_ptr());
 }

 // let data = include_bytes!("/Users/e/Desktop/ep2.mp4");
 // let mut offset: usize = 0;
 // loop {
 //  let (_, b) = Box::parse(&data[offset..])?;
 //  dbg!(&b);
 //  offset += b.size as usize;
 // }

 Ok(())
}
