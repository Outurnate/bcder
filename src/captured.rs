//! Captured BER-encoded data.

use std::{io, ops};
use bytes::Bytes;
use super::{decode, encode};
use super::mode::Mode;


//------------ Captured ------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Captured {
    bytes: Bytes,
    mode: Mode,
}

impl Captured {
    pub(crate) fn new(bytes: Bytes, mode: Mode) -> Self {
        Captured { bytes, mode }
    }

    pub fn empty() -> Self {
        Captured {
            bytes: Bytes::new(),
            mode: Mode::Ber
        }
    }

    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    pub fn decode<F, T>(self, op: F) -> Result<T, decode::Error>
    where
        F: FnOnce(
            &mut decode::Constructed<Bytes>
        ) -> Result<T, decode::Error>
    {
        self.mode.decode(self.bytes, op)
    }

    pub fn decode_partial<F, T>(&mut self, op: F) -> Result<T, decode::Error>
    where
        F: FnOnce(
            &mut decode::Constructed<&mut Bytes>
        ) -> Result<T, decode::Error>
    {
        self.mode.decode(&mut self.bytes, op)
    }

    pub fn extend<V: encode::Values>(&mut self, values: V) {
        values.write_encoded(
            self.mode,
            &mut CapturedWriter(&mut self.bytes)
        ).unwrap()
    }
}


impl ops::Deref for Captured {
    type Target = Bytes;

    fn deref(&self) -> &Bytes {
        &self.bytes
    }
}

impl AsRef<Bytes> for Captured {
    fn as_ref(&self) -> &Bytes {
        &self.bytes
    }
}

impl AsRef<[u8]> for Captured {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl encode::Values for Captured {
    fn encoded_len(&self, mode: Mode) -> usize {
        if self.mode != mode && mode != Mode::Ber {
            panic!("Trying to encode a captured value with incompatible mode");
        }
        self.bytes.len()
    }

    fn write_encoded<W: io::Write>(
        &self,
        mode: Mode,
        target: &mut W
    ) -> Result<(), io::Error> {
        if self.mode != mode && mode != Mode::Ber {
            panic!("Trying to encode a captured value with incompatible mode");
        }
        target.write_all(self.bytes.as_ref())
    }
}


//------------ CapturedWriter ------------------------------------------------

struct CapturedWriter<'a>(&'a mut Bytes);

impl<'a> io::Write for CapturedWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
