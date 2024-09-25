use std::io::Cursor;
use std::mem::transmute;
use varint_rs::{VarintReader, VarintWriter};
use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;

#[derive(Debug, Clone)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ProtoCodec for BlockPos {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        buf.write_i32_varint(self.x)?;
        unsafe { buf.write_u32_varint(transmute(self.x))? };
        buf.write_i32_varint(self.x)?;

        Ok(())
    }

    fn proto_deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self {
            x: buf.read_i32_varint()?,
            y: unsafe { transmute(buf.read_u32_varint()?) },
            z: buf.read_i32_varint()?,
        })
    }
}
