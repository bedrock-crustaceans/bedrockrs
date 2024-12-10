use bedrockrs_proto_core::error::ProtoCodecError;
use bedrockrs_proto_core::ProtoCodec;
use std::io::Cursor;
use std::mem::size_of;
use std::mem::transmute;
use varint_rs::{VarintReader, VarintWriter};

#[derive(Debug, Clone)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ProtoCodec for BlockPos {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), ProtoCodecError> {
        buf.write_i32_varint(self.x)?;
        // the y i32 height is serialized as an u32
        unsafe { buf.write_u32_varint(transmute::<i32, u32>(self.y))? };
        buf.write_i32_varint(self.z)?;

        Ok(())
    }

    fn proto_deserialize(buf: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError> {
        Ok(Self {
            x: buf.read_i32_varint()?,
            // the y i32 height is deserialized as an u32
            y: unsafe { transmute::<u32, i32>(buf.read_u32_varint()?) },
            z: buf.read_i32_varint()?,
        })
    }

    fn get_size_prediction(&self) -> usize {
        size_of::<i32>() + size_of::<u32>() + size_of::<i32>()
    }
}
