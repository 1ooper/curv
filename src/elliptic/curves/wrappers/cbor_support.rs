use minicbor::encode::Write;
use minicbor::{decode, encode};
use minicbor::{Decode, Decoder, Encode, Encoder};

use crate::elliptic::curves::{Curve, Point, Scalar};

impl<E:Curve> Encode for Point<E> {
	fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
		e.bytes(&self.to_bytes(true))?.ok()
	}
}

impl<'b, E:Curve> Decode<'b> for Point<E> {
	fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
		d.bytes().map(|b| Point::from_bytes(&b).unwrap())
	}
}

impl<E:Curve> Encode for Scalar<E> {
	fn encode<W: Write>(&self, e: &mut Encoder<W>) -> Result<(), encode::Error<W::Error>> {
		e.bytes(&self.to_bytes())?.ok()
	}
}

impl<'b, E:Curve> Decode<'b> for Scalar<E> {
	fn decode(d: &mut Decoder<'b>) -> Result<Self, decode::Error> {
		d.bytes().map(|b| Scalar::from_bytes(&b).unwrap())
	}
}