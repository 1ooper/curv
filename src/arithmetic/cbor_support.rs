use minicbor::encode::Write;
use minicbor::{decode, encode};
use minicbor::{Decode, Decoder, Encode, Encoder};

use super::traits::Converter;
use super::BigInt;

impl Encode for BigInt {
    fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
        e.bytes(&self.to_bytes())?.ok()
    }
}

impl<'b> Decode<'b> for BigInt {
    fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
        d.bytes().map(|b| BigInt::from_bytes(&b))
    }
}
