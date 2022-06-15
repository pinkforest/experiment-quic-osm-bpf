// Automatically generated rust module for 'osm.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlobHeader<'a> {
    pub type_pb: Cow<'a, str>,
    pub indexdata: Option<Cow<'a, [u8]>>,
    pub datasize: i32,
}

impl<'a> MessageRead<'a> for BlobHeader<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.type_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.indexdata = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.datasize = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlobHeader<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.type_pb).len())
        + self.indexdata.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_varint(*(&self.datasize) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.type_pb))?;
        if let Some(ref s) = self.indexdata { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        w.write_with_tag(24, |w| w.write_int32(*&self.datasize))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Blob<'a> {
    pub raw_size: Option<i32>,
    pub data: mod_Blob::OneOfdata<'a>,
}

impl<'a> MessageRead<'a> for Blob<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.raw_size = Some(r.read_int32(bytes)?),
                Ok(10) => msg.data = mod_Blob::OneOfdata::raw(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.data = mod_Blob::OneOfdata::zlib_data(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.data = mod_Blob::OneOfdata::lzma_data(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.data = mod_Blob::OneOfdata::bzip2_data(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.data = mod_Blob::OneOfdata::lz4_data(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.data = mod_Blob::OneOfdata::zstd_data(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Blob<'a> {
    fn get_size(&self) -> usize {
        0
        + self.raw_size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + match self.data {
            mod_Blob::OneOfdata::raw(ref m) => 1 + sizeof_len((m).len()),
            mod_Blob::OneOfdata::zlib_data(ref m) => 1 + sizeof_len((m).len()),
            mod_Blob::OneOfdata::lzma_data(ref m) => 1 + sizeof_len((m).len()),
            mod_Blob::OneOfdata::bzip2_data(ref m) => 1 + sizeof_len((m).len()),
            mod_Blob::OneOfdata::lz4_data(ref m) => 1 + sizeof_len((m).len()),
            mod_Blob::OneOfdata::zstd_data(ref m) => 1 + sizeof_len((m).len()),
            mod_Blob::OneOfdata::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.raw_size { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        match self.data {            mod_Blob::OneOfdata::raw(ref m) => { w.write_with_tag(10, |w| w.write_bytes(&**m))? },
            mod_Blob::OneOfdata::zlib_data(ref m) => { w.write_with_tag(26, |w| w.write_bytes(&**m))? },
            mod_Blob::OneOfdata::lzma_data(ref m) => { w.write_with_tag(34, |w| w.write_bytes(&**m))? },
            mod_Blob::OneOfdata::bzip2_data(ref m) => { w.write_with_tag(42, |w| w.write_bytes(&**m))? },
            mod_Blob::OneOfdata::lz4_data(ref m) => { w.write_with_tag(50, |w| w.write_bytes(&**m))? },
            mod_Blob::OneOfdata::zstd_data(ref m) => { w.write_with_tag(58, |w| w.write_bytes(&**m))? },
            mod_Blob::OneOfdata::None => {},
    }        Ok(())
    }
}

pub mod mod_Blob {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfdata<'a> {
    raw(Cow<'a, [u8]>),
    zlib_data(Cow<'a, [u8]>),
    lzma_data(Cow<'a, [u8]>),
    bzip2_data(Cow<'a, [u8]>),
    lz4_data(Cow<'a, [u8]>),
    zstd_data(Cow<'a, [u8]>),
    None,
}

impl<'a> Default for OneOfdata<'a> {
    fn default() -> Self {
        OneOfdata::None
    }
}

}

