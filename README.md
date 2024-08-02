# bedrock-rs

<a href="https://discord.gg/ArvWEVHGWs"><img src="https://img.shields.io/discord/1218673790775726182.svg?style=flat&label=Azurite&logo=discord&logoColor=ffffff&color=011e2c&labelColor=1f3157"><a/>

_Universal toolkit for MCBE in Rust_

An easy-to-use universal library for Minecraft Bedrock written in Rust, that aims to provide:

- [X] Standards
- [X] Common implementations
- [X] An easy-to-use api

## Crates:

- [Core](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/bedrock_core):
    - Provides common base datatypes.

- [Shared](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/bedrock_core)


- [Proto](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/proto):
    - Full implementation of the Bedrock protocol.
    - Support for both Server and Client side intended.
    - Built-in login procedure.

- [World](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/world):
    - Implementation of the Bedrock level format using our own `leveldb` bindings for mojangs leveldb fork.

- [Addons](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/packs):
    - Datatypes defining the structure of Addons.
    - Serialization and Deserialization of addons.

- [Nbt](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/nbt):
    - A simple nbt implementation focused on Minecraft Bedrock.
    - Provides Serialization for:
        - `NbtLittleEndian`
        - `NbtLittleEndianNetwork` (Commonly used in the mcbe protocol)
        - `NbtBigEndian`

- [Form](https://github.com/Adrian8115/bedrock-rs/tree/main/crates/form):
    - Implementation of the JSON form format used in Minecraft Bedrock.

## Contributing:

Feel free to join in at any time. Your contributions are highly valued, and a big thank you to all who participate. We
recommend getting acquainted with the bedrock-rs codebase. Whether it's tackling existing issues, adding new features,
or even introducing entirely fresh modules, your creativity is welcome.

(If you like this library, remember to give bedrockrs a Star!)
