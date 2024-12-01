use bedrockrs_macros::{gamepacket, ProtoCodec};
use crate::version::v729::types::actor_link::ActorLink;

#[gamepacket(id = 41)]
#[derive(ProtoCodec)]
pub struct SetActorLinkPacket {
    pub link: ActorLink,
}