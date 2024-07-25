pub mod core {
    pub use ::bedrockrs_core::*;
}

pub mod nbt {
    pub use ::bedrockrs_nbt::byte_order::*;
    pub use ::bedrockrs_nbt::endian::*;
    pub use ::bedrockrs_nbt::error::*;
    pub use ::bedrockrs_nbt::*;
}

#[cfg(feature = "proto")]
pub mod proto {
    pub use ::bedrockrs_proto::*;

    pub mod codec {
        pub use ::bedrockrs_proto_core::error::ProtoCodecError;
        pub use ::bedrockrs_proto_core::ProtoCodec;

        //pub use ::bedrockrs_proto_derive::ProtoCodec;
    }
}

#[cfg(feature = "addons")]
pub mod addons {
    pub use ::bedrockrs_addons::*;
}

#[cfg(feature = "world")]
pub mod world {
    pub use ::bedrockrs_world::*;
}

pub mod form {
    pub use ::bedrockrs_form::*;
}
