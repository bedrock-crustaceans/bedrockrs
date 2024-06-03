use std::io::Cursor;
use bedrock_core::read::ByteStreamRead;

use bedrock_core::u32le;
use bedrock_core::write::ByteStreamWrite;
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;

use crate::types::experiment::Experiment;

#[derive(Debug)]
pub struct Experiments {
    pub experiments: Vec<Experiment>,
    pub ever_toggled: bool,
}

impl ProtoCodec for Experiments {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        match u32le(self.experiments.len() as u32).proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        };

        for experiment in &self.experiments {
            match experiment.proto_serialize(buf) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        match self.ever_toggled.proto_serialize(buf) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }

    fn proto_deserialize(cursor: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let len = match u32le::proto_deserialize(cursor) {
            Ok(v) => v.0,
            Err(e) => {
                return Err(e);
            }
        };

        let mut experiments = vec![];

        for _ in 0..len {
            match Experiment::proto_deserialize(cursor) {
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
