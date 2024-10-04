use bedrockrs_macros::ProtoCodec;
use uuid::Uuid;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_repr(u32)]
#[enum_endianness(var)]
pub enum CommandOriginType {
    Player = 0,
    CommandBlock = 1,
    MinecartCommandBlock = 2,
    DevConsole = 3,
    Test = 4,
    AutomationPlayer = 5,
    ClientAutomation = 6,
    DedicatedServer = 7,
    Entity = 8,
    Virtual = 9,
    GameArgument = 10,
    EntityServer = 11,
    Precompiled = 12,
    GameDirectorEntityServer = 13,
    Scripting = 14,
    ExecuteContext = 15,
}

#[derive(ProtoCodec, Debug, Clone)]
pub struct CommandOriginData {
    pub command_type: CommandOriginType,
    pub command_uuid: Uuid,
    pub request_id: String,
}
