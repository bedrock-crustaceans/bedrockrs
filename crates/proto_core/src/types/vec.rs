use std::io::Cursor;

use bedrockrs_core::{Vec2, Vec3};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl<T: ProtoCodec> ProtoCodec for Vec2<T> {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        T::proto_serialize(&self.x, stream)?;
        T::proto_serialize(&self.y, stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: T::proto_deserialize(stream)?,
            y: T::proto_deserialize(stream)?,
        })
    }
}

impl<T: ProtoCodec> ProtoCodec for Vec3<T> {
    fn proto_serialize(&self, stream: &mut Vec<u8>) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        T::proto_serialize(&self.x, stream)?;
        T::proto_serialize(&self.y, stream)?;
        T::proto_serialize(&self.z, stream)?;

        Ok(())
    }

    fn proto_deserialize(stream: &mut Cursor<&[u8]>) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: T::proto_deserialize(stream)?,
            y: T::proto_deserialize(stream)?,
            z: T::proto_deserialize(stream)?,
        })
    }
}
