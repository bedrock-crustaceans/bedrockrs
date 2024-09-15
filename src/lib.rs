pub mod core {
    pub use ::bedrockrs_core::*;

    pub use ::bedrockrs_shared::*;
}

#[cfg(feature = "proto")]
pub mod proto {
    pub use ::bedrockrs_proto::*;
    pub use ::bedrockrs_proto_core::GamePacket;

    pub mod codec {
        pub use ::bedrockrs_proto_core::error::ProtoCodecError;
        pub use ::bedrockrs_proto_core::ProtoCodec;
    }
}

#[cfg(feature = "addon")]
pub mod addon {
    pub use ::bedrockrs_addon::*;
}

#[cfg(feature = "world")]
pub mod world {
    pub use ::bedrockrs_world::*;

    pub mod palette {
        pub use ::bedrockrs_paletted_storage::*;
    }
}

pub mod form {
    pub use ::bedrockrs_form::*;
}
