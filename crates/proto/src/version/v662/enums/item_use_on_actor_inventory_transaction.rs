pub mod ItemUseOnActorInventoryTransaction {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec, Clone, Debug)]
    pub enum ActionType {
        Interact = 0,
        Attack = 1,
        ItemInteract = 2,
    }
}