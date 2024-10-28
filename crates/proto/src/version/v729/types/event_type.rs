use bedrockrs_macros::ProtoCodec;
use bedrockrs_shared::actor_unique_id::ActorUniqueID;

#[derive(ProtoCodec, Debug, Clone)]
#[enum_endianness(var)]
#[enum_repr(u32)]
#[repr(u32)]
pub enum BossEventType {
    Add {
        name: String,
        #[endianness(le)]
        health_percentage: f32,
        #[endianness(le)]
        darken_screen: u16,
        #[endianness(var)]
        color: u32,
        #[endianness(var)]
        overlay: u32,
    } = 0,
    PlayerAdded {
        actor_id: ActorUniqueID,
    } = 1,
    Remove = 2,
    PlayerRemoved {
        actor_id: ActorUniqueID,
    } = 3,
    UpdatePercent {
        #[endianness(le)]
        health_percentage: f32,
    } = 4,
    UpdateName {
        name: String,
    } = 5,
    UpdateProperties {
        #[endianness(le)]
        darken_screen: u16,
        #[endianness(var)]
        color: u32,
        #[endianness(var)]
        overlay: u32,
    } = 6,
    UpdateStyle {
        #[endianness(var)]
        color: u32,
        #[endianness(var)]
        overlay: u32,
    } = 7,
    Query {
        actor_id: ActorUniqueID,
    } = 8,
}
