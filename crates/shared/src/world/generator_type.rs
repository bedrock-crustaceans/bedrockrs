use bedrockrs_core::int::VAR;
use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(VAR::<i32>)]
pub enum GeneratorType {
    Legacy = 0x00,
    Overworld = 0x01,
    Flat = 0x02,
    Nether = 0x03,
    End = 0x04,
    Void = 0x05,
}
