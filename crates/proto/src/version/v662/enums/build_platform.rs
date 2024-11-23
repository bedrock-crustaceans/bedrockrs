use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec)]
pub enum BuildPlatform {
    Google = 1,
    iOS = 2,
    OSX = 3,
    Amazon = 4,
    GearVR = 5,
    UWP = 7,
    Win32 = 8,
    Dedicated = 9,
    #[deprecated] TvOs = 10,
    Sony = 11, /// PlayStation
    Nx = 12, /// Nintendo Switch
    Xbox = 13,
    #[deprecated] WindowsPhone = 14,
    Linux = 15,
    Unknown = -1,
}