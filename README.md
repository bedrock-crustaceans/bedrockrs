# bedrock-rs
_The universal toolkit for MCBE_

A universal library for rust to interact with mcbe and provide standards, as well as common implementations for mcbe technologies written in pure rust.

## Modules:
- ### [Bedrock Core](https://github.com/Adrian8115/bedrock-rs/tree/main/bedrock_core):
  - Provides common data types and other shared code for other modules to use.
    
- ### [Nbt](https://github.com/Adrian8115/bedrock-rs/tree/main/nbt):
  - A simple nbt implementation focused on MCBE.
  - Provides Serialization for `LittleEndian` and `LittleEndianNetwork` (Commonly used in the mcbe protocol).
  
- ### [Protocol](https://github.com/Adrian8115/bedrock-rs/tree/main/protocol):
  - Basic implemtation of the MCBE protocol.
  - Support for both Server and Client side intended.
 
- ### [World](https://github.com/Adrian8115/bedrock-rs/tree/main/world):
  - Implementation of the MCBE level format using `rusty-leveldb` serialization and deserialization.

- ### [Packs](https://github.com/Adrian8115/bedrock-rs/tree/main/packs):
  - Common collection of metadata for behavior packs, resource packs and other MCBE related packs.

## Contributing:
Feel free to contribute at any moment. Contribute is always appreciated, a huge thanks goes to all contributors.
It is recommended to make yourself familiar with the `bedrock-rs` codebase.
You can always look at some issues and fix some bugs/add some features,
but also feel free to add entirely new features/modules to this library for enhancing the capabilities of `bedrock-rs`.
Just be creative and let us all profit from our work!

