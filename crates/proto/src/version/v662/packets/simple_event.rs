use bedrockrs_macros::{gamepacket, ProtoCodec};

#[derive(ProtoCodec)]
#[enum_repr(u16)]
#[enum_endianness(le)]
#[repr(u16)]
enum Subtype {
    UninitializedSubtype = 0,
    EnableCommands = 1,
    DisableCommands = 2,
    UnlockWorldTemplateSettings = 3,
}

#[gamepacket(id = 64)]
#[derive(ProtoCodec)]
pub struct SimpleEventPacket {
    pub simple_event_type: Subtype,
}