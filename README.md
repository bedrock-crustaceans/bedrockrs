# bedrock-rs

<a href="https://discord.gg/ArvWEVHGWs"><img src="https://img.shields.io/discord/1218673790775726182.svg?style=flat&label=Azurite&logo=discord&logoColor=ffffff&color=011e2c&labelColor=1f3157"><a/>

_The universal toolkit for MCBE in Rust_

A universal library for the Minecraft Bedrock Edition written in Rust, that aims to provide:

- standards
- common implementations
- an easy to use api

## Modules:

- [Bedrock Core](https://github.com/Adrian8115/bedrock-rs/tree/main/bedrock_core):
    - Provides common data types and other shared code for other modules to use.

- [Nbt](https://github.com/Adrian8115/bedrock-rs/tree/main/nbt):
    - A simple nbt implementation focused on MCBE.
    - Provides Serialization for `NbtLittleEndian`, `NbtLittleEndianNetwork` (Commonly used in the mcbe protocol)
      and `NbtBigEndian`.

- [Proto](https://github.com/Adrian8115/bedrock-rs/tree/main/proto):
    - Basic implementation of the Bedrock protocol.
    - Support for both Server and Client side intended.

- [Form](https://github.com/Adrian8115/bedrock-rs/tree/main/form):
    - An implementation of MCBEs json forms.

- [World](https://github.com/Adrian8115/bedrock-rs/tree/main/world):
    - Implementation of the Bedrock level format using our own `leveldb` bindings for mojangs leveldb fork.

- [Packs](https://github.com/Adrian8115/bedrock-rs/tree/main/packs):
    - Common collection of metadata for behavior packs, resource packs and other MCBE related packs.

## Contributing:

Feel free to join in at any time. Your contributions are highly valued, and a big thank you to all who participate. We
recommend getting acquainted with the bedrock-rs codebase. Whether it's tackling existing issues, adding new features,
or even introducing entirely fresh modules, your creativity is welcome. Let's all benefit from our collective efforts!
