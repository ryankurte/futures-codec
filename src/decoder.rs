use super::framed_write::FramedWrite2;
use super::fuse::Fuse;
use bytes::BytesMut;
use std::io::Error;

/// Decoding of frames via buffers, for use with `FramedRead`.
pub trait Decoder {
    /// The type of items returned by `decode`
    type Item;
    /// The type of decoding errors.
    type Error: From<Error>;

    /// Decode an item from the src `BytesMut` into an item
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>;
}

impl<T, U: Decoder> Decoder for Fuse<T, U> {
    type Item = U::Item;
    type Error = U::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.u.decode(src)
    }
}

impl<T: Decoder> Decoder for FramedWrite2<T> {
    type Item = T::Item;
    type Error = T::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.inner.decode(src)
    }
}
