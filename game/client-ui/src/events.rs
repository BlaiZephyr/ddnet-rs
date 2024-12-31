use std::{net::SocketAddr, path::PathBuf};

use base::network_string::NetworkReducedAsciiString;
use game_interface::votes::{
    MapCategoryVoteKey, MiscVoteCategoryKey, PlayerVoteKey, RandomUnfinishedMapKey,
};
use hiarc::{hiarc_safer_rc_refcell, Hiarc};
use math::math::vector::ubvec4;

#[derive(Debug, Hiarc)]
pub enum UiEvent {
    StartLocalServer,
    CheckLocalServer,
    PlayDemo {
        name: PathBuf,
    },
    EncodeDemoToVideo {
        name: PathBuf,
        video_name: String,
    },
    RecordDemo,
    StopRecordDemo,
    InstantReplay,
    StartEditor,
    Connect {
        addr: SocketAddr,
        cert_hash: [u8; 32],
        rcon_secret: Option<[u8; 32]>,
        can_start_local_server: bool,
    },
    Disconnect,
    ConnectLocalPlayer {
        as_dummy: bool,
    },
    DisconnectLocalPlayer,
    Quit,
    Kill,
    JoinSpectators,
    JoinGame,
    JoinOwnTeam {
        name: String,
        color: ubvec4,
    },
    JoinOtherTeam(String),
    JoinDefaultTeam,
    JoinVanillaSide {
        is_red_side: bool,
    },
    SwitchToFreeCam,
    /// Window settings changed
    WindowChange,
    VsyncChanged,
    MsaaChanged,
    VoteKickPlayer(PlayerVoteKey),
    VoteSpecPlayer(PlayerVoteKey),
    VoteMap(MapCategoryVoteKey),
    VoteRandomUnfinishedMap(RandomUnfinishedMapKey),
    VoteMisc(MiscVoteCategoryKey),
    ChangeAccountName {
        name: NetworkReducedAsciiString<32>,
    },
    RequestAccountInfo,
}

#[hiarc_safer_rc_refcell]
#[derive(Debug, Hiarc, Default)]
pub struct UiEvents {
    events: Vec<UiEvent>,
}

#[hiarc_safer_rc_refcell]
impl UiEvents {
    pub fn new() -> Self {
        Self {
            events: Default::default(),
        }
    }

    pub fn push(&mut self, ev: UiEvent) {
        self.events.push(ev);
    }

    pub fn take(&mut self) -> Vec<UiEvent> {
        std::mem::take(&mut self.events)
    }
}
