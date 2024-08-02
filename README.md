# bedrock-rs

<a href="https://discord.gg/ArvWEVHGWs"><img src="https://img.shields.io/discord/1218673790775726182.svg?style=flat&label=Azurite&logo=discord&logoColor=ffffff&color=011e2c&labelColor=1f3157"><a/>

_Universal toolkit for MCBE in Rust_

An easy-to-use universal library for Minecraft Bedrock written in Rust, that aims to provide:

- [X] Standards
- [X] Common implementations
- [X] An easy to use api

## Crates:

- [Core](https://github.com/Adrian8115/bedrock-rs/tree/main/bedrock_core):
    - Provides common base datatypes.

- [Shared]()

- [Nbt](https://github.com/Adrian8115/bedrock-rs/tree/main/nbt):
    - A simple nbt implementation focused on Minecraft Bedrock.
    - Provides Serialization for:
        - `NbtLittleEndian`
        - `NbtLittleEndianNetwork` (Commonly used in the mcbe protocol)
        - `NbtBigEndian`

- [Proto](https://github.com/Adrian8115/bedrock-rs/tree/main/proto):
    - Basic implementation of the Bedrock protocol.
    - Support for both Server and Client side intended.

- [Form](https://github.com/Adrian8115/bedrock-rs/tree/main/form):
    - An implementation of the json forms format used in Minecraft Bedrock.

- [World](https://github.com/Adrian8115/bedrock-rs/tree/main/world):
    - Implementation of the Bedrock level format using our own `leveldb` bindings for mojangs leveldb fork.

- [Addons](https://github.com/Adrian8115/bedrock-rs/tree/main/packs):
    - Datatypes defining the structure of Addons, including serialization and deserialization logic.

## Contributing:

Feel free to join in at any time. Your contributions are highly valued, and a big thank you to all who participate. We
recommend getting acquainted with the bedrock-rs codebase. Whether it's tackling existing issues, adding new features,
or even introducing entirely fresh modules, your creativity is welcome.

(If you like this library, don't forget to star it!)
