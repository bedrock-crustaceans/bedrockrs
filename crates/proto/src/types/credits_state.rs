use bedrockrs_proto_derive::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
pub enum CreditsState {
    Start = 0,
    Finished = 1,
}
