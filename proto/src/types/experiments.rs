use bedrock_core::read::ByteStreamRead;
use bedrock_core::write::ByteStreamWrite;
use bedrock_core::LE;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

use crate::types::experiment::Experiment;

#[derive(Debug)]
pub struct Experiments {
    pub experiments: Vec<Experiment>,
    pub ever_toggled: bool,
}

impl ProtoCodec for Experiments {
    fn proto_serialize(&self, stream: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match self.experiments.len().try_into() {
            Ok(v) => v,
            Err(e) => {
                return Err(ProtoCodecError::FromIntError(e));
            }
        };

        // Write length of downloading packs as an u16le
        match LE::<u16>::new(len).proto_serialize(stream) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        for experiment in &self.experiments {
            match experiment.proto_serialize(stream) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        match self.ever_toggled.proto_serialize(stream) {
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
        let len = match LE::<u32>::proto_deserialize(stream) {
            Ok(v) => v.into_inner(),
            Err(e) => {
                return Err(e);
            }
        };

        let mut experiments = vec![];

        for _ in 0..len {
            match Experiment::proto_deserialize(stream) {
                Ok(v) => experiments.push(v),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(Self {
            experiments,
            ever_toggled: true,
        })
    }
}
