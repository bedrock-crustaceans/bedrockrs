use std::io::Cursor;

use bedrock_core::types::u32le;
use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

use crate::types::experiment::Experiment;

#[derive(Debug)]
pub struct Experiments {
    pub experiments: Vec<Experiment>,
    pub ever_toggled: bool,
}

impl MCProtoSerialize for Experiments {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        match u32le(self.experiments.len() as u32).proto_serialize(buf) {
            Ok(_) => {}
            Err(_) => {
                return Err(SerilizationError::WriteIOError);
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
}

impl MCProtoDeserialize for Experiments {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        todo!()
    }
}
