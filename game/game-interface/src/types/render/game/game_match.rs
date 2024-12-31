use hiarc::Hiarc;
use serde::{Deserialize, Serialize};

use crate::types::id_types::CharacterId;

#[derive(Debug, Hiarc, Serialize, Deserialize, Clone, Copy)]
pub struct LeadingCharacter {
    /// id of the character
    pub character_id: CharacterId,
    /// The score of the player
    pub score: i64,
}

#[derive(Debug, Hiarc, Serialize, Deserialize, Clone, Copy)]
pub struct FlagCarrierCharacter {
    /// id of the character
    pub character_id: CharacterId,
    /// The score of the player
    pub score: i64,
}

/// Current results for the match.
#[derive(Debug, Hiarc, Serialize, Deserialize, Clone, Copy)]
pub enum MatchStandings {
    Solo {
        /// The top characters in the current match
        /// Usually spectators do not count.
        leading_characters: [Option<LeadingCharacter>; 2],
    },
    Sided {
        /// Score for the red side
        score_red: i64,
        /// Score for the blue side
        score_blue: i64,
        /// A player from red side that currently carries a flag.
        flag_carrier_red: Option<FlagCarrierCharacter>,
        /// A player from blue side that currently carries a flag.
        flag_carrier_blue: Option<FlagCarrierCharacter>,
    },
}

/// The side in the current match
#[derive(Debug, Hiarc, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatchSide {
    Red,
    Blue,
}
