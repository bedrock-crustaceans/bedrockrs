use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref COLOR_CODE_MINECRAFT_MAP: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("black", "§0"),
            ("dark-blue", "§1"),
            ("dark-green", "§2"),
            ("dark-aqua", "§3"),
            ("dark-red", "§4"),
            ("dark-purple", "§5"),
            ("gold", "§6"),
            ("gray", "§7"),
            ("dark-gray", "§8"),
            ("blue", "§9"),
            ("green", "§a"),
            ("aqua", "§b"),
            ("red", "§c"),
            ("light-purple", "§d"),
            ("yellow", "§e"),
            ("white", "§f"),
            ("minecoin-gold", "§g"),
            ("material-quartz", "§h"),
            ("material-iron", "§i"),
            ("material-netherite", "§j"),
            ("material-redstone", "§m"),
            ("material-copper", "§n"),
            ("material-gold", "§p"),
            ("material-emerald", "§q"),
            ("material-diamond", "§s"),
            ("material-lapis", "§t"),
            ("material-amethyst", "§u"),
            ("quartz", "§h"),
            ("iron", "§i"),
            ("netherite", "§j"),
            ("redstone", "§m"),
            ("copper", "§n"),
            ("emerald", "§q"),
            ("diamond", "§s"),
            ("lapis", "§t"),
            ("amethyst", "§u"),
            ("obfuscated", "§k"),
            ("bold", "§l"),
            ("italic", "§o"),
            ("reset", "§r"),
        ])
    };
}
