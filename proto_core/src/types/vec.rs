use std::convert::TryInto;
use std::io::{Read, Write};

use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use bedrock_core::{Vec2, Vec2f, Vec3, Vec3f, LE, VAR};

use crate::error::ProtoCodecError;
use crate::ProtoCodec;

impl<T: ProtoCodec> ProtoCodec for Vec<T> {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match self.len().try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::FromIntError(e));
            }
        };

        match VAR::<u32>::new(len).write(buf) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        for item in self {
            match item.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match VAR::<u32>::read(stream) {
            Ok(v) => v.into_inner(),
            Err(e) => {
                return Err(ProtoCodecError::IOError(e));
            }
        };

        let mut array = vec![];

        for _ in 0..len {
            array.push(match T::proto_deserialize(stream) {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            })
        }

        Ok(array)
    }
}

impl ProtoCodec for Vec2 {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match LE::<i32>::proto_serialize(&LE::new(self.x), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match LE::<i32>::proto_serialize(&LE::new(self.z), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match LE::<i32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
            z: match LE::<i32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec2f {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match LE::<f32>::proto_serialize(&LE::new(self.x), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match LE::<f32>::proto_serialize(&LE::new(self.z), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match LE::<f32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
            z: match LE::<f32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec3 {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match LE::<i32>::proto_serialize(&LE::new(self.x), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match LE::<i32>::proto_serialize(&LE::new(self.y), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match LE::<i32>::proto_serialize(&LE::new(self.z), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match LE::<i32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
            y: match LE::<i32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
            z: match LE::<i32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl ProtoCodec for Vec3f {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match LE::<f32>::proto_serialize(&LE::new(self.x), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match LE::<f32>::proto_serialize(&LE::new(self.y), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match LE::<f32>::proto_serialize(&LE::new(self.z), stream) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(stream: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        Ok(Self {
            x: match LE::<f32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
            y: match LE::<f32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
            z: match LE::<f32>::proto_deserialize(stream) {
                Ok(v) => v.into_inner(),
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}
