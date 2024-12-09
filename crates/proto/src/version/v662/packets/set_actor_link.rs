use crate::version::v729::types::actor_link::ActorLink;
use bedrockrs_macros::{gamepacket, ProtoCodec};

#[gamepacket(id = 41)]
#[derive(ProtoCodec)]
pub struct SetActorLinkPacket {
    pub link: ActorLink,
}