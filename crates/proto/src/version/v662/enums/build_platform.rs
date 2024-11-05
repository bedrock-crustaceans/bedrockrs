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
    TvOsDeprecated = 10,
    Sony = 11,
    Nx = 12,
    Xbox = 13,
    WindowsPhoneDeprecated = 14,
    Linux = 15,
    Unknown = -1,
}