use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
pub enum CommandOriginType {
    Player = 0,
    CommandBlock = 1,
    MinecartCommandBlock = 2,
    #[endianness(var)]
    DevConsole(i64) = 3,
    #[endianness(var)]
    Test(i64) = 4,
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