mod handler;

use bedrockrs_proto::connection::Connection;
use shipyard::World;
use bedrockrs_proto::connection::shard::arc::{ConnectionShared, shard};
use bedrockrs_proto::version::v729::helper::ProtoHelperV729;
use crate::error::LoginError;
use crate::login::handler::LoginHandler;

pub async fn login(connection: Connection, world: &mut World, login_handler: impl LoginHandler) -> Result<(), LoginError> {
    let mut shard = shard::<ProtoHelperV729>(connection);

    todo!()
}