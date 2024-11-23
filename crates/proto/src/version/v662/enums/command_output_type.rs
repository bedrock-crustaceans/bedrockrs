use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum CommandOutputType {
    None = 0,
    LastOutput = 1,
    Silent = 2,
    AllOutput = 3,
    DataSet = 4,
}