# bedrock-rs

_Universal toolkit for MCBE in Rust_

An easy-to-use universal library for Minecraft Bedrock written in Rust, that aims to provide:

- [X] Standards
- [X] Common implementations
- [X] An easy-to-use api

We also have a community discord, feel free to join it to learn more about our future and get help when needed: https://discord.com/invite/VCVcrvt3JC

## Crates:

- [Core](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/core):
  - Provides common base datatypes.

- [Shared](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/shared):
  - Shared datatypes that can use derive macros defined in other crates.

- [Proto](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/proto):
  - Full implementation of the Bedrock protocol.
  - Support for both Server and Client side intended.
  - Built-in login procedure.

- [World](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/world):
  - Implementation of the Bedrock level format using our own `leveldb` bindings for mojangs leveldb fork.

- [Addons](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/addon):
  - Datatypes defining the structure of Addons.
  - Serialization and Deserialization of addons.

- [Form](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/form):
  - Implementation of the JSON form format used in Minecraft Bedrock.

## Contributing:

Feel free to join in at any time. Your contributions are highly valued, and a big thank you to all who participate. We
recommend getting acquainted with the bedrock-rs codebase. Whether it's tackling existing issues, adding new features,
or even introducing entirely fresh modules, your creativity is welcome.

(If you like this library, remember to give bedrockrs a Star!)
