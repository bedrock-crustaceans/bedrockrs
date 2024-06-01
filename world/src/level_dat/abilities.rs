use bedrock_core::permissions_level::PermissionLevel;

/// The default permissions for players in the world.
#[derive(Debug)]
pub struct WorldAbilities {
    /// If the player can attack mobs. (NBT entry: `attackmobs`)
    attack_mobs: bool,
    /// If the player can attack other players. (NBT entry: `attackplayers`)
    attack_players: bool,
    /// If the player is able to interact with redstone components. (NBT entry: `doorsandswitches`)
    redstone_interact: bool,
    /// If the player can place blocks. (NBT entry: `build`)
    build: bool,
    /// If the player can destroy blocks. (NBT entry: `mine`)
    mine: bool,
    /// If the player can instantly destroy blocks. (NBT entry: `instabuild`)
    mine_instantly: bool,
    /// If the player is currently flying. (NBT entry: `flying`)
    flying: bool,
    /// The flying speed, always 0.05. (NBT entry: `flySpeed`)
    fly_speed: f32,
    /// The walking speed, always 0.1. (NBT entry: `walkSpeed`)
    walk_speed: f32,
    /// If the player is immune to all damage and harmful effects. (NBT entry: `invulnerable`)
    invulnerable: bool,
    /// If lightning struck the player. (NBT entry: `lightning`)
    lightning: bool,
    /// If the player can fly. (NBT entry: `mayfly`)
    mayfly: bool,
    /// If the player messages cannot be seen by other players. (NBT entry: `mute`)
    mute: bool,
    /// If the player can phase through blocks. (NBT entry: `noclip`)
    no_clip: bool,
    /// If the player is able to open containers. (NBT entry: `opencontainers`)
    open_containers: bool,
    /// If the player is a world builder. (NBT entry: `worlNBTuilder`)
    world_builder: bool,
    /// If the player is allowed to teleport. (NBT entry: `teleport`)
    teleport: bool,
    /// If the player has operator commands. (NBT entry: `op`)
    op: bool,

    /// What permissions a player defaults to, when joining a world. (NBT entry: `permissionsLevel`)
    permissions_level: PermissionLevel,
    /// What permissions a player has. (NBT entry: `playerPermissionsLevel`)
    player_permissions_level: PermissionLevel,
}