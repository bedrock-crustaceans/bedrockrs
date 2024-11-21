use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum PacketCompressionAlgorithm {
    ZLib = 0,
    Snappy = 1,
    None = 0xffff,
}