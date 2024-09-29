use std::io::Cursor;
use std::sync::Arc;

use bedrockrs_core::int::LE;
use bedrockrs_macros::ProtoCodec;
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(i16)]
#[enum_endianness(le)]
pub enum SpawnBiomeType {
    Default = 0,
    UserDefined = 1,
}
