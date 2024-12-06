use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v662::types::{ActorUniqueID, BlockPos, MapDecoration, MapItemTrackedActor};

#[derive(ProtoCodec)]
struct PixelEntry {
    #[endianness(var)]
    pub pixel: u32,
}

#[derive(ProtoCodec)]
#[enum_repr(u32)]
#[enum_endianness(var)]
#[repr(u32)]
enum Type {
    Invalid = 0,
    TextureUpdate {
        #[endianness(var)]
        texture_width: i32,
        #[endianness(var)]
        texture_height: i32,
        #[endianness(var)]
        x_tex_coordinate: i32,
        #[endianness(var)]
        y_tex_coordinate: i32,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        pixels: Vec<PixelEntry>,
    } = 1 << 1,
    DecorationUpdate {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        actor_ids: Vec<MapItemTrackedActor::UniqueId>,
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        decoration_list: Vec<MapDecoration>
    } = 1 << 2,
    Creation {
        #[vec_repr(u32)]
        #[vec_endianness(var)]
        map_id_list: Vec<ActorUniqueID>
    } = 1 << 3,
}

#[gamepacket(id = 67)]
#[derive(ProtoCodec)]
pub struct ClientboundMapItemDataPacket {
    pub map_id: ActorUniqueID,
    pub type_flags: Type,
    pub dimension: i8,
    pub is_locked: bool,
    pub map_origin: BlockPos,
}

// TODO: custom proto impl, enum variants