use std::time::Duration;

use game_interface::{
    events::{GameWorldActionKillWeapon, KillFlags},
    types::{character_info::NetworkSkinInfo, resource_key::ResourceKey},
};

#[derive(Debug, Clone)]
pub struct ActionPlayer {
    pub name: String,
    pub skin: ResourceKey,
    pub skin_info: NetworkSkinInfo,
    pub weapon: ResourceKey,
}

#[derive(Debug, Clone)]
pub struct ActionKill {
    pub killer: Option<ActionPlayer>,
    /// assists to the killer
    pub assists: Vec<ActionPlayer>,
    pub victims: Vec<ActionPlayer>,
    pub weapon: GameWorldActionKillWeapon,
    pub flags: KillFlags,
}

#[derive(Debug, Clone)]
pub enum Action {
    Custom(String),
    Kill(ActionKill),
    RaceFinish {
        player: ActionPlayer,
        finish_time: Duration,
    },
    RaceTeamFinish {
        players: Vec<ActionPlayer>,
        team_name: String,
        finish_time: Duration,
    },
}

#[derive(Debug, Clone)]
pub struct ActionInFeed {
    pub action: Action,
    pub add_time: Duration,
}
